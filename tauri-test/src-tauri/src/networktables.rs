use std::net::{Ipv4Addr, SocketAddrV4};
use network_tables::v4::{Client, Config, Subscription};
use network_tables::v4::client_config::default_should_reconnect;
use network_tables::v4::subscription::SubscriptionOptions;

#[tokio::main]
pub async fn nt4() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt::init();

  let client: Client = Client::try_new_w_config(
    SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5810),
        Config {
            connect_timeout: 5000,
            disconnect_retry_interval: 10000,
            should_reconnect: Box::new(default_should_reconnect),
            on_announce: Box::new(|_| Box::pin(async {
              tracing::info!("Announced");
            })),
            on_un_announce: Box::new(|_| Box::pin(async {
              tracing::info!("Un-announced");
            })),
            on_disconnect: Box::new(|| Box::pin(async {
              tracing::info!("Disconnected");
            })),
            on_reconnect: Box::new(|| Box::pin(async {
              tracing::info!("Reconnected");
            })),
        }
    ).await?;
  tracing::info!("Client created");

  let published_topic = client
      .publish_topic("/Test/number", network_tables::v4::Type::Int, None)
      .await?;
  tracing::info!("Topic published");

  let mut subscription: Subscription = client
      .subscribe_w_options(
        &[""],
        Some(SubscriptionOptions {
            periodic: Option::from(100),
            all: Option::from(true),
            topics_only: Option::from(false),
            prefix: Option::from(false),
            rest: None,
        })
      )
      .await?;
  tracing::info!("Subscription created");

  let task_client: Client = client.clone();
  tokio::spawn(async move {
    let mut counter: i32 = 0;
    loop {
      task_client
          .publish_value(&published_topic, &network_tables::Value::from(counter))
          .await
          .unwrap();
      counter += 1;
      tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }
  });
  tracing::info!("Task spawned");

  while let Some(message) = subscription.next().await {
    tracing::info!("{:?}", message);
    println!("{:?}", message);
  }

  Ok(())
}
    