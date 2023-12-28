// Copyright (c) 2023-2024 Liu Qiang <liulcsy@qq.com> In Wuhan, China
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod unit;
use unit::*;

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
