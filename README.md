# mini-vocabook

**mini-vocabook** 是一个基于 Rust 简易的 C/S 架构单词本桌面应用，并在组件库上采用 Vuetify3 的 Material Design 3 设计语言。

## 一、功能特性
1. 单词管理：添加、编辑、删除单词，包括单词、音标、释义和示例语句。
2. 单词本管理：创建、编辑、删除单词本，将单词加入指定单词本。
3. 学习记录：记录用户学习过的单词，包括熟练程度和学习时间。
4. 用户管理：支持多用户，每个用户可以选择自己的学习单词本。

## 二、技术栈
* 前端：Tauri、Vue.js、Vuetify
* 后端：Rust、Axum、SeaORM
* 数据库：MySql

## 三、快速开始

### Ⅰ、Axum后端服务
默认数据库使用 MySql，请根据需要更改配置文件 .env 中的数据库连接信息。

①在 MySql 中创建`mini_vocabook`数据库并运行 sql 语句：[mini_vocabook.sql]()；

②安装依赖：
~~~bash
cargo build
~~~

③运行应用：
~~~bash
cargo run
~~~
默认后端地址为 http://localhost:3030。

### Ⅱ、Tauri前端应用
①进入前端目录：
~~~bash
cd tauri-mini-vocabook
~~~

②安装依赖：
~~~bash
pnpm install
~~~

③运行应用：
~~~bash
pnpm tauri dev
~~~
前端默认占用1420端口
