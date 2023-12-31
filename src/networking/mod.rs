/* 
 * @Author: Sofia Langer-Osuna
 */
use bevy::{
    prelude::*,
};
use bevy_steamworks::*;
use std::sync::Mutex;
pub use serializable_impls::*;

pub mod serializable_impls;

#[derive(Clone)]
pub struct NetworkingPlugin {
    pub max_players: u16,
    pub max_synced_objects: u16,
    pub app_id: u32,
    pub packet_per_frame_limit: u8,
}

#[derive(Resource)]
pub struct NetworkingResource {
    pub max_players: u16,
    pub max_synced_objects: u16,

    pub connected: bool,
    pub client: Client,
    pub player_id: SteamId,
    pub active_players: Vec<SteamId>,
    pub sync_messages: Vec<Vec<u8>>,
    pub packet_per_frame_limit: u8,
    
    event_queue_out: Mutex<Vec<NetworkingEvent>>,
    event_queue_in: Vec<Mutex<Vec<NetworkingEvent>>>, // The index of the outer vector is the event type
}

impl NetworkingResource {
    pub fn queue_event_out(&self, event: NetworkingEvent) {
        self.event_queue_out.lock().unwrap().push(event);
    }
    pub fn get_event_in(&self, event_type: u8) -> Vec<NetworkingEvent> {
        self.event_queue_in[event_type as usize].lock().unwrap().drain(..).collect()
    }
}

#[derive(Clone)]
pub struct NetworkingEvent {
    pub event_type: u8,
    length: u16,
    pub data: Vec<u8>,
}

impl NetworkingEvent {
    fn new(event_type: u8, data: Vec<u8>) -> NetworkingEvent {
        NetworkingEvent {
            event_type,
            length: data.len() as u16,
            data,
        }
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(self.event_type);
        bytes.push(self.length.to_le_bytes()[0]);
        bytes.push(self.length.to_le_bytes()[1]);
        bytes.append(&mut self.data.clone().into());
        bytes
    }
    fn from_bytes(bytes: &[u8]) -> NetworkingEvent {
        let event_type = bytes[0];
        let length = u16::from_le_bytes([bytes[1], bytes[2]]);
        let data = bytes[3..(length as usize + 3)].to_vec();
        NetworkingEvent {
            event_type,
            length,
            data,
        }
    }
}

//part of message header
const ENTITY_CREATE: u8 = 0;
const ENTITY_DELETE: u8 = 1;
const ENTITY_UPDATE: u8 = 2;
const PLAYER_JOIN: u8 = 3;
const PLAYER_LEAVE: u8 = 4;
const EVENT: u8 = 5;

#[derive(Component)]
pub struct SynchronizedSlave {
    object_info: u8, /*First bit marks whether or not to delete, 
                      *Second bit marks whether to sync periodically,
                      */
    static_id: u16,
    destroy_on_owner_disconnect: bool,
    owner: u16,
}

#[derive(Component)]
pub struct SynchronizedMaster {
    object_info: u8, /*First bit marks whether or not to delete, 
                      *Second bit marks whether to sync periodically,
                      */
    static_id: u16,
    marked_for_deletion: bool,
}
impl SynchronizedMaster {
    pub fn destroy(&mut self, networking: &NetworkingResource) {
         //sets the first bit which signifies whether to delete to 1 marking it for deletion
         self.object_info |= 0b10000000;

         let mut bytes: Vec<u8> = Vec::new();

         bytes.push(ENTITY_DELETE);
         bytes.push(self.static_id.to_le_bytes()[0]);
         bytes.push(self.static_id.to_le_bytes()[1]);

         networking.send_all_reliable(bytes);
    }
}

#[bevy_trait_query::queryable]
pub trait Serializable {
    fn from_bytes(&mut self, bytes: &[u8]);
    fn to_bytes(&self) -> Vec<u8>;
    
    fn get_length(&self) -> usize;
    //used to identify the type of the component when synchronizing
    fn get_type_id(&self) -> u16;
}

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        //leaks the plugin to prevent it from being dropped until the setup system
        //TODO make this less sketchy (low priority since it works)
        let clone = Box::leak(Box::new(self.clone()));
        app
          .add_plugin(SteamworksPlugin::new(AppId(self.app_id)))

          //Registering components as serializable
          //TODO add other components when I implement Serializable for them
          .register_serializable::<Transform>()
        
          //Uses a closure to pass settings to setup system
          .add_startup_system(|a: Commands, b: ResMut<Client>| setup(clone, a, b))
          .add_system(handle_networking)
          .add_system(delete_marked_slaves)
          .add_system(delete_marked_masters)
          .add_system(sync_slave_entities)
          .add_system(sync_master_entities);
    }
}

