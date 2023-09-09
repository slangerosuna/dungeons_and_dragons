use bevy::prelude::*;
use bevy_steamworks::*;

pub mod serializable_impls;

pub struct NetworkingPlugin {
    pub max_players: u16,
    pub max_synced_objects: u16,
    pub app_id: u32,
}

#[derive(Resource)]
pub struct NetworkingResource {
    pub player_id: u16,
    pub active_players: Vec<SteamId>,
    pub sync_messages: Vec<Vec<u8>>,
}

enum MessageType {
    EntityCreate,
    EntityDelete,
    EntityUpdate,
}

#[derive(Component)]
pub struct SynchronizedSlave {
    static_id: u16,
    marked_for_deletion: bool,
    destroy_on_owner_disconnect: bool,
    owner: u16,
}

#[derive(Component)]
pub struct SynchronizedMaster {
    static_id: u16,
    marked_for_deletion: bool,
}
impl SynchronizedMaster {
    pub fn destroy(&mut self) {
         self.marked_for_deletion = true;

         let mut bytes: Vec<u8> = Vec::new();

         bytes.push(MessageType::EntityDelete as u8);
         bytes.push(self.static_id.to_le_bytes()[0]);
         bytes.push(self.static_id.to_le_bytes()[1]);

         //TODO fix send(bytes);
    }
}


pub trait Serializable {
    fn from_bytes(&mut self, bytes: &[u8]);
    fn to_bytes(&self) -> Vec<u8>;

    fn get_type_id(&self) -> u16;
}

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
    (steam_client: Res<Client>) {
        //TODO
}

fn handle_networking
  (mut networking_res: ResMut<NetworkingResource>,
   steam_client: Res<Client>) {
        let networking = steam_client.networking();

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
     steam_client: Res<Client>,
     mut query: Query<&mut SynchronizedSlave>,) {
        //TODO
}

fn sync_master_entities
    (networking: Res<NetworkingResource>,
     steam_client: Res<Client>,
     mut query: Query<&mut SynchronizedMaster>,){
        //TODO
}

fn delete_marked_slaves
    (){
        //TODO
}

fn delete_marked_masters
    (){
        //TODO
}

fn send_all_unreliable
    (bytes: Vec<u8>, client: &Client, networking_res: &NetworkingResource) {
        let networking = client.networking();

        for player in networking_res.active_players.iter() {
            networking.send_p2p_packet(*player, SendType::Unreliable, &bytes);
        }
}

pub fn create_networked_entity
    (components: &[Box<dyn Serializable>],
     entity: Entity, static_id: u16) {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(MessageType::EntityCreate as u8);
        bytes.extend_from_slice(&static_id.to_le_bytes());

        for component in components {
            let type_id = component.get_type_id().to_le_bytes();
            let comp_bytes = component.to_bytes();

            bytes.extend_from_slice(&type_id);
            bytes.push(comp_bytes.len().try_into().unwrap());
            bytes.extend_from_slice(&comp_bytes);
        }

        //TODO fix send(bytes);
}

