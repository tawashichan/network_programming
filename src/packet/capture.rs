
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{Packet, MutablePacket};
use pnet::packet::arp::{ArpPacket,ArpOperation};
use pnet::packet::ethernet::{EthernetPacket,EtherTypes};
use pnet::packet::ipv4::{Ipv4Packet};
use std::env;

pub fn get_packet(){
    let interface_name = env::args().nth(1).unwrap();

    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();

    // Create a new channel, dealing with layer 2 packets
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                match EthernetPacket::new(packet) {
                    Some(packet) => {
                        read_ethernet_packet(&packet)
                    }
                    _ => {
                        continue;
                    }
                }
            },
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

fn read_ethernet_packet(packet: &EthernetPacket) {
    match packet.get_ethertype() {
        EtherTypes::Arp => {
            let p = ArpPacket::new(packet.payload());
            match p {
                Some(packet) => {
                    //println!("{:?}",packet);
                }
                _ => {}
            }
        }
        EtherTypes::Ipv4 => {
            let packet = Ipv4Packet::new(packet.payload());
            match packet {
                Some(packet) => {
                    println!("{:?}",packet);
                    println!("{:?}",packet.payload())
                }
                _ => {}
            }
        }
        _ => {}
    }
}