use crypto::digest::Digest;
use crypto::md5::Md5;
pub fn crack(target_hash: &str) {
    let mut hasher = Md5::new();

    // 遍历可能的密码
    for password in generate_passwords() {
        hasher.input_str(&password);

        let result = hasher.result_str();
        if result == target_hash {
            println!("Password cracked: {}", password);
            return;
        }

        hasher.reset();
    }

    println!("Failed to crack the password");
}



fn generate_passwords() -> Vec<String> {
    // 生成可能的密码列表
    // 这里可以根据你的需求生成密码，例如使用不同的字符集、密码长度等
    vec![
        "password123".to_string(),
        "admin".to_string(),
        // 更多密码...
    ]
}