fn setup(
    plugin: &mut NetworkingPlugin,
    mut commands: Commands,
    steam_client: ResMut<Client>
) {
    let player_id = steam_client.user().steam_id();

    let mut active_players: Vec<SteamId> = Vec::new();
    active_players.push(player_id);

    let sync_messages: Vec<Vec<u8>> = Vec::new();
    let mut event_queue_in: Vec<Mutex<Vec<NetworkingEvent>>> = 
        Vec::with_capacity(u8::max_value() as usize);

    for _ in 0..u8::max_value() { event_queue_in.push(Mutex::new(Vec::new())); }

    let networking_resource = NetworkingResource {
        max_players: plugin.max_players,
        max_synced_objects: plugin.max_synced_objects,
        connected: false,
        client: steam_client
            .as_ref().clone(),
        player_id,
        active_players,
        sync_messages,
        packet_per_frame_limit:
            plugin.packet_per_frame_limit,
        
        event_queue_out: Mutex::new(Vec::new()),
        event_queue_in,
    };
    
    //drops plugin manually because it was leaked earlier
    unsafe { std::ptr::drop_in_place(plugin); }

    commands.insert_resource(networking_resource);
}

fn handle_networking(
    mut networking_res: ResMut<NetworkingResource>
) {
    if !networking_res.connected
        { return; }

    let networking = networking_res.client.networking();
    
    let mut guard = networking_res.event_queue_out.lock().unwrap();
    let events_to_send: Vec<NetworkingEvent> = guard.drain(..).collect();
    
    drop(guard); //unlocks the mutex

    events_to_send.into_iter().map(
        |event| { networking_res.send_all_reliable(event.to_bytes()); } 
    ).for_each(drop);

    let mut i: u8 = 0;
    loop {
        //limits the number of packets read per frame to packet_per_frame_limit
        if i >= networking_res.packet_per_frame_limit
            { break; }
        i += 1;

        let is_packet_available = networking.is_p2p_packet_available();
        //if no packet is available, return
        if !is_packet_available.is_some()
            { return; }

        //creates a buffer with the size of the packet
        let mut buffer: Vec<u8> =
            Vec::with_capacity(is_packet_available.unwrap());
        
        //reads the packet into the buffer
        let (_, sender) = networking.read_p2p_packet(buffer.as_mut_slice()).unwrap();

        //if the sender is not in the active players list, add them
        match buffer[0] {
            ENTITY_UPDATE => networking_res.sync_messages.push(buffer),
            ENTITY_DELETE => networking_res.sync_messages.push(buffer),
            ENTITY_CREATE => {
                //TODO create entity
            },
            PLAYER_JOIN => {
                //TODO add player
            },
            PLAYER_LEAVE => {
                //TODO remove player and all their entities that are destroy_on_owner_disconnect
                //TODO decide who inherits their entities
            },
            EVENT => {
                //doesn't include the first byte which is the msg type
                let event = NetworkingEvent::from_bytes(&buffer[1..]); 
                
                let mut queue_in = networking_res.event_queue_in[event.event_type as usize].lock().unwrap();
                queue_in.push(event);
            },
            _ => (),
        }
    }
}

const MAX_COMPONENTS_PER_ENTITY: usize = 10;

