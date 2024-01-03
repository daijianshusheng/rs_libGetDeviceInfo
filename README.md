# get_local_info [![Crates.io](https://img.shields.io/crates/v/get_local_info.svg)](https://crates.io/crates/get_local_info) ![License](https://img.shields.io/crates/l/get_local_info.svg) [![Documentation](https://docs.rs/get_local_info/badge.svg)](https://docs.rs/get_local_info/)

# Project Introduction

* get_local_info is a Rust crate that obtains linux device information,Its goal is to solve the difficulty of obtaining Linux local information.Support Chinese operating systems such as KyLin, UOS, HarmonyOS, etc

	Project maintenance: long-term

# Current features:
1.Obtain activity network card information:
* Netcard
* IPv4
* IPv6
* mac

2.Obtain network interface information

3.Get system version
* Kylin10 supports 2017 and above
* Ubuntu supports 18.04 and above
* UOS20 supports 1020 and above

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
    //Obtain network interface information
    println!("{:?}", get_local_info::get_pc_net_card_info());
    //osname:ubuntu or uos or kylin
    let osname = "uos";  
    println!("{}", get_local_info::get_pc_system_ver(osname));
}
```

# About the Author
* Liu Qiang in Wuhan, China
* crates: <https://crates.io/crates/get_local_info>
* github: <https://github.com/daijianshusheng/rs_libGetDeviceInfo>
* create time：2023.12.28
