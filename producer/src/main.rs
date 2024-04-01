use std::thread;

use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use tracing::info;

fn main() {
    tracing_subscriber::fmt().with_thread_names(true).init();
    //TODO: read network interface name from config
    let network_interface_name = "en0";
    let interface = datalink::interfaces()
        .into_iter()
        .filter(|iface| iface.name == network_interface_name)
        .next()
        .expect("Failed to get interface");

    info!("Capturing packets for '{:?}' interface", interface.name);
    //TODO: Give interface name to the thread
    let handle = thread::spawn(move || {
        capture_packets(interface.clone());
    });
    handle.join().unwrap();
}

fn capture_packets(interface: datalink::NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, datalink::Config::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "Failed to create datalink channel for interface {}: {}",
            interface.name, e
        ),
    };
    info!("Start reading packets...");
    loop {
        match rx.next() {
            Ok(packet) => {
                info!("Received packet: {:?}", packet);
            }
            Err(e) => {
                info!("Failed to read packet: {}", e);
            }
        }
    }
}
