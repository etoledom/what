use ::pnet::datalink::Channel::Ethernet;
use ::pnet::datalink::DataLinkReceiver;
use ::pnet::datalink::{self, Config, NetworkInterface};
use ::std::collections::HashMap;

use crate::network::{Connection};

use std::net::{SocketAddr};
use super::lsof_utils;
use crate::os::lsof_utils::RawConnection;

pub fn get_datalink_channel(
    interface: &NetworkInterface,
) -> Result<Box<dyn DataLinkReceiver>, failure::Error> {
    match datalink::channel(interface, Config::default()) {
        Ok(Ethernet(_tx, rx)) => Ok(rx),
        Ok(_) => failure::bail!("Unknown interface type"),
        Err(e) => failure::bail!("Failed to listen to network interface: {}", e),
    }
}

pub fn get_open_sockets() -> HashMap<Connection, String> {
    let mut open_sockets = HashMap::new();

    let connections = lsof_utils::get_raw_connections_output();

    for raw_str in connections.lines() {
        let raw_connection_option = lsof_utils::RawConnection::new(raw_str);
        if raw_connection_option.is_none() {
            continue;
        }
        let raw_connection = raw_connection_option.unwrap();

        let protocol = raw_connection.get_protocol();
        let ip_address = raw_connection.get_ip_address();
        let remote_port = raw_connection.get_remote_port();
        let local_port = raw_connection.get_local_port();

        let socket_addr = SocketAddr::new(ip_address, remote_port);
        let connection = Connection::new(socket_addr, local_port, protocol).unwrap();

        open_sockets.insert(connection, raw_connection.process_name.clone());
    }

    return open_sockets;
}
