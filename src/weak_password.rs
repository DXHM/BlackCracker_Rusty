use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn check(target: &str) {
    let weak_passwords_path = "weak_passwords.txt";  // 弱密码列表文件路径，根据你的实际情况进行修改

    // 打开弱密码列表文件
    let file = File::open(weak_passwords_path).expect("Failed to open weak passwords file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let weak_password = line.expect("Failed to read line from weak passwords file");
        if weak_password.trim() == target {
            println!("Weak password detected: {}", target);
            return;
        }
    }

    println!("Password is not weak");
}
