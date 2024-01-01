// Copyright (c) 2023-2024 Liu Qiang <liulcsy@qq.com> In Wuhan, China
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
extern crate pnet;
use pnet::datalink;
use pnet::ipnetwork;
use pnet::datalink::NetworkInterface;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

pub fn get_ac_net_card_info() -> Vec<NetworkInterface>{
    let interfaces = datalink::interfaces();
    return interfaces;
}

pub fn get_ac_net_card_name() -> String{
    let mut r = "0".to_string();
    let interfaces = datalink::interfaces();
    for interface in interfaces {
        let ip_v4:Vec<Ipv4Addr> =  interface.ips.iter().map(|ip| match ip {
            ipnetwork::IpNetwork::V4(ref ipv4) => Ok(ipv4.ip()),
            _ => Err(""),
        }).filter_map(Result::ok).collect();

        #[cfg(unix)]
        if !ip_v4.is_empty() && !interface.is_loopback() && interface.is_running() && interface.is_up() {
            r= interface.name;
        }     
       
    }
    return r.to_string();
}

pub fn get_ipv4() -> String{
    let mut r = "0".to_string();
    let interfaces = datalink::interfaces();
    for interface in interfaces {
        let ip_v4:Vec<Ipv4Addr> =  interface.ips.iter().map(|ip| match ip {
            ipnetwork::IpNetwork::V4(ref ipv4) => Ok(ipv4.ip()),
            _ => Err(""),
        }).filter_map(Result::ok).collect();

        #[cfg(unix)]
        if !ip_v4.is_empty() && !interface.is_loopback() && interface.is_running() && interface.is_up() {
            r= ip_v4[0].to_string();
        }     
    }
    return r.to_string();
}

pub fn get_ipv6() -> String{
    let mut r = "0".to_string();
    let interfaces = datalink::interfaces();
    for interface in interfaces {
        let ip_v6:Vec<Ipv6Addr> =  interface.ips.iter().map(|ip| match ip {
            ipnetwork::IpNetwork::V6(ref ipv6) => Ok(ipv6.ip()),
                _ => Err(""),
            }).filter_map(Result::ok).collect();
        #[cfg(unix)]
        if !ip_v6.is_empty() && !interface.is_loopback() && interface.is_running() && interface.is_up() {
            r= ip_v6[0].to_string();
        }     
    }
    return r.to_string();
}

pub fn get_ac_mac() -> String{
    let mut r = "0".to_string();
    let interfaces = datalink::interfaces();
    for interface in interfaces {
        let ip_v4:Vec<Ipv4Addr> =  interface.ips.iter().map(|ip| match ip {
            ipnetwork::IpNetwork::V4(ref ipv4) => Ok(ipv4.ip()),
            _ => Err(""),
        }).filter_map(Result::ok).collect();

        #[cfg(unix)]
        if !ip_v4.is_empty() && !interface.is_loopback() && interface.is_running() && interface.is_up() {
            r= interface.mac.expect("REASON").to_string();
        }     
    }
    return r.to_string();
}
