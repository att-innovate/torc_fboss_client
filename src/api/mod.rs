// The MIT License (MIT)
//
// Copyright (c) 2015 AT&T
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::net;

use {Protocol, Transport};
use protocol::{MessageType, Type};
use protocol::binary_protocol::BinaryProtocol;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub struct PortStat {
    pub id: i32,
    pub connected: bool,
}

pub struct Route {
    pub from: String,
    pub to: String,
}

pub fn get_ports_stats(connect_string: &str) -> Vec<PortStat> {
    let mut transport = net::TcpStream::connect(connect_string).unwrap();
    let mut protocol = BinaryProtocol;

    let request_number = 99;

    send_request_for_all_ports_stats(&mut protocol, &mut transport, request_number);
    let result = handle_response_for_all_ports_stats(&mut protocol, &mut transport, request_number);
    result
}

pub fn get_routes(connect_string: &str) -> Vec<Route> {
    let mut transport = net::TcpStream::connect(connect_string).unwrap();
    let mut protocol = BinaryProtocol;

    let request_number = 100;

    send_request_for_get_routes(&mut protocol, &mut transport, request_number);
    let result = handle_response_for_get_routes(&mut protocol, &mut transport, request_number);
    result
}

pub fn sync_routes(connect_string: &str) {
    let mut transport = net::TcpStream::connect(connect_string).unwrap();
    let mut protocol = BinaryProtocol;

    let request_number = 101;

    send_request_for_sync_fib(&mut protocol, &mut transport, request_number);
    handle_default_response(&mut protocol, &mut transport, request_number);
}

pub fn add_route(connect_string: &str, route_from: &str, route_to: &str) {
    let mut transport = net::TcpStream::connect(connect_string).unwrap();
    let mut protocol = BinaryProtocol;

    let request_number = 102;

    send_request_for_add_route(&mut protocol,
                               &mut transport,
                               request_number,
                               route_from,
                               route_to);
    handle_default_response(&mut protocol, &mut transport, request_number);
}

pub fn delete_route(connect_string: &str, route_from: &str) {
    let mut transport = net::TcpStream::connect(connect_string).unwrap();
    let mut protocol = BinaryProtocol;

    let request_number = 103;

    send_request_for_delete_route(&mut protocol, &mut transport, request_number, route_from);
    println!("sent, waiting for response");
    handle_default_response(&mut protocol, &mut transport, request_number);
}



fn send_request_for_all_ports_stats<P, T>(protocol: &mut P, transport: &mut T, request_number: i32)
    where P: Protocol,
          T: Transport
{

    protocol.write_message_begin(transport,
                                 "getAllPortStats",
                                 MessageType::Call,
                                 request_number)
            .unwrap();
    protocol.write_struct_begin(transport, "getAllPortStats_args").unwrap();
    protocol.write_field_stop(transport).unwrap();
    protocol.write_struct_end(transport).unwrap();
    protocol.write_message_end(transport).unwrap();
}

fn handle_response_for_all_ports_stats<P, T>(protocol: &mut P,
                                             transport: &mut T,
                                             _request_number: i32)
                                             -> Vec<PortStat>
    where P: Protocol,
          T: Transport
{

    let mut result: Vec<PortStat> = vec![];

    let response = protocol.read_message_begin(transport).unwrap();
    match response {
        (name, MessageType::Reply, _request_number) => println!("-- Response for {}", name),
        (_, _, _) => {
            println!("wrong response");
            return result;
        }
    }

    protocol.read_struct_begin(transport).unwrap();
    loop {
        let field = protocol.read_field_begin(transport).unwrap();
        match field {
            (_, Type::Stop, _) => break,
            (_, _, 0) => {
                let (_, _, map_size) = protocol.read_map_begin(transport).unwrap();
                for _ in 0..map_size {
                    result.push(read_ports_stat_field(protocol, transport));
                }
            }
            (_, field_type, _) => protocol.skip(transport, field_type).unwrap(),
        }
        protocol.read_field_end(transport).unwrap();
    }
    protocol.read_struct_end(transport).unwrap();

    result
}