fn sync_slave_entities(
    mut networking: ResMut<NetworkingResource>,
    mut query: Query<(&mut dyn Serializable, &mut SynchronizedSlave)>,
) {
    if !networking.connected
        { return; }
    //clones the sync messages to prevent borrowing issues
    let sync_messages = networking.sync_messages.clone();

    for message in sync_messages.into_iter() {
        match message[0] {
            ENTITY_UPDATE => {
                let static_id = u16::from_le_bytes([message[1], message[2]]);
                for mut entity in query.iter_mut() {
                    if entity.1.static_id == static_id {
                        //skips the first 3 bytes which are the message type and static id
                        let mut i = 3;
                        
                        //used to keep track of the number of components updated
                        let mut n = 0;
                        while i < message.len() {
                            if n > MAX_COMPONENTS_PER_ENTITY
                                { break; }
                            let component_id = u16::from_le_bytes([message[i], message[i + 1]]);
                            i += 2;
                            
                            //finds the component with the matching id and updates it
                            for mut component in &mut entity.0 {
                                if component.get_type_id() == component_id {
                                    let len = component.get_length();
                                    component.from_bytes(&message[i..i+len]);
                                    i += len;
                                    break;
                                }
                            }
                            n += 1;
                        }
                        break;
                    }
                }
            },
            ENTITY_DELETE => {
                let static_id = u16::from_le_bytes([message[1], message[2]]);
                for mut entity in query.iter_mut() {
                    if entity.1.static_id == static_id {
                        entity.1.object_info |= 0b10000000;
                        break;
                    }
                }
            },
            _ => { panic!("Invalid sync message"); }, //Should be impossible to reach
        }
    }
    
    networking.sync_messages.clear();
}

fn sync_master_entities(
    networking: Res<NetworkingResource>,
    query: Query<(&dyn Serializable, &SynchronizedMaster)>
) {
    if !networking.connected
        { return; }

    for entity in query.iter() {
        if (entity.1.object_info & 0b01000000) != 0 { //checks whether or not to sync periodically
            let mut bytes: Vec<u8> = Vec::new();

            //Adds header data (message type and static id)
            bytes.push(ENTITY_UPDATE);
            bytes.extend_from_slice(&entity.1.static_id.to_le_bytes());
            
            for component in entity.0 {
                //Adds component type id and component data 
                //  (length is constant per type and
                //   therefore doesn't need to be sent)
                bytes.extend_from_slice(&component.get_type_id().to_le_bytes());
                bytes.extend_from_slice(&component.to_bytes());
            }

            //sends all unreliable because it's ok if some packets are dropped 
            //because they are sent every frame anyways
            networking.send_all_unreliable(bytes);
        }
    }
}

fn delete_marked_slaves(
    networking: Res<NetworkingResource>,
    mut commands: Commands,
    query: Query<(Entity, &SynchronizedSlave)>
) {
    if !networking.connected
        { return; }
    
    for entity in query.iter() {
        if (entity.1.object_info & 0b10000000) != 0 {
            commands.entity(entity.0).despawn_recursive();
        }
    }
}

fn delete_marked_masters(
    networking: Res<NetworkingResource>,
    mut commands: Commands,
    query: Query<(Entity, &SynchronizedMaster)>
) {
    if !networking.connected
        { return; }

    for entity in query.iter() {
        if (entity.1.object_info & 0b10000000) != 0 {
            commands.entity(entity.0).despawn_recursive();
        }
    }
}

impl NetworkingResource {
    fn send_all_unreliable(
        &self, bytes: Vec<u8>
    ) {
        let networking = self.client.networking();

        for player in self.active_players.iter() {
            networking.send_p2p_packet(*player, SendType::Unreliable, &bytes);
        }
    }

    fn send_all_reliable(
        &self, bytes: Vec<u8>
    ) {
            let networking = self.client.networking();

            for player in self.active_players.iter() {
                networking.send_p2p_packet(*player, SendType::Reliable, &bytes);
            }
    }

    pub fn create_networked_entity(
        &self,
        commands: &mut Commands,
        components: &[Box<impl Serializable>],
        entity: &Entity, 
        sync_periodically: bool,
        static_id: u16
    ) {
        let mut object_info: u8 = 0;
        if sync_periodically { object_info |= 0b01000000; }

        commands.entity(*entity)
            .insert(SynchronizedMaster {
                object_info,
                static_id,
                marked_for_deletion: false,
            });

        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(ENTITY_CREATE);
        bytes.extend_from_slice(&static_id.to_le_bytes());
        bytes.push(object_info);
        bytes.push(components.len().try_into().unwrap());


        for component in components {
            let type_id = component.get_type_id().to_le_bytes();
            let comp_bytes = component.to_bytes();

            bytes.extend_from_slice(&type_id);
            bytes.push(comp_bytes.len().try_into().unwrap());
            bytes.extend_from_slice(&comp_bytes);
        }

        self.send_all_reliable(bytes);
    }
}
