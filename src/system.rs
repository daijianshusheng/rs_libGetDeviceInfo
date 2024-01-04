// Copyright (c) 2023-2024 Liu Qiang <liulcsy@qq.com> In Wuhan, China
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
pub mod pubunit;
use pubunit::*;

// flase is Real machine
pub fn get_system_is_vm() -> bool{
    let command = "systemd-detect-virt";
	let bret = doshell_out(command);
	if bret == "none" {
		return false;
	}
	return true;
}

//check proccess name
pub fn get_system_check_pname(osn:&str) -> String{
    let res;
    let command = "ps -aux |grep ";
    let tmpcom = command.to_owned() + &osn;
    let allcom = tmpcom + "|grep -v grep |awk '{for(i=10+1;i<=NF;i++)printf $i}'";
    let pname = doshell_out(&allcom);
    if pname.chars().count() !=0 {
        let res_tmp = pname;
        res = res_tmp.to_string();
    } else {
        res = "can not find proccess name".to_string();
    } 
    return res.to_string();
}

//Get system version
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
                res = "Not a ubuntu operating system".to_string();
            } 
        },
        "uos"=> {
            let command = "cat /etc/product-info";
            let os_ver = doshell_out(command);
            if os_ver.chars().count() !=0 {
                let command = "cat /etc/os-version";
                let os_ver = doshell_out(command);
                if os_ver.chars().count() !=0 {
                    let fname = "osver.ini";
                    write_conf(&fname,&os_ver);
                    let ver_pro = read_conf(fname,"Version","EditionName[zh_CN]");
                    let ver_no = read_conf(fname,"Version","MinorVersion");
                    let res_tmp = ver_pro+ "(" + &ver_no + ")";
                    res = res_tmp.to_string();
                } else {
                    res = "err".to_string();
                }
             } else {
                res = "Not a UOS operating system".to_string();
            }
        },
        "kylin"=> {
            let command = "cat /etc/.kyinfo";
            let os_ver = doshell_out(command);
            if os_ver.chars().count() !=0 {
                let fname = "osver.ini";
                write_conf(&fname,&os_ver);
                res = read_conf(fname,"dist","milestone");
            }  else {
                res = "Not a kylin operating system".to_string();
            } 
        },
        //&_ => todo!(),
        _=> res = "err".to_string(),
    }
    return res.to_string();
}


