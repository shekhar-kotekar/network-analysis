use std::thread;

use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use tracing::{error, info};

fn main() {
    tracing_subscriber::fmt().with_thread_names(true).init();
    let interfaces = datalink::interfaces();
    let mut handles = vec![];

    for interface in interfaces {
        info!("Creating thread for interface: {:?}", interface.name);
        let handle = thread::spawn(move || {
            capture_packets(interface.clone());
        });
        handles.push(handle);
    }
    info!("number of interfaces: {}", handles.len());
    for handle in handles {
        handle.join().unwrap();
    }
}

fn capture_packets(interface: datalink::NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => error!(
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
