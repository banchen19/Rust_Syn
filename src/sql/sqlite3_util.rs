use rusqlite::{params, Connection, Error, Result};

use crate::{
    shttp::http_player_config::Players,
    var_config::{
        def_Config::{Config, DefPlayer, EconomyInfo},
        yml_util::generate_random_key,
    },
};


// 创建表
pub fn create_sqlite3() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("sqlite3.db")?;
    conn.execute("PRAGMA foreign_keys = ON;", ())?;
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS `player` (
            `name` TEXT UNIQUE NOT NULL,
            `pw` TEXT NOT NULL,
            `level` INTEGER DEFAULT 0,
            `prefix` TEXT DEFAULT NULL,
            `online` INTEGER NOT NULL,
            `ip` TEXT DEFAULT NULL,
            `time` TEXT DEFAULT 0
        );
        "#,
        (),
    )?;

    // 在 `player` 表的 `name` 列上创建索引
    conn.execute(
        r#"
        CREATE INDEX IF NOT EXISTS idx_player_name
        ON `player` (`name`);
        "#,
        (),
    )?;

    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS economy_key (
            id INTEGER PRIMARY KEY,
            economy_name TEXT UNIQUE,
            key TEXT
        );
        "#,
        (),
    )?;

    // 添加外键约束并指定级联删除
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS economy (
            id INTEGER PRIMARY KEY,
            economy_name TEXT,
            player_name TEXT,
            money INTEGER,
            UNIQUE (economy_name, player_name),
            FOREIGN KEY (player_name)
            REFERENCES `player`(`name`)
            ON DELETE CASCADE
        );
        "#,
        (),
    )?;
    Ok(())
}

// 添加玩家数据
pub fn insert_player_sqlite3(config: Config, player: DefPlayer) -> Result<()> {
    let conn = Connection::open("sqlite3.db")?;
    conn.execute(
        "INSERT INTO player (
            name,
            pw,
            level,
            prefix,
            online,
            ip,
            time
        ) VALUES (
            ?1,
            ?2,
            ?3,
            ?4,
            ?5,
            ?6,
            ?7
        )",
        params![
            player.name,
            player.pw,
            player.level,
            player.prefix,
            player.online,
            player.ip,
            player.time,
        ],
    )?;
    let _ = create_pl_bind_balance_sqlite3(config.clone(), player.name, config.def_money_name);
    Ok(())
}

// 根据name查询玩家密码
pub fn getplayer_pw_name_sqlite3(name: &str) -> Result<String, Error> {
    let conn = Connection::open("sqlite3.db")?;
    let sql = "SELECT pw FROM player WHERE player.name = ?1";
    let pw: Result<String, _> = conn.query_row(sql, &[name], |row| row.get(0));
    pw
}

// 根据name查询玩家数据
pub fn getplayer_information_name_sqlite3(
    name: &str,
    economy_name: &str,
) -> Result<DefPlayer, Error> {
    let conn = Connection::open("sqlite3.db")?;
    let sql = "SELECT
    player.name,
    player.pw,
    IFNULL(economy.money, 0) AS money,
    player.level,
    player.prefix,
    player.online,
    player.ip,
    player.time
FROM player
LEFT JOIN economy ON player.name = economy.player_name
WHERE player.name = ?1 AND economy_name=?2";

    let player = conn.query_row(sql, &[name, economy_name], |row| {
        Ok(DefPlayer {
            name: row.get(0)?,
            pw: row.get(1)?,
            money: row.get(2)?,
            level: row.get(3)?,
            prefix: row.get(4)?,
            online: row.get(5)?,
            ip: row.get(6)?,
            time: row.get(7)?,
        })
    });
    player
}

// 删除玩家数据
pub fn delete_player_sqlite3_by_name(name: &str) -> Result<()> {
    let conn = Connection::open("sqlite3.db")?;
    conn.execute("DELETE FROM player WHERE name = ?1", &[name])?;
    Ok(())
}

// 查询所有玩家数据
pub fn getplayer_information_all_sqlite3(economy_name: String) -> Result<Option<Players>> {
    let conn = Connection::open("sqlite3.db")?;
    let sql = "SELECT
    player.name,
    player.pw,
    IFNULL(economy.money, 0) AS money,
    player.level,
    player.prefix,
    player.online,
    player.ip,
    player.time
FROM player
LEFT JOIN economy ON player.name = economy.player_name 
WHERE economy_name = ?1";

    let mut stmt = conn.prepare(sql)?;

    let player_iter = stmt.query_map([economy_name], |row| {
        Ok(DefPlayer {
            name: row.get(0)?,
            pw: row.get(1)?,
            money: row.get(2)?,
            level: row.get(3)?,
            prefix: row.get(4)?,
            online: row.get(5)?,
            ip: row.get(6)?,
            time: row.get(7)?,
        })
    })?;

    let mut players: Vec<DefPlayer> = Vec::new();

    for player in player_iter {
        match player {
            Ok(player) => players.push(player),
            Err(err) => return Err(err),
        }
    }

    if !players.is_empty() {
        Ok(Some(Players { players }))
    } else {
        Ok(None)
    }
}

// 一般玩家获取数据

