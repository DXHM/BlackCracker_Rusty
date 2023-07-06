# Black Cracker Rusty

## 项目介绍

一个使用 Rust 编写的密码破解框架，集成了多种的密码破解功能，包括字典攻击、弱口令检测和哈希破解。

## 功能

- **字典攻击模式**：通过遍历字典文件中的密码，尝试破解目标密码。
- **弱口令检测模式**：检测目标密码是否为弱口令，进行口令安全性评估。
- **哈希破解模式**：通过破解密码哈希值，尝试还原原始密码。

## 安装

1. 克隆项目到本地：

   ```bash
   git clone https://github.com/dxhm/Blackcracker_rusty.git
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

   - `<mode>`：选择要运行的模式，可以是 `dictionary`、`weak_password` 或 `hash_cracker`。
   - `<target>`：目标密码或哈希值。

## 使用示例

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

## 依赖



## 贡献



## 许可证



---

