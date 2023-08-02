use serde::{Serialize, Deserialize};


// 启动配置文件结构体
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Config{
    pub(crate) database_host:String,
    pub(crate) database_port:u32,
    pub(crate) database_username:String,
    pub(crate) database_password:String,
    pub(crate) database_dataname:String,
    pub(crate) ws_port:u32,
    pub(crate) ws_key:String,
    pub(crate) http_port:u32,
    pub(crate) sql_mode:String,
    pub(crate) money_name:String,
    pub(crate) def_money:u64,
}

use mysql::Error as MySQLError;
use rusqlite::Error as SQLiteError;
pub enum DatabaseError {
    MySQL(MySQLError),
    SQLite(SQLiteError),
}

//添加玩家白名单
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct AddPlayer
{
    pub(crate) key:String,
    pub(crate) v:String,
    pub(crate) player:Player,
}

// 玩家默认数据
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Player {
    pub xuid: String,
    pub name: String,
    pub psw: String,
    pub money: i32,
    pub ip: String,
    pub online: u8,
    pub device: String,
    pub perm_level:u8,
    pub server_name: String,
    pub data:String,
}