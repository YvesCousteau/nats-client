use bytes::Bytes;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let client = async_nats::connect("demo.nats.io").await?;
    let mut subscriber = client.subscribe("messages").await?;
    for _ in 0..10 {
        client.publish("messages", "data".into()).await?;
    }
    while let Some(message) = subscriber.next().await {
        println!("Received message {:?}", message);
    }

    Ok(())
}
