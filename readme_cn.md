# Black Cracker Rusty

最近沉迷于玩Rust，想着用Rust随手写个Web或密码学的工具。

此项目暂属于半成品，先立个项，有时间了再慢慢研究完善😁……

欢迎各界大佬贡献智慧，求star，有什么想法欢迎issue讨论，期待能在致谢中能添上您的头像

Love and Peace ！


> 此项目是基于 Rust 编写的密码破解框架。

> 集成多种的密码破解功能，包括字典攻击、弱口令检测和哈希破解等功能。
 

> 未完待续...

## 项目仓库

Github：[BlackCracker_Rusty](https://github.com/DXHM/BlackCracker_Rusty)

发布版：[Release](https://github.com/dxhm/BlackCracker_Rusty/releases/latest)

## 功能

- **字典攻击模式**：通过遍历字典文件中的密码，尝试破解目标密码。
- **弱口令检测模式**：检测目标密码是否为弱口令，进行密码安全性评估。
- **哈希破解模式**：通过破解密码的哈希值，尝试还原原始密码。

## 安装

1. 将项目克隆到本地：

   ```bash
   git clone https://github.com/DXHM/BlackCracker_Rusty.git
   ```

2. 进入项目目录：

   ```bash
   cd Blackcracker_rusty
   ```

3. 构建项目：

   ```bash
   cargo build --release
   ```

4. 运行项目：

   ```bash
   cargo run -- <mode> <target>
   ```

## 用法
   - `<mode>`：选择要运行的模式，可以是 `dictionary`、`weak_password` 或 `hash_cracker`。
   - `<target>`：目标密码或哈希值。

## 示例

### Linux

- 字典攻击模式：

  ```bash
  blackcracker_rusty dictionary password123
  ```

- 弱密码检测模式：

  ```bash
  blackcracker_rusty weak_password user1
  ```

- 哈希破解模式：

  ```bash
  blackcracker_rusty hash_cracker 5f4dcc3b5aa765d61d8327deb882cf99
  ```

### Windows

- 字典攻击模式：

  ```bash
  blackcracker_rusty.exe dictionary password123
  ```

- 弱密码检测模式：

  ```bash
  blackcracker_rusty.exe weak_password user1
  ```

- 哈希破解模式：

  ```bash
  blackcracker_rusty.exe hash_cracker 5f4dcc3b5aa765d61d8327deb882cf99
  ```

## 依赖

- rust-crypto = "^0.2"
- embed-resource="^2.0"

## 贡献者

[<img alt="AShujiao" src="https://avatars.githubusercontent.com/u/69539047?v=4" width="117">](https://github.com/dxhm)

## 许可

---