[‼️]: ✏️README.mdt

# alive : 极简主义的监控 & 状态页 (无需数据库即可部署)

[演示网站](https://status.i18n.site)

## 截图演示

### 桌面

![](https://i-01.eu.org/1712928379.webp)

### 手机

<img src="https://i-01.eu.org/1713321140.webp" width="300">

## 设计特性

无需数据库即可部署

监控和报警都插件化

`./conf/plugin.yml` 可以用相对路径引用外部插件

## 设计初衷

监控有很多复杂的逻辑 , 比如配置 orchestrator 自动切换主从的 mysql 高可用 , 那么应该监控 mysql 主从拓扑是不是正确 , 有没有脑裂 。

监控和自愈应该是一体的 , 比如发现 IP 挂了 , 应该去屏蔽 cloudflare 上解析的 ip , 发现恢复了 , 应该去启用 ip 。

现在有的方案想实现我的这些需求 , 都太重了。

干脆自己写一个 , 所有的监控都插件化 , 按需启用

### 免费部署

用下面的方案可以免费部署

后端部署到 fly.io

#### 前端部署到 cloudflare

##### 绑定自动代码库

![](https://i-01.eu.org/1712834002.webp)

<img src="https://i-01.eu.org/1712834225.webp" width="500">

##### 绑定域名

![](https://i-01.eu.org/1712834657.webp)

## 本地运行

运行 `./sh/conf_init.sh` 初始化配置

修改 `./conf/plugin.yml` 启用插件

运行 `./plugin.sh` 生成 rust 代码

`./dev.sh srv` 启动后端 , `./srv/ssl/up.sh` 启动本地 https 代理

`./htm/dev.sh` 启动前端

## 代码结构

0. `./doc.sh` 生成文档 (比如 ,`./doc.sh alive_api`)
0. `./htm` 前端
0. [`./alive_api/api.proto`](./alive_api/api.proto) 后端返回数据是 protobuf 格式
0. `./srv` 后端
0. `./srv/ssl/up.sh` 后端的本地 https 代理 (开了才能配合前端调试)
0. `./watch` 监控插件
0. `./new.watch.sh xxx` 新建监控插件
0. `./alter` 报警插件
0. `./new.alter.sh xxx` 新建报警插件
0. `./sh/conf_example.sh` 从实际的配置导出演示的配置文件

## 后续计划

后续计划暂无排期 , 只是备忘

- 基于报警插件 , 可以对接后端数据库 , 持久化报警日志
- 现在后端返回的数据有 `runed`, `cost_sum` , `avg10` , 可以用展示监控服务访问延时的变化 , 但是前端没做展示 (可以在服务延时异常的时候显示前端警告)
- 貌似有一些内存泄露 , 需要排查 , 参考
[记一次 Rust 内存泄漏排查之旅 | 经验总结篇](https://xie.infoq.cn/article/ac333e916b4594627ff322463)
[蚂蚁集团 ｜ 如何在生产环境排查 Rust 内存占用过高问题](https://rustmagazine.github.io/rust_magazine_2021/chapter_5/rust-memory-troubleshootting.html)
