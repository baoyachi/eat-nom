# eat-nom

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](
https://github.com/baoyachi/eat-nom)
[![Cargo](https://img.shields.io/crates/v/eat-nom.svg)](
https://crates.io/crates/eat-nom)
[![Documentation](https://docs.rs/eat-nom/badge.svg)](
https://docs.rs/eat-nom)

eat nom based on [nom](https://github.com/Geal/nom). A tools about normal domain extend use nom

# 《玩转nom解析引擎》
* https://www.bilibili.com/video/BV1UK4y157BP



## parse ip

* parse_ip_mask
```rust
extern crate eat_nom;

use eat_nom::ip::parse_ip_mask;
use std::net::Ipv4Addr;

fn main() {
    let ip_mask = "127.0.0.1/255.0.255.0";
    let (ip, mask) = parse_ip_mask(ip_mask, "/").unwrap();
    assert_eq!(ip, Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!(mask, Ipv4Addr::new(255, 0, 255, 0));
}
```
*# parse_ip_mask_opt
```rust
extern crate eat_nom;

use eat_nom::ip::parse_ip_mask_opt;
use std::net::Ipv4Addr;

fn main() {
    let ip_mask = "127.0.0.1/255.0.255.0";
    let (ip, mask) = parse_ip_mask_opt(ip_mask).unwrap();
    assert_eq!(ip, Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!(mask, Ipv4Addr::new(255, 0, 255, 0));
}
```

* net
* time

