// Copyright (c) 2023-2024 Liu Qiang <liulcsy@qq.com> In Wuhan, China
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate pnet;
use pnet::datalink::NetworkInterface;

pub mod netcard;
pub mod system;
use netcard::*;
use system::*;

pub fn get_pc_net_card_info() -> Vec<NetworkInterface> {
    return get_ac_net_card_info();
}

pub fn get_pc_net_card_name()->String{
    return get_ac_net_card_name().to_string();
}

pub fn get_pc_ipv4()->String{
    return get_ipv4().to_string();
}

pub fn get_pc_ipv6()->String{
    return get_ipv6().to_string();
}

pub fn get_pc_mac()->String{
    return get_ac_mac().to_string();
}

pub fn get_pc_system_ver(osname:&str) -> String{
    return get_system_ver(osname).to_string();
}

pub fn get_pc_system_check_pname(osname:&str) -> String{
    return get_system_check_pname(osname).to_string();
}
pub fn get_pc_system_is_vm() -> bool{
    return get_system_is_vm();
}
