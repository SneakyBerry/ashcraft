use crate::core::app::get_app;
use crate::core::prelude::*;
use crate::session_handler::Connection;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

const TICK_TIME: Duration = Duration::from_millis(1000 / 60);

pub struct WorldHandler {
    incoming_connections: IncomingConnections,
    realm_server_sender: mpsc::UnboundedSender<Connection>,
}

type IncomingConnections = mpsc::UnboundedReceiver<Connection>;

impl WorldHandler {
    pub fn new(
        incoming_connections: IncomingConnections,
        realm_server_sender: mpsc::UnboundedSender<Connection>,
    ) -> Self {
        Self {
            incoming_connections,
            realm_server_sender,
        }
    }

    pub fn run(self) {
        let mut app = get_app(self.incoming_connections, self.realm_server_sender);

        info!("World server started");
        let mut sleep_time = Duration::default();
        loop {
            std::thread::sleep(sleep_time);
            let t = Instant::now();
            app.update();
            sleep_time = TICK_TIME.saturating_sub(t.elapsed());
        }
    }
}
