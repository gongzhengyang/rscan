use std::net::Ipv4Addr;
use std::sync::Mutex;

use hashbrown::HashMap;
use tokio::sync::OnceCell;

pub static SEND_IP_MONITOR: OnceCell<Mutex<HashMap<Ipv4Addr, u8>>> = OnceCell::const_new();

async fn get_send_ip_monitor_handle() -> &'static Mutex<HashMap<Ipv4Addr, u8>> {
    SEND_IP_MONITOR.get_or_init(||async {
        Mutex::new(HashMap::new())
    }).await
}

async fn add_send_ip_monitor(host: Ipv4Addr) {
    let mut cache = get_send_ip_monitor_handle().await.lock().unwrap();
    let value = cache.entry(host).or_insert(0);
    *value += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_ip() {
        for i in 0..10 {
            add_send_ip_monitor(Ipv4Addr::from(i)).await;
        }
        for i in 0..3 {
            add_send_ip_monitor(Ipv4Addr::from(i)).await;
        }
        let cache = get_send_ip_monitor_handle().await;
        println!("{:?}", cache.lock().unwrap());
    }
}