
use std::{collections::HashMap, net::{Ipv4Addr, IpAddr}, time::Duration, sync::Arc};

use parking_lot::Mutex;
use surge_ping::{Client, Config, PingIdentifier, PingSequence};

use crate::error::{EnokiError, log_result_consume};

const PING_COUNT: usize = 3;

pub async fn ping_addresses(
    addresses: Vec<Ipv4Addr>,
) -> Result<HashMap<Ipv4Addr, bool>, EnokiError> {
    let mut map = HashMap::with_capacity(addresses.len());

    let client = Client::new(&Config::default())?;

    let mut tasks = Vec::with_capacity(addresses.len());

    for addr in addresses {
        let task = tokio::spawn(ping_task(client.clone(), addr));
        tasks.push(task);
    }

    for task in tasks {
        match task.await {
            Ok((addr, result)) => {
                map.insert(addr, result);
            }
            Err(err) => {
                tracing::error!("Ping error: {}", err);
            }
        }
    }

    Ok(map)
}

async fn ping_task(client: Client, addr: Ipv4Addr) -> (Ipv4Addr, bool) {
    let result = ping(client, IpAddr::V4(addr)).await;

    (addr, result.unwrap_or(false))
}

async fn ping(client: Client, addr: IpAddr) -> Result<bool, EnokiError> {
    let payload = [0; 8];

    let mut pinger = client.pinger(addr, PingIdentifier(rand::random())).await;
    pinger.timeout(Duration::from_secs(1));

    let mut interval = tokio::time::interval(Duration::from_secs(1));

    for idx in 0..PING_COUNT {
        interval.tick().await;
        match pinger.ping(PingSequence(idx as u16), &payload).await {
            Ok(_) => {
                return Ok(true);
            }
            Err(surge_ping::SurgeError::Timeout { .. }) => {
                // do nothing
            }
            Err(err) => {
                tracing::error!("Ping error: {}", err);
            }
        }
    }

    Ok(false)
}