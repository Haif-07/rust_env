use std::{env:: args, io::{self, Write}};

use config_file::ConfigFile;
mod config_file;
fn main() {
    let args:Vec<String> = args().collect();
    if args.len() < 2 {
       println!("Usage: zshrc <file>");
       return;
    }
     let file_path = &args[1];
     let mut shell_config = config_file::ShellConfig::new(file_path);
     shell_config.load().unwrap();
     loop {
          print!("{} => ",file_path); // 提示符
          io::stdout().flush().unwrap(); // 立即刷新输出
          
          let mut input = String::new();
          io::stdin().read_line(&mut input).unwrap();
          let input = input.trim(); // 移除换行和空格
          
          if input == "exit" || input == "quit" {
              println!("Exiting...");
              break;
          }
  
          let mut args: Vec<&str> = input.split_whitespace().collect();
          if args.is_empty() {
              continue;
          }
  
          match args[0] {
              "list" => {
                  for line in shell_config.list_all() {
                      println!("{}", line);
                  }
              }
              "add" => {
                  if args.len() < 3 {
                      println!("Usage: add <key> <value>");
                      continue;
                  }
                  shell_config.set_env(args[1], args[2]);
                  shell_config.save().unwrap();
                  println!("Added: {}={}", args[1], args[2]);
              }
              _ => {
                  println!("Unknown command: {}", args[0]);
              }
          }
     }
     
}
