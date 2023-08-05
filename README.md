# MC同步器项目文档

### `添加玩家初始化数据`
请求者权限高于权限要求可
- 请求方式：POST
- 参数：
  - 用户数据（JSON）：
    - name：玩家名称
    - pw：玩家密码
    - player: 玩家初始化数据
    - 请求头：`"Content-Type: application/json"`
- 请求体参数：
```json
{
    "name": "banchen21",
    "pw": "密码",
    "player":{
        "name": "JohnDoe",//玩家名字
        "pw": "secretpassword",//设定这个玩家的默认密码
        "level": 5,//等级必须比上传者的等级低
        "prefix": "string",//前缀，不重要，默认随着发送端变化
        "online": 0,//默认在线状态，不重要，玩家上线后将会更新
        "ip": "192.168.1.100",//IP，不重要
        "time": "2023-08-01 12:34:56"//最后的上线时间，不重要
        }
}
```
- 响应：
  - 200 OK：成功
    - 响应体：`{"code": 200,"message": "true"}`
  - 403 Forbidden：失败
    - 响应体：`{"code":403,"message": "false"}`
---
### `删除玩家数据`
请求者权限高于权限要求可
- 请求方式：GET
- 参数：
  - 用户数据（JSON）：
    - name：玩家名称
    - pw：玩家密码
    - pl_name：被操作者的名字
- 请求体路径：
```
/deleteplayer/?<name>&<pw>&<pl_name>
```
- 响应：
  - 200 OK：成功
    - 响应体：`{"code": 200,"message": "true"}`
  - 403 Forbidden：失败
    - 响应体：`{"code":403,"message": "false"}`
---
### `玩家自主注销`
配置文件开启对应权限可实现
- 请求方式：GET
- 参数：
  - 用户数据（JSON）：
    - name：玩家名称
    - pw：玩家密码
- 请求体路径：
```
/deleteplayer_me/?<name>&<pw>
```
- 响应：
  - 200 OK：成功
    - 响应体：`{"code": 200,"message": "true"}`
  - 403 Forbidden：失败
    - 响应体：`{"code":403,"message": "false"}`
---
### `获取数据库内所有玩家`
配置文件开启对应权限可实现
- 请求方式：GET
- 参数：
  - 用户数据（JSON）：
    - name：玩家名称
    - pw：玩家密码
    - moeny_name：经济体
- 请求体路径：
```
/getinformation_all/?<name>&<pw>&<moeny_name>
```
- 响应：
  - 200 OK：成功
    - 响应体：
    ```json
    {
      "code": 200,
      "message": {
          "players": [
              {
                "money":111,
                {
                "name": "JohnDoe",
                "pw": "secretpassword",
                "level": 5,
                "prefix": "string",
                "online": 0,
                "ip": "192.168.1.100",
                "time": "2023-08-01 12:34:56"
                }
              },
              {
                "money":222,
                {
                "name": "JohnDoe2",
                "pw": "secretpassword",
                "level": 5,
                "prefix": "string",
                "online": 0,
                "ip": "192.168.1.100",
                "time": "2023-08-01 12:34:56"
                }
              }
          ]
      }
      }
    ```
  - 403 Forbidden：失败
    - 响应体：`{"code":403,"message": "false"}`
---

### `登录以获取个人信息`
配置文件开启对应权限可实现
- 请求方式：GET
- 参数：
  - 用户数据（JSON）：
    - name：玩家名称
    - pw：玩家密码
    - moeny_name：经济体
- 请求体路径：
```
/getlogin/?<name>&<pw>&<moeny_name>
```
- 响应：
  - 200 OK：成功
    - 响应体：
    ```json
    {
      "code": 200,
      "message": 
      {
        "money":111,
        {
        "name": "JohnDoe",
        "pw": "secretpassword",
        "level": 5,
        "prefix": "string",
        "online": 0,
        "ip": "192.168.1.100",
        "time": "2023-0812:34:56"
        }
        },
      }
    ```
  - 403 Forbidden：失败
    - 响应体：`{"code":403,"message": "false"}`
---
### `删除经济体`
配置文件开启对应权限可实现
- 请求方式：GET
- 参数：
  - 用户数据（JSON）：
    - moneysName：经济名
    - key：经济体密钥
    - 请求体路径：
```
/delmoney/?<moneysName>&<key>
```
- 响应：
  - 200 OK：成功
    - 响应体：`{"code":200,"message": "true"}`
  - 403 Forbidden：失败
    - 响应体：`{"code":403,"message": "false"}`
---
### `获取玩家余额`
配置文件开启对应权限可实现
- 请求方式：GET
- 参数：
  - 用户数据（JSON）：
    - moneysName：经济名
    - key：经济体密钥
    - 请求体路径：
```
/getplmoney/?<name>&<moeny_name>
```
- 响应：
  - 200 OK：成功
    - 响应体：`{"code":200,"message": "true"}`
  - 403 Forbidden：失败
    - 响应体：`{"code":403,"message": "false"}`
---
## WS端接口说明
### Chat通信
#### 客户端消息发送格式要求
```json
{
    "type":"chat",
    "data":"{
        "name":"玩家名",
        "sender":"来源于谁？",
        "msg":"要广播到所有子服的消息"
    }"
}
```
#### 消息接受格式
 ```json
{
  "data":{
    "player_name":"string",
    "perm_level":0,
    "data":"chat_msg",
    "serverver_name":"玩家进行游戏服务服务端名"
  },
  "type":"chat",
}
```
---
## 其他说明
 -POST 请求格式
 - JSON

-GET 请求格式
 - 参数
 - 示例
```
http://127.0.0.1:8082/getpermissions?name=banchen21
```