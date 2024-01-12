// Copyright (c) 2023-2024 Liu Qiang <liulcsy@qq.com> In Wuhan, China
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate ini;
use ini::Ini;
use std::fs;
use std::process::Command;
use std::io::Write;
use std::fs::DirEntry;
use std::path::Path;
use std::io;


pub fn doshell_out(ml:&str) -> String{
    let output = Command::new("sh").arg("-c").arg(ml.to_string()).output().expect("err");
    let res = String::from_utf8(output.stdout).unwrap();
    return res.to_string();
}

pub fn write_conf(fname:&str,fcontent:&str){
    let mut file = std::fs::File::create(fname).expect("create failed");
    file.write_all(fcontent.as_bytes()).expect("write failed");
}

pub fn read_conf(fname:&str,section:&str,key:&str)-> String{
    let conf = Ini::load_from_file(fname).unwrap();
    let section = conf.section(Some(section)).unwrap();
    let res = section.get(key).unwrap();
    return res.to_string();
}

pub fn visit_dirs(dir: &Path, cb: &dyn Fn(DirEntry,&mut Vec<DirEntry>),vec: &mut Vec<DirEntry>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb, vec)?;
            } else {
                //println!("file => {}", entry.path().to_string_lossy());
                cb(entry,vec);
            }
        }
    }
    Ok(())
}
