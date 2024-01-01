// Copyright (c) 2023-2024 Liu Qiang <liulcsy@qq.com> In Wuhan, China
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
pub mod pubunit;
use pubunit::*;

pub fn get_system_ver(osn:&str) -> String{
    let res;
    match osn{
        "ubuntu" => {
            let command = "cat /etc/issue";
            let mut os_ver = doshell_out(command);
            if os_ver.chars().count() !=0 {
                os_ver = os_ver.replace("\\l", "");
                os_ver = os_ver.replace("\\n", "");
                let res_tmp = os_ver.trim();
                res = res_tmp.to_string();
            }  else {
                res = "err".to_string();
            } 
        },
        "uos"=> {
            res = "uos ok".to_string();
        },
        "kylin"=> {
            let command = "cat /etc/.kyinfo";
            let os_ver = doshell_out(command);
            if os_ver.chars().count() !=0 {
                let fname = "osver.ini";
                write_conf(&fname,&os_ver);
                res = read_conf(fname,"dist","milestone");
            }  else {
                res = "err".to_string();
            } 
        },
        //&_ => todo!(),
        _=> res = "err".to_string(),
    }
    return res.to_string();
}


