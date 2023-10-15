use bevy::{
    prelude::*,
    ecs::component::{
        ComponentInfo, 
        ComponentId,
    },
};
use bevy_steamworks::*;

pub mod serializable_impls;

pub struct NetworkingPlugin {
    pub max_players: u16,
    pub max_synced_objects: u16,
    pub app_id: u32,
}

#[derive(Resource)]
pub struct NetworkingResource {
    pub connected: bool,
    pub client: Client,
    pub player_id: SteamId,
    pub active_players: Vec<SteamId>,
    pub sync_messages: Vec<Vec<u8>>,
}

enum MessageType {
    EntityCreate,
    EntityDelete,
    EntityUpdate,
    PlayerJoin,
}

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
         self.object_info |= 0b10000000;

         let mut bytes: Vec<u8> = Vec::new();

         bytes.push(MessageType::EntityDelete as u8);
         bytes.push(self.static_id.to_le_bytes()[0]);
         bytes.push(self.static_id.to_le_bytes()[1]);

         networking.send_all_reliable(bytes);
    }
}

pub trait Serializable {
    fn from_bytes(&mut self, bytes: &[u8]);
    fn to_bytes(&self) -> Vec<u8>;

    fn get_type_id(&self) -> u16;
}

pub trait SerializableComponent: Component + Serializable { }

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
          .add_plugin(SteamworksPlugin::new(AppId(self.app_id)))
          .add_startup_system(setup)
          .add_system(handle_networking)
          .add_system(delete_marked_slaves)
          .add_system(delete_marked_masters)
          .add_system(sync_slave_entities)
          .add_system(sync_master_entities); 
    }
}

fn setup
    (mut commands: Commands,
     steam_client: ResMut<Client>) {
        let player_id = steam_client.user().steam_id();

        let mut active_players: Vec<SteamId> = Vec::new();
        active_players.push(player_id);

        let sync_messages: Vec<Vec<u8>> = Vec::new();

        let networking_resource = NetworkingResource {
            connected: false,
            client: steam_client
                .as_ref().clone(),
            player_id,
            active_players,
            sync_messages,
        };

        commands.insert_resource(networking_resource);
}

fn handle_networking
  (mut networking_res: ResMut<NetworkingResource>) {
        if !networking_res.connected
            { return; }

        let networking = networking_res.client.networking();

        let is_packet_available = networking.is_p2p_packet_available();
        if !is_packet_available.is_some()
            { return; }

        let mut buffer: Vec<u8> =
            Vec::with_capacity(is_packet_available.unwrap());

        let (_, sender) = networking.read_p2p_packet(buffer.as_mut_slice()).unwrap();

        if buffer[0] == MessageType::EntityUpdate as u8
        || buffer[0] == MessageType::EntityDelete as u8 {
            networking_res.sync_messages.push(buffer);
        } else if buffer[0] == MessageType::EntityCreate as u8 {
            //TODO create
        }
}

fn sync_slave_entities
    (networking: Res<NetworkingResource>,
     mut commands: Commands,
     mut query: Query<&mut SynchronizedSlave>,) {
        if !networking.connected
            { return; }
        //TODO
}

fn sync_master_entities
    (networking: Res<NetworkingResource>,
     app: &mut App,
     query: Query<(Entity, &SynchronizedMaster)>){
        if !networking.connected
            { return; }
        let world = app.world;

        for entity in query.iter() {
            if (entity.1.object_info & 0b01000000) != 0 {
                let mut bytes: Vec<u8> = Vec::new();
                bytes.push(MessageType::EntityUpdate as u8);
             
                bytes.push(entity.1.static_id.to_le_bytes()[0]);
                bytes.push(entity.1.static_id.to_le_bytes()[1]);
                
                if let Some(component_ids) = get_component_ids(&world, &entity.0) {
                    for component_id in component_ids {
                        |mut bytes: &Vec<u8>| -> Option<()>{
                        let component_info = component_id_to_component_info(&world, component_id);
                        let component_name = extract_component_name(component_info?);
                        
                        if let Ok(component) = world.get_component::<SerializableComponent>(entity.0, component_id){
                            bytes.extend_from_slice(&component.get_static_id().to_le_bytes());
                            bytes.extend_from_slice(&component.to_bytes());
                        } // TODO fix this
                        Some(())}(&mut bytes);
                    }
                }

                networking.send_all_unreliable(bytes);
            }
        }
}

/// gets an iterator component id from the world corresponding to your entity
fn get_component_ids<'a>(world: &'a World, entity: &Entity) -> Option<impl Iterator<Item=ComponentId> + 'a> {
    // components and entities are linked through archetypes
    for archetype in world.archetypes().iter() {
        if archetype.entities().iter().any(|e| e.entity() == *entity) { return Some(archetype.components()) } // TODO fix this
    }
    None
}

fn component_id_to_component_info(world: &World, component_id: ComponentId) -> Option<&ComponentInfo> {
    let components = world.components();
    components.get_info(component_id)
}

fn extract_component_name(component_info: &ComponentInfo) -> &str {
    component_info.name()
}


fn delete_marked_slaves
    (networking: Res<NetworkingResource>,) {
        if !networking.connected
            { return; }
        //TODO
}

fn delete_marked_masters
    (networking: Res<NetworkingResource>,) {
        if !networking.connected
            { return; }
        //TODO
}

impl NetworkingResource {
    fn send_all_unreliable
        (&self, bytes: Vec<u8>) {
            let networking = self.client.networking();

            for player in self.active_players.iter() {
                networking.send_p2p_packet(*player, SendType::Unreliable, &bytes);
            }
    }

    fn send_all_reliable
        (&self, bytes: Vec<u8>) {
            let networking = self.client.networking();

            for player in self.active_players.iter() {
                networking.send_p2p_packet(*player, SendType::Reliable, &bytes);
            }
    }

    pub fn create_networked_entity
        (&self,
         commands: &mut Commands,
         components: &[Box<impl Serializable>],
         entity: &Entity, 
         sync_periodically: bool,
         static_id: u16) {
            let mut object_info: u8 = 0;
            if sync_periodically { object_info |= 0b01000000; }

            commands.entity(*entity)
                .insert(SynchronizedMaster {
                    object_info,
                    static_id,
                    marked_for_deletion: false,
                });

            let mut bytes: Vec<u8> = Vec::new();

            bytes.push(MessageType::EntityCreate as u8);
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
