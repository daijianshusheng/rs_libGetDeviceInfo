# Project Introduction

* name:Getlocalinfo is a Rust crate that obtains device information;
* create time：2023.12.28；
* os：linux,Including Chinese operating systems.For example:KyLin,UOS；

# Current features:
Obtain activity network card information:
Netcard
IPv4
IPv6
mac

# How to use
1.cargo add GetLocalInfo

2.Import into your project

extern crate get_local_info;
```
extern crate get_local_info;

fn main() {

    println!("{}", get_local_info::get_pc_net_card_name());
    println!("{}", get_local_info::get_pc_ipv4());
    println!("{}", get_local_info::get_pc_ipv6());
    println!("your mac:{}", get_local_info::get_pc_mac());
}
```

# About the Author
* Liu Qiang in Wuhan, China
* liuqiang <liulcsy@qq.com>