fn read_ports_stat_field<P, T>(protocol: &mut P, transport: &mut T) -> PortStat
    where P: Protocol,
          T: Transport
{

    let key = protocol.read_i32(transport).unwrap();
    let mut connected = false;

    protocol.read_struct_begin(transport).unwrap();
    loop {
        let port_stat_field = protocol.read_field_begin(transport).unwrap();
        match port_stat_field {
            (_, Type::Stop, _) => break,
            (_, _, 4) => {
                let oper_state = protocol.read_i32(transport).unwrap();
                connected = if oper_state == 1 {
                    true
                } else {
                    false
                };
            }
            (_, port_stat_field_type, _) => protocol.skip(transport, port_stat_field_type).unwrap(),
        }
        protocol.read_field_end(transport).unwrap();
    }
    protocol.read_struct_end(transport).unwrap();

    PortStat {
        id: key,
        connected: connected,
    }
}



fn send_request_for_get_routes<P, T>(protocol: &mut P, transport: &mut T, request_number: i32)
    where P: Protocol,
          T: Transport
{

    protocol.write_message_begin(transport,
                                 "getRouteTable",
                                 MessageType::Call,
                                 request_number)
            .unwrap();
    protocol.write_struct_begin(transport, "getRouteTable_args").unwrap();
    protocol.write_field_stop(transport).unwrap();
    protocol.write_struct_end(transport).unwrap();
    protocol.write_message_end(transport).unwrap();
}

fn handle_response_for_get_routes<P, T>(protocol: &mut P,
                                        transport: &mut T,
                                        _request_number: i32)
                                        -> Vec<Route>
    where P: Protocol,
          T: Transport
{

    let mut result: Vec<Route> = vec![];

    let response = protocol.read_message_begin(transport).unwrap();
    match response {
        (name, MessageType::Reply, _request_number) => println!("-- Response for {}", name),
        (_, _, _) => {
            println!("wrong response");
            return result;
        }
    }

    protocol.read_struct_begin(transport).unwrap();
    loop {
        let field = protocol.read_field_begin(transport).unwrap();
        match field {
            (_, Type::Stop, _) => break,
            (_, _, 0) => {
                let (_, list_size) = protocol.read_list_begin(transport).unwrap();
                for _ in 0..list_size {
                    result.push(read_unicast_route(protocol, transport));
                }
                protocol.read_list_end(transport).unwrap();
            }
            (_, field_type, _) => protocol.skip(transport, field_type).unwrap(),
        }
        protocol.read_field_end(transport).unwrap();
    }
    protocol.read_struct_end(transport).unwrap();

    result
}

fn read_unicast_route<P, T>(protocol: &mut P, transport: &mut T) -> Route
    where P: Protocol,
          T: Transport
{
    let mut from: String = "undefined".to_string();
    let mut to: String = "undefined".to_string();

    protocol.read_struct_begin(transport).unwrap();
    loop {
        let route_field = protocol.read_field_begin(transport).unwrap();
        match route_field {
            (_, Type::Stop, _) => break,
            (_, _, 1) => {
                from = read_ip_from(protocol, transport);
            }
            (_, _, 2) => {
                to = read_ip_to(protocol, transport);
            }
            (_, port_stat_field_type, _) => protocol.skip(transport, port_stat_field_type).unwrap(),
        }
        protocol.read_field_end(transport).unwrap();
    }
    protocol.read_struct_end(transport).unwrap();

    Route {
        from: from.to_string(),
        to: to.to_string(),
    }
}

fn read_ip_from<P, T>(protocol: &mut P, transport: &mut T) -> String
    where P: Protocol,
          T: Transport
{
    let mut from: String = "undefined".to_string();
    let mut mask: i16 = 0;

    protocol.read_struct_begin(transport).unwrap();
    loop {
        let route_from = protocol.read_field_begin(transport).unwrap();
        match route_from {
            (_, Type::Stop, _) => break,
            (_, _, 1) => {
                from = read_binary_address(protocol, transport);
            }
            (_, _, 2) => {
                mask = protocol.read_i16(transport).unwrap();
            }
            (_, port_stat_field_type, _) => protocol.skip(transport, port_stat_field_type).unwrap(),
        }
        protocol.read_field_end(transport).unwrap();
    }
    protocol.read_struct_end(transport).unwrap();

    format!("{}/{}", from, mask).to_string()
}

