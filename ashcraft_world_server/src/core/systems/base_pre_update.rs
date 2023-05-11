use crate::core::systems::prelude::*;
use bevy::prelude::*;

pub(crate) fn handle_incoming_connections(
    mut commands: Commands,
    mut connections: ResMut<Connections>,
    mut connection_events: EventWriter<ClientPacketReceived<CmsgPlayerLogin>>,
    mut incoming_connections: NonSendMut<mpsc::UnboundedReceiver<Connection>>,
) {
    while let Ok(connection) = incoming_connections.try_recv() {
        let ConnectionState::WorldLogin(pkt) = &connection.state else {panic!("Invalid connection state!")};
        let entity = commands.spawn(LoggedIn).id();
        // It is a unique system then we don't care about order of event send and insertion
        // No one can receive this event until system return
        connection_events.send(ClientPacketReceived(entity, Opcode::CmsgPlayerLogin, *pkt));
        connections.insert(entity, (connection, vec![]));
    }
}

/// Swap update buffer each tick
pub(crate) fn swap_update_data(
    mut players: Query<&mut Player>,
    mut units: Query<&mut Unit>,
    mut objects: Query<&mut crate::core::components::object::Object>,
) {
    players.iter_mut().for_each(|mut p| (*p).swap());
    units.iter_mut().for_each(|mut u| (*u).swap());
    objects.iter_mut().for_each(|mut o| (*o).swap());
}
