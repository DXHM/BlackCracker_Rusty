use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn attack(target: &str) {
    let dictionary_path = "dictionary.txt";  // 字典文件路径

    // 打开字典文件
    let file = File::open(dictionary_path).expect("Failed to open dictionary file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let password = line.expect("Failed to read line from dictionary");
        if password.trim() == target {
            println!("Password found: {}", password);
            return;
        }
    }

    println!("Password not found");
}
