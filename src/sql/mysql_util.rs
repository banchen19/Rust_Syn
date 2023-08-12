
use mysql::{params, prelude::*};
use rand::Error;



// 创建表
pub fn create_mysql(pool: &mysql::Pool) -> mysql::Result<()> {
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r#"
        CREATE TABLE IF NOT EXISTS `player` (
            `name` varchar(255) NOT NULL COMMENT '名字',
            `pw` varchar(255) NOT NULL COMMENT '密码',
            `level` int(50) NOT NULL,
            `prefix` varchar(255) NOT NULL,
            `online` int(2) NOT NULL,
            `ip` varchar(255) NOT NULL,
            `time` varchar(255) NOT NULL
          ) ENGINE=MyISAM DEFAULT CHARSET=utf8;
        "#,
    )?;
    conn.query_drop(
        r#"
        CREATE TABLE IF NOT EXISTS economy_key (
            id INTEGER PRIMARY KEY,
            economy_name TEXT UNIQUE,
            key TEXT,
            FOREIGN KEY (economy_name) REFERENCES economy(economy_name)
        );
        "#,
    )?;
    conn.query_drop(
        r#"
        CREATE TABLE IF NOT EXISTS economy (
            id INTEGER PRIMARY KEY,
            economy_name TEXT,
            player_name TEXT,
            balance INTEGER,
            UNIQUE (economy_name, player_name)
        );
        "#,
    )?;
    Ok(())
}
