use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9900").await.unwrap();
    loop {
        let (mut stream, addr) = listener.accept().await?;
        stream.set_nodelay(true);
        loop {
            let mut in_buf = bytes::BytesMut::new();
            stream.read(&mut in_buf).await?;
            println!("{:?}", &in_buf);
        }
    }
    Ok(())
}
