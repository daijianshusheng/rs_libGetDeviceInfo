# get_local_info [![Crates.io](https://img.shields.io/crates/v/get_local_info.svg)](https://crates.io/crates/get_local_info) ![License](https://img.shields.io/crates/l/get_local_info.svg) [![Documentation](https://docs.rs/get_local_info/badge.svg)](https://docs.rs/get_local_info/)

# Project Introduction

* get_local_info is a Rust crate that obtains linux local information,Its goal is to solve the difficulty of obtaining Linux local information.Support Chinese operating systems such as KyLin, UOS, HarmonyOS, etc
* Project maintenance:  long-term 

# Current features:
1.Network function

1.1 Obtain network interface information

1.2 Obtain activity network card information:
* Netcard name
* IPv4
* IPv6
* mac
* Obtain external IPv4 address
* Domain name resolution

2.Get system version
* Kylin10 supports 2017 and above
* Ubuntu supports 18.04 and above
* UOS20 supports 1020 and above

3.System function
* Process detection 
* Virtual Machine Detection
* Dual system detection

4.Security information detection
* topsec Antivirus Database Date Detection

5.File Info
* Traverse all files in the specified directory

extra features
* find_peak_element

# How to use
1.cargo add get_local_info

2.Import into your project

```
extern crate get_local_info;

fn main() {
    println!("{}", get_local_info::get_pc_net_card_name());
    println!("{}", get_local_info::get_pc_ipv4());
    println!("{}", get_local_info::get_pc_ipv6());
    println!("{}", get_local_info::get_pc_mac());
    println!("{:?}", get_local_info::get_pc_net_card_info());
    // osname: ubuntu or uos or kylin
    let osname = "uos";
    println!("{}", get_local_info::get_pc_system_ver(osname));
    //check proccess name
    let pname = "gnome";
    println!("{}", get_local_info::get_pc_system_check_pname(pname));
    // flase is Real machine, true is vm
    println!("Running in VM:{}", get_local_info::get_pc_system_is_vm());
     // true is multi os
    println!("multi os:{}", get_local_info::get_pc_system_is_d_sys());
    // Obtain the update time of the antivirus database
    let antiname = "topsec";
    println!("virus lib time:{}", get_local_info::get_pc_check_antiviruslib(antiname));
    //find_peak_element
    let nums = [1];
    println!("{}", get_local_info::alg::find_peak_element(nums.to_vec()));
    //Traverse all files in the specified directory
    let idir = "/opt";
    println!("files:{:?}", get_local_info::get_dir_filename(idir));
    let uri = "https://api.ipify.org/?format=text";
    println!("wai wang ip:{}", get_local_info::network::get_ww_ipv4(uri));
    let hostname = "www.baidu.com";
    let parse_ip = get_local_info::network::get_domain_ip(hostname);
    println!("domain parse_ip:{:?}",parse_ip);
}
```

# About the Author
* Liu Qiang in Wuhan, China
* crates: <https://crates.io/crates/get_local_info>
* github: <https://github.com/daijianshusheng/rs_libGetDeviceInfo>
* mail: <liulcsy@qq.com>
* Created on 2023.12.28
