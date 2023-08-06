use serde::{Serialize, Deserialize};


// 异步
use std::{
    error::Error,
    fs,
    sync::{Arc, Mutex},
    thread,
};

// 有色日志
use colored::Colorize;

use crate::{var_config::yml_util, sws::ws_key::generate_md5_key};

// 启动配置文件结构体
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Config{
    pub(crate) ws_port:u32,
    pub(crate) ws_keymode:String,
    pub(crate) server_name:String,
    pub(crate) http_port:u32,
    pub(crate) sqlmode:String,
    pub(crate) def_money_name:String,
    pub(crate) def_money_number:u64,
    pub(crate) whitelist:bool,
    pub(crate) delplayer:i32,
    pub(crate) addplayer:i32,
    pub(crate) delplme:bool,
    pub(crate) getplall:i32,

}

// 玩家默认数据
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct DefPlayer {
    pub name: String,
    pub pw: String,
    pub level: i32,
    pub prefix: String,
    pub online: u8,
    pub ip: String,
    pub time:String,
    pub money: i32,

}

use mysql::Error as MySQLError;
use rusqlite::Error as SQLiteError;

use super::yml_util::generate_random_key;
pub enum DatabaseError{
    MySQL(MySQLError),
    SQLite(SQLiteError),
}

// 返回配置
pub fn inti_config() -> Result<Config, Box<dyn Error>> {
    let key=generate_random_key(16).to_owned();
    let file_path = "config.yml";
    let config = crate::Config {
        ws_port:20102,
        http_port:20103,
        sqlmode:"sqlite3".to_owned(),
        def_money_name:"贡献点".to_owned(),
        def_money_number:0,
        whitelist:true,
        delplayer:4,
        addplayer:4,
        delplme:false,
        getplall:4,
        ws_keymode: "AES-128".to_owned(),
        server_name:key,
    };
    match fs::metadata(&file_path) {
        Err(_) => {
            let text = "文件不存在, 开始写入".to_string();
            println!("{}", text.yellow());
            if let Err(err) = yml_util::write_config_to_yml(&config, &file_path) {
                println!("无法写入配置文件：{}", err);
            }
        }
        Ok(_) => {
            let text = "检测到配置文件存在".to_string();
            println!("{}", text.green());
        }
    }
    read_yml_to_str(&file_path)
}

// 读取配置文件
pub fn read_yml_to_str(file_path: &str) -> Result<Config, Box<dyn Error>> {
    let config = yml_util::read_yml(file_path)?;
    Ok(config)
}
