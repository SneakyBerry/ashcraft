use super::events::packets::ClientPacketReceived;
use super::systems::*;
use crate::core::events::packets::ServerPacketSend;
use bevy::prelude::*;
use bevy_time::TimePlugin;
use rustycraft_world_packets::position::CMovementData;
use rustycraft_world_packets::query::{CmsgItemQuerySingle, CmsgNameQuery};

#[derive(Debug, StageLabel)]
enum Stages {
    GetConnections,
    BeforeUpdate,
    Update,
    AfterUpdate,
}

pub fn get_app<RC: 'static, SN: 'static>(
    incoming_connections: RC,
    outgoing_connections: SN,
) -> App {
    let mut app = App::new();
    app.add_plugin(CorePlugin::default())
        .add_plugin(TimePlugin::default())
        .insert_non_send_resource(incoming_connections)
        .insert_non_send_resource(outgoing_connections)
        .init_resource::<Time>()
        .add_event::<ServerPacketSend>()
        .add_event::<ClientPacketReceived<CmsgNameQuery>>()
        .add_event::<ClientPacketReceived<CmsgItemQuerySingle>>()
        .add_event::<ClientPacketReceived<CMovementData>>()
        .add_system_to_stage(CoreStage::First, handle_incoming_connections.at_start())
        .add_system_to_stage(CoreStage::First, handle_opcodes.at_start())
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
        .add_system_to_stage(CoreStage::PostUpdate, debug_events.at_end());
    app
}

fn debug_events(mut events: EventReader<ClientPacketReceived<CMovementData>>) {
    if events.len() != 0 {
        println!("{:?}", &events.len());
    }
    for _ in events.iter() {
    }
}
