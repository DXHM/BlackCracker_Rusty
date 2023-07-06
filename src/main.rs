mod dictionary;
mod weak_password;
mod hash_cracker;

use std::env;

fn main() {
    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    
    
        let author = "
    ██████╗ ██╗   ██╗██████╗ ███████╗    ██████╗ ██╗      █████╗  ██████╗██╗  ██╗
    ██╔══██╗██║   ██║██╔══██╗██╔════╝    ██╔══██╗██║     ██╔══██╗██╔════╝██║ ██╔╝
    ██████╔╝██║   ██║██████╔╝█████╗      ██████╔╝██║     ███████║██║     █████╔╝ 
    ██╔═══╝ ██║   ██║██╔══██╗██╔══╝      ██╔══██╗██║     ██╔══██║██║     ██╔═██╗ 
    ██║     ╚██████╔╝██║  ██║███████╗    ██████╔╝███████╗██║  ██║╚██████╗██║  ██╗
    ╚═╝      ╚═════╝ ╚═╝  ╚═╝╚══════╝    ╚═════╝ ╚══════╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝
    ";
    
        let logo = "
        ____   __              __      ______                    __              
       / __ ) / /____ _ _____ / /__   / ____/_____ ____ _ _____ / /__ ___   _____
      / __  |/ // __ `// ___// //_/  / /    / ___// __ `// ___// //_// _ \\ / ___/
     / /_/ // // /_/ // /__ / ,<    / /___ / /   / /_/ // /__ / ,<  /  __// /    
    /_____/ /_/ \\__,_/ \\___//_/|_|   \\____//_/    \\__,_/ \\___//_/|_| \\___//_/     
                                                                                 
               ______     ____                __                                 
              / ____ \\   / __ \\ __  __ _____ / /_                                
             / / __ `//  / /_/ // / / // ___// __/                                
            / / /_/ /  / _, _// /_/ /(__  )/ /_                                  
            \\ \\__,_/  /_/ |_| \\__,_//____/ \\__/                                  
             \\____/                                                              
    ";
    
        println!("{}", logo);
        println!("{}",author);
    

    if args.len() > 1 {
        let option = &args[1];
        if option == "-h" || option == "--help" {
            print_help();
            return;
        }
    }
    
    if args.len() < 3 {
        println!("Usage: {} <mode> <target>", args[0]);
        println!();
        println!("MoreGuide : {} -h | --help", args[0]);
        return;
    }

    let mode = &args[1];
    let target = &args[2];

    match mode.as_str() {
        "dictionary" => {
            // 字典攻击模式
            dictionary::attack(target);
        }
        "weak_password" => {
            // 弱密码检测模式
            weak_password::check(target);
        }
        "hash_cracker" => {
            // 哈希破解模式
            hash_cracker::crack(target);
        }
        _ => {
            println!("Invalid mode");
        }
    }
}
fn print_help() {
    println!("-------------------");
    println!("Black Cracker Rusty");
    println!("-------------------");
    println!();
    println!("About:");
    println!("    A password cracking tool developed by Pure Black");
    println!();
    println!("Usage:");
    println!("    blackcracker_rusty <mode> <target>");
    println!();
    println!("Modes:");
    println!("    dictionary    - Perform dictionary attack");
    println!("    weak_password - Check for weak passwords");
    println!("    hash_cracker  - Crack password hashes");
    println!();
    println!("Examples:");
    println!("    blackcracker_rusty dictionary password123");
    println!("    blackcracker_rusty weak_password user1");
    println!("    blackcracker_rusty hash_cracker 5f4dcc3b5aa765d61d8327deb882cf99");

}