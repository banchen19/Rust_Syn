use std::{collections::HashMap, fs::{File, OpenOptions}};

use crate::yml_util::generate_random_key;


pub fn getpermkeys(key: &str) -> String {
    let file_path_s = "perm_data.json";
    let file: File = match OpenOptions::new().read(true).write(true).create(true).open(&file_path_s) {
        Ok(file) => file,
        Err(e) => panic!("打开文件失败: {:?}", e),
    };
    // 随机生成十六位密钥
    let mut matching_values = generate_random_key(16);

    // 解析 JSON 数据并读取文件
    let data: HashMap<String, String> = match serde_json::from_reader(file) {
        Ok(data) => data,
        Err(e) => panic!("解析 JSON 失败: {:?}", e),
    };

    for (k, v) in &data {
        if key == k {
            // 提供密钥与json文件内的数据一致
            matching_values = v.to_string();
        }
    }
    matching_values
}