fn read_ip_to<P, T>(protocol: &mut P, transport: &mut T) -> String
    where P: Protocol,
          T: Transport
{
    let mut to_address: String = "".to_string();

    let to_list = protocol.read_list_begin(transport).unwrap();
    match to_list {
        (_, size) => {
            // we handle one next hop only
            for _ in 0..size {
                to_address = read_binary_address(protocol, transport);
            }
        }
    }

    protocol.read_list_end(transport).unwrap();

    to_address.to_string()
}


fn read_binary_address<P, T>(protocol: &mut P, transport: &mut T) -> String
    where P: Protocol,
          T: Transport
{
    let mut from_address: String = "undefined".to_string();

    protocol.read_struct_begin(transport).unwrap();
    loop {
        let route_from = protocol.read_field_begin(transport).unwrap();
        match route_from {
            (_, Type::Stop, _) => break,
            (_, _, 1) => {
                let address = protocol.read_binary(transport).unwrap();

                if address.len() == 4 {
                    let ipv4 = Ipv4Addr::new(address[0], address[1], address[2], address[3]);
                    from_address = format!("{:?}", ipv4);
                } else {
                    let ipv6 = Ipv6Addr::new(((address[0] as u16) << 8) + (address[1] as u16),
                                             ((address[2] as u16) << 8) + (address[3] as u16),
                                             ((address[4] as u16) << 8) + (address[5] as u16),
                                             ((address[6] as u16) << 8) + (address[7] as u16),
                                             ((address[9] as u16) << 8) + (address[8] as u16),
                                             ((address[11] as u16) << 8) + (address[10] as u16),
                                             ((address[13] as u16) << 8) + (address[12] as u16),
                                             ((address[15] as u16) << 8) + (address[14] as u16));
                    from_address = format!("{:?}", ipv6);
                }
            }
            (_, port_stat_field_type, _) => protocol.skip(transport, port_stat_field_type).unwrap(),
        }
        protocol.read_field_end(transport).unwrap();
    }
    protocol.read_struct_end(transport).unwrap();

    from_address.to_string()
}


fn send_request_for_sync_fib<P, T>(protocol: &mut P, transport: &mut T, request_number: i32)
    where P: Protocol,
          T: Transport
{

    protocol.write_message_begin(transport, "syncFib", MessageType::Call, request_number)
            .unwrap();
    protocol.write_struct_begin(transport, "syncFib_args").unwrap();
    protocol.write_field_stop(transport).unwrap();
    protocol.write_struct_end(transport).unwrap();
    protocol.write_message_end(transport).unwrap();
}


fn send_request_for_add_route<P, T>(protocol: &mut P,
                                    transport: &mut T,
                                    request_number: i32,
                                    route_from: &str,
                                    route_to: &str)
    where P: Protocol,
          T: Transport
{

    protocol.write_message_begin(transport,
                                 "addUnicastRoute",
                                 MessageType::Call,
                                 request_number)
            .unwrap();
    protocol.write_struct_begin(transport, "addUnicastRoute_args").unwrap();
    handle_write_add_route_args(protocol, transport, route_from, route_to);
    protocol.write_field_stop(transport).unwrap();
    protocol.write_struct_end(transport).unwrap();
    protocol.write_message_end(transport).unwrap();
}

