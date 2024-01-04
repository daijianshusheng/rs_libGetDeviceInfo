# get_local_info [![Crates.io](https://img.shields.io/crates/v/get_local_info.svg)](https://crates.io/crates/get_local_info) ![License](https://img.shields.io/crates/l/get_local_info.svg) [![Documentation](https://docs.rs/get_local_info/badge.svg)](https://docs.rs/get_local_info/)

# Project Introduction

* get_local_info is a Rust crate that obtains linux device information,Its goal is to solve the difficulty of obtaining Linux local information.Support Chinese operating systems such as KyLin, UOS, HarmonyOS, etc
* Current version:  0.1.5
* Project maintenance:  long-term 

# Current features:
1.Network function
1.Obtain activity network card information:
* Netcard name
* IPv4
* IPv6
* mac
1.2 Obtain network interface information

2.Get system version
* Kylin10 supports 2017 and above
* Ubuntu supports 18.04 and above
* UOS20 supports 1020 and above

3.System function
* Process detection 
* Virtual Machine Detection

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
    let osname = "uos";
    println!("{}", get_local_info::get_pc_system_ver(osname));
    let pname = "gnome";
    println!("{}", get_local_info::get_pc_system_check_pname(pname));
}
```

# About the Author
* Liu Qiang in Wuhan, China
* crates: <https://crates.io/crates/get_local_info>
* github: <https://github.com/daijianshusheng/rs_libGetDeviceInfo>
* mail: <liulcsy@qq.com>
* create time：2023.12.28
