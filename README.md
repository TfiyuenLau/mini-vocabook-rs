# mini-vocabook-rs

**mini-vocabook-rs** 是一个基于 Rust 和 Tauri 的简易 C/S 架构单词本桌面应用，其在后端服务器采用 Rust Axum 框架，在客户端组件库上采用
Vuetify3 及其 Material Design 3 设计语言。

[在此处](https://github.com/TfiyuenLau/mini-vocabook-rs/releases)下载对应平台的客户端和服务端程序。

## 一、功能特性

1. 每日打卡：逐步学习目标单词本的相关知识，包括单词、音标、读音、释义和示例语句。
2. 单词测验：通过单项选择和填空等题型考察对单词的释义和拼写的掌握程度。
3. 学习记录：对用户学习过的单词进行记录统计，包括熟练程度和学习时间等。
4. 用户管理：支持多用户，每个用户可以选择自己的学习单词本。

## 二、技术栈

* 前端：Tauri、Vue.js、Vuetify
* 后端：Rust、Axum、SeaORM
* 数据库：MySql

# 三、客户端界面

![Views](assets/screenshots/result.jpeg)

## 四、快速开始

### Ⅰ、Axum后端服务

默认数据库使用 MySql，请根据需要更改配置文件 .env 中的数据库连接信息。

①在 MySql 中创建`mini_vocabook`数据库并运行 assets/db 文件夹下的 sql 文件；

②安装依赖：

~~~bash
cargo build
~~~

③运行应用：

~~~bash
cargo run
~~~

后端默认地址为 http://localhost:3030。

### Ⅱ、Tauri客户端应用

①进入客户端项目目录：

~~~bash
cd app
~~~

②安装依赖：

~~~bash
pnpm install
~~~

③修改 tauri 项目下的 `http.ts` 文件，将 `server` 变量更改为指定的后端地址，例如：
~~~ts
// 服务器地址
const server = 'http://localhost:3030';
~~~

④调试应用：

~~~bash
cargo tauri dev
~~~

前端应用默认占用 1420 端口。