fn handle_write_add_route_args<P, T>(protocol: &mut P,
                                     transport: &mut T,
                                     route_from: &str,
                                     route_to: &str)
    where P: Protocol,
          T: Transport
{
    let route_elements: Vec<&str> = route_from.split("/").collect();
    let addr_from = Ipv4Addr::from_str(route_elements[0]).unwrap();
    let prefix_length = i16::from_str(route_elements[1]).unwrap();

    let addr_to = Ipv4Addr::from_str(route_to).unwrap();

    protocol.write_field_begin(transport, "clientId", Type::I16, 1).unwrap();
    protocol.write_i16(transport, 1).unwrap();
    protocol.write_field_end(transport).unwrap();

    protocol.write_field_begin(transport, "r", Type::Struct, 2).unwrap();
    protocol.write_struct_begin(transport, "UnicastRoute").unwrap();
    protocol.write_field_begin(transport, "dest", Type::Struct, 1).unwrap();
    protocol.write_struct_begin(transport, "IpPrefix").unwrap();

    protocol.write_field_begin(transport, "ip", Type::Struct, 1).unwrap();
    handle_write_binary_address(protocol, transport, &addr_from);
    protocol.write_field_end(transport).unwrap();

    protocol.write_field_begin(transport, "prefixLength", Type::I16, 2).unwrap();
    protocol.write_i16(transport, prefix_length).unwrap();
    protocol.write_field_end(transport).unwrap();
    protocol.write_field_stop(transport).unwrap();
    protocol.write_struct_end(transport).unwrap();
    protocol.write_field_end(transport).unwrap();

    protocol.write_field_begin(transport, "nextHopAddrs", Type::List, 2).unwrap();
    protocol.write_list_begin(transport, Type::Struct, 1).unwrap();

    handle_write_binary_address(protocol, transport, &addr_to);

    protocol.write_list_end(transport).unwrap();
    protocol.write_field_end(transport).unwrap();

    protocol.write_field_stop(transport).unwrap();
    protocol.write_struct_end(transport).unwrap();
    protocol.write_field_end(transport).unwrap();
}


fn send_request_for_delete_route<P, T>(protocol: &mut P,
                                       transport: &mut T,
                                       request_number: i32,
                                       route_from: &str)
    where P: Protocol,
          T: Transport
{
    let route_elements: Vec<&str> = route_from.split("/").collect();
    let prefix_length = i16::from_str(route_elements[1]).unwrap();

    protocol.write_message_begin(transport,
                                 "deleteUnicastRoute",
                                 MessageType::Call,
                                 request_number)
            .unwrap();
    protocol.write_struct_begin(transport, "deleteUnicastRoute_args").unwrap();
    handle_write_delete_route_args(protocol, transport, route_elements[0], prefix_length);
    protocol.write_field_stop(transport).unwrap();
    protocol.write_struct_end(transport).unwrap();
    protocol.write_message_end(transport).unwrap();
}

fn handle_write_delete_route_args<P, T>(protocol: &mut P,
                                        transport: &mut T,
                                        route_from: &str,
                                        prefix_length: i16)
    where P: Protocol,
          T: Transport
{
    let addr = Ipv4Addr::from_str(route_from).unwrap();

    protocol.write_field_begin(transport, "clientId", Type::I16, 1).unwrap();
    protocol.write_i16(transport, 1).unwrap();
    protocol.write_field_end(transport).unwrap();

    protocol.write_field_begin(transport, "r", Type::Struct, 2).unwrap();
    protocol.write_struct_begin(transport, "IpPrefix").unwrap();

    protocol.write_field_begin(transport, "ip", Type::Struct, 1).unwrap();
    handle_write_binary_address(protocol, transport, &addr);
    protocol.write_field_end(transport).unwrap();

    protocol.write_field_begin(transport, "prefixLength", Type::I16, 2).unwrap();
    protocol.write_i16(transport, prefix_length).unwrap();
    protocol.write_field_end(transport).unwrap();

    protocol.write_field_stop(transport).unwrap();
    protocol.write_struct_end(transport).unwrap();
    protocol.write_field_end(transport).unwrap();
}


fn handle_write_binary_address<P, T>(protocol: &mut P, transport: &mut T, address: &Ipv4Addr)
    where P: Protocol,
          T: Transport
{
    protocol.write_struct_begin(transport, "BinaryAddress").unwrap();
    protocol.write_field_begin(transport, "addr", Type::String, 1).unwrap();
    protocol.write_binary(transport, &address.octets()).unwrap();
    protocol.write_field_end(transport).unwrap();
    protocol.write_field_begin(transport, "port", Type::I64, 2).unwrap();
    protocol.write_i64(transport, 0).unwrap();
    protocol.write_field_end(transport).unwrap();
    protocol.write_field_stop(transport).unwrap();
    protocol.write_struct_end(transport).unwrap();

}

fn handle_default_response<P, T>(protocol: &mut P, transport: &mut T, _request_number: i32)
    where P: Protocol,
          T: Transport
{
    let response = protocol.read_message_begin(transport).unwrap();
    match response {
        (name, MessageType::Reply, _request_number) => println!("-- Response for {}", name),
        (_, _, _) => {
            println!("wrong response");
        }
    }
}
