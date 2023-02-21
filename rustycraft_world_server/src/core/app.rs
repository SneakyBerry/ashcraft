use super::events::packets::ClientPacketReceived;
use crate::core::events::packets::ServerPacketSend;
use crate::session_handler::Connection;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_time::TimePlugin;
use rustycraft_world_packets::prelude::*;
use crate::core::systems::prelude::*;

#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct Connections(HashMap<Entity, Connection>);

pub fn get_app<RC: 'static, SN: 'static>(incoming_connections: RC, realm_server_sender: SN) -> App {
    let mut app = App::new();
    app.add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .insert_non_send_resource(incoming_connections)
        .insert_non_send_resource(realm_server_sender)
        .init_resource::<Connections>()
        .init_resource::<Time>()
        .add_event::<ServerPacketSend>()
        .add_event::<ClientPacketReceived<CmsgNameQuery>>()
        .add_event::<ClientPacketReceived<CmsgItemQuerySingle>>()
        .add_event::<ClientPacketReceived<CMovementData>>()
        .add_event::<ClientPacketReceived<CmsgPlayerLogin>>()
        .add_system_to_stage(CoreStage::First, handle_incoming_connections.at_start())
        .add_system_to_stage(
            CoreStage::First,
            handle_opcodes.after(handle_incoming_connections),
        )
        .add_system_to_stage(CoreStage::PreUpdate, handle_player_login)
        .add_system_to_stage(CoreStage::PostUpdate, send_updates)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            send_position_update.before(send_updates),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            send_player_update.before(send_updates),
        )
        .add_system_to_stage(CoreStage::PostUpdate, sync_time.after(send_updates))
        .add_system_to_stage(CoreStage::PostUpdate, swap_update_data.at_end());
    app
}