// 查询所有玩家数据
pub fn getplayer_information_all_sqlite3_pl(economy_name: String) -> Result<Option<Players>> {
    let conn = Connection::open("sqlite3.db")?;
    let sql = "SELECT
    player.name,
    player.pw,
    IFNULL(economy.money, 0) AS money,
    player.level,
    player.prefix,
    player.online,
    player.ip,
    player.time
FROM player
LEFT JOIN economy ON player.name = economy.player_name 
WHERE economy_name = ?1";
    let mut stmt = conn.prepare(sql)?;

    let player_iter = stmt.query_map([economy_name], |row| {
        Ok(DefPlayer {
            name: row.get(0)?,
            pw: "*******".to_owned(),
            money: row.get(2)?,
            level: row.get(3)?,
            prefix: row.get(4)?,
            online: row.get(5)?,
            ip: row.get(6)?,
            time: row.get(7)?,
        })
    })?;

    let mut players: Vec<DefPlayer> = Vec::new();

    for player in player_iter {
        match player {
            Ok(player) => players.push(player),
            Err(err) => return Err(err),
        }
    }

    if !players.is_empty() {
        Ok(Some(Players { players }))
    } else {
        Ok(None)
    }
}
// 修改玩家权限
pub fn update_player_level_name_sqlite3(name: String, level: i32) -> Result<(), Error> {
    let conn = Connection::open("sqlite3.db")?;
    conn.execute(
        "UPDATE player SET
        level = ?1
         WHERE name = ?8",
        (level, name),
    )?;
    Ok(())
}

/*********************************************************************************************************************/
// 添加经济体
pub fn add_money_name_sqlite3(moneysName: String) -> Result<String> {
    let conn = Connection::open("sqlite3.db")?;
    let key = generate_random_key(8);
    conn.execute(
        "INSERT INTO economy_key (
            economy_name,
            key
        ) VALUES (
            ?1,
            ?2
        )",
        (moneysName, key.clone()),
    )?;
    Ok(key)
}

// 查询经济体密钥
pub fn getmoney_key_sqlite3(name: String) -> Result<String, Error> {
    let conn = Connection::open("sqlite3.db")?;
    let sql = "SELECT key FROM economy_key WHERE economy_name = ?1";
    let key: String = conn.query_row(sql, [name], |row| Ok(row.get(0)?))?;
    Ok(key)
}

// 删除经济体
pub fn delete_money_name_sqlite3(moneysName: String, key: String) -> Result<()> {
    let conn = Connection::open("sqlite3.db")?;
    conn.execute(
        "DELETE FROM economy_key (
            economy_name,
            key
        ) VALUES (
            ?1,
            ?2
        )",
        (moneysName, generate_random_key(8)),
    )?;

    Ok(())
}

// 删除玩家经济信息
pub fn delete_plmoney_name_sqlite3(player_name: String) -> Result<()> {
    let conn = Connection::open("sqlite3.db")?;
    conn.execute(
        "DELETE FROM economy WHERE player_name = ?1",
        params![player_name],
    )?;

    Ok(())
}

//为玩家添加经济体-初始化
pub fn create_pl_bind_balance_sqlite3(
    config: Config,
    player_name: String,
    economy_name: String,
) -> Result<()> {
    let conn = Connection::open("sqlite3.db")?;
    conn.execute(
        "INSERT OR IGNORE INTO economy (
            economy_name,
            player_name,
            money
        ) VALUES (
            ?1,
            ?2,
            ?3
        )",
        (economy_name, player_name, config.def_money_number),
    )?;
    Ok(())
}

// 修改玩家对应的经济余额
pub fn update_player_money_sqlite3(
    newBalance: i64,
    moneysName: String,
    player_name: String,
) -> Result<(), Error> {
    let conn = Connection::open("sqlite3.db")?;
    conn.execute(
        "UPDATE economy SET
        money = ?1,
         money = ?2,
         player_name = ?3",
        (newBalance, moneysName, player_name),
    )?;
    Ok(())
}


// 查询所有经济体
pub fn getmoney_name_sqlite3() -> Result<Vec<EconomyInfo>, Error> {
    let mut conn = Connection::open("sqlite3.db")?;

    let mut stmt = conn.prepare(
        "SELECT
        economy_key.economy_name,
        economy_key.key
    FROM
        economy_key",
    )?;

    let economy_iter = stmt.query_map((), |row| {
        Ok(EconomyInfo {
            economy_name: row.get(0)?,
            key: row.get(1)?,
        })
    })?;

    let mut economy_info_list: Vec<EconomyInfo> = Vec::new();
    for economy_info in economy_iter {
        economy_info_list.push(economy_info?);
    }

    Ok(economy_info_list)
}


// 查询所有经济体——pl
pub fn getmoney_name_sqlite3_pl() -> Result<Vec<EconomyInfo>, Error> {
    let mut conn = Connection::open("sqlite3.db")?;

    let mut stmt = conn.prepare(
        "SELECT
        economy_key.economy_name,
        economy_key.key
    FROM
        economy_key",
    )?;

    let economy_iter = stmt.query_map((), |row| {
        Ok(EconomyInfo {
            economy_name: row.get(0)?,
            key: "********".to_owned(),
        })
    })?;

    let mut economy_info_list: Vec<EconomyInfo> = Vec::new();
    for economy_info in economy_iter {
        economy_info_list.push(economy_info?);
    }

    Ok(economy_info_list)
}