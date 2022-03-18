use battlenet_server::{load_certs, load_keys, Server};
use battlenet_server::socket_manager::SocketManager;
use battlenet_server::web_handler::WebServiceHandler;

mod logging;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = logging::init_logging();
    let certs = load_certs("./authserver.cert.pem")?;
    let mut keys = load_keys("./authserver.key.pem")?;
    let tls_context = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))
        .unwrap();

    let session_manager = SocketManager::builder().build(tls_context.clone())?;
    let a = WebServiceHandler{ bind_address: "0.0.0.0:9990", tls_context: tls_context.clone() };
    tokio::spawn(a.serve());
    session_manager.run_forever::<Server>().await?;
    Ok(())
}
