# BlackCracker_Rusty

BlackCracker_Rusty 是一个使用 Rust 编写的密码破解工具。它提供了一些基本的密码破解功能，包括字典攻击、弱密码检测和哈希破解。

## 功能特性

- 字典攻击：通过使用自定义的字典文件对目标进行密码破解。
- 弱密码检测：检查目标是否使用了常见的弱密码。
- 哈希破解：尝试破解常见哈希算法的哈希值。

## 使用方法

运行 BlackCracker_Rusty 需要以下命令行参数：

cargo run -- <mode> <target>

- `<mode>`: 模式选项，可选的值包括 "dictionary"、"weak_password" 和 "hash_cracker"。
- `<target>`: 目标选项，表示你要对其进行密码破解或检测的目标。

示例：
cargo run -- dictionary example_target.txt
cargo run -- weak_password example_target.txt
cargo run -- hash_cracker example_target_hash


请确保你已经按照项目的需求准备好字典文件和哈希值。

## 测试

BlackCracker_Rusty 包含了一些基本的单元测试。可以使用以下命令运行测试：

cargo test

## 许可证

BlackCracker_Rusty 使用 [MIT 许可证](LICENSE)。

## 贡献

欢迎对 BlackCracker_Rusty 进行贡献！如果你发现了 bug，或者有任何改进建议，请提出 issue 或提交 pull 请求。

