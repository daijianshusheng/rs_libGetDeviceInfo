// Copyright (c) 2023-2024 Liu Qiang <liulcsy@qq.com> In Wuhan, China
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::fs;
use std::time::SystemTime;
use chrono::{TimeZone, Utc};
use chrono::FixedOffset;
use std::path::Path;
use crate::pubunit;

//check anti-virus lib
pub fn get_check_antivirus(osn:&str) -> String{
    //println!("check_antivirus");
    let mut res = "".to_string();
    match osn {
        "topsec" =>{
            let file_path = "/opt/apps/cn.com.topsec.edragent/files/database/topsec.db";
            match fs::metadata(file_path) {
                Ok(meta) => {
                    if meta.is_file() {
                        let t = SystemTime::from(meta.modified().unwrap());
                        res = time_to_beijing(&t);
                    } else {
                        println!("Not a valid file.");
                    }
                },
                Err(_) => {
                    //println!("Failed to get virus lib for the file.");
                    res = "Failed to get virus lib for the file.".to_string();
                }
            };
        }
        _=> res = "err".to_string(),
    }

    return res.to_string();
}

//directory search
pub fn get_dir_search(_path:&str) -> Vec<String>{
    let mut files = vec![];
    let path = Path::new(&_path);
    let Ok(_) = pubunit::visit_dirs(path, &|entry, files| {
        //println!("{:?}", entry);
        files.push(entry);
    },&mut files) else {
        eprintln!("Failed to read the directory!");
        return vec![];
    };
    return files.iter()
    .map(|it| it.path().to_string_lossy().to_string().replacen(_path, "", 1))
    .collect::<Vec<String>>();
}

pub fn time_to_beijing(st:&SystemTime)-> String{
    let t = st.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let dt = Utc.timestamp_opt(t.try_into().unwrap(), 0).unwrap();
    let china_timezone = FixedOffset::east_opt(8 * 3600);
    let chinatime = dt.with_timezone(&china_timezone.unwrap());      
    let res = chinatime.format("%Y-%m-%d %H:%M:%S");        
    return res.to_string();
}