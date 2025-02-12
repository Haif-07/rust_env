
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;


// const ZSHENV: &str = "~/.zshenv";
// const ZPROFILE: &str = "~/.zprofile";
// const ZSHRC: &str = "~/.zshrc";
// const ZLOGIN: &str = "~/.zlogin";
// const ZLOGOUT: &str = "~/.zlogout";
// const BASH_PROFILE: &str = "~/.bash_profile";
// const BASH_RC: &str = "~/.bashrc";
// const BASH_LOGIN: &str = "~/.bash_login";
// const BASH_LOGOUT: &str = "~/.bash_logout";
// const PROFILE: &str = "~/.profile";

// 定义通用配置文件接口
pub trait ConfigFile {
    fn load(&mut self) -> io::Result<()>;
    fn save(&self) -> io::Result<()>;
    fn get_env(&self, key: &str) -> Option<&String>;
    fn set_env(&mut self, key: &str, value: &str);
    fn remove_env(&mut self, key: &str);
    fn add_alias(&mut self, name: &str, command: &str);
    fn remove_alias(&mut self, name: &str);
    fn list_all(&self) -> Vec<String>;
}


// 通用配置解析器
pub struct ShellConfig {
    path: String,
    env_vars: HashMap<String, String>,
    aliases: HashMap<String, String>,
    comments: Vec<String>,
    raw_lines: Vec<String>,
}

impl ShellConfig {
    pub  fn new(path: &str) -> Self {
        ShellConfig {
            path: path.to_string(),
            env_vars: HashMap::new(),
            aliases: HashMap::new(),
            comments: Vec::new(),
            raw_lines: Vec::new(),
        }
    }
    fn parse_line(&mut self, line: &str) {
        let line = line.trim();
        if line.starts_with('#') {
            self.comments.push(line.to_string());
            return;
        }

        // 解析环境变量
        let env_re = Regex::new(r#"^export\s+([a-zA-Z_]+)=['"]?(.*?)['"]?$"#).unwrap();
        if let Some(caps) = env_re.captures(line) {
            self.env_vars.insert(caps[1].to_string(), caps[2].to_string());
            return;
        }
        // 解析别名
        let alias_re =  Regex::new(r#"^alias\s+([a-zA-Z_]+)=['"]?(.*?)['"]?$"#).unwrap();

        if let Some(caps) = alias_re.captures(line) {
            self.aliases.insert(caps[1].to_string(), caps[2].to_string());
            return;
        }

        self.raw_lines.push(line.to_string());
    }
}

impl ConfigFile for ShellConfig {
    fn load(&mut self) -> io::Result<()> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            self.parse_line(&line);
        }
        Ok(())
    }

    fn save(&self) -> io::Result<()> {
        let mut contents = String::new();
        
        // 保留原始注释
        for comment in &self.comments {
            contents.push_str(comment);
            contents.push('\n');
        }

        // 写入环境变量
        for (key, value) in &self.env_vars {
            contents.push_str(&format!("export {}={}\n", key, value));
        }

        // 写入别名
        for (name, cmd) in &self.aliases {
            contents.push_str(&format!("alias {}='{}'\n", name, cmd));
        }

        // 保留其他原始内容
        for line in &self.raw_lines {
            contents.push_str(line);
            contents.push('\n');
        }

        fs::write(&self.path, contents)
    }

    fn get_env(&self, key: &str) -> Option<&String> {
        self.env_vars.get(key)
    }

    fn set_env(&mut self, key: &str, value: &str) {
        self.env_vars.insert(key.to_string(), value.to_string());
    }

    fn remove_env(&mut self, key: &str) {
        self.env_vars.remove(key);
    }

    fn add_alias(&mut self, name: &str, command: &str) {
        self.aliases.insert(name.to_string(), command.to_string());
    }

    fn remove_alias(&mut self, name: &str) {
        self.aliases.remove(name);
    }

    fn list_all(&self) -> Vec<String> {
        let mut output = Vec::new();
        output.push("=== Environment Variables ===".to_string());
        for (k, v) in &self.env_vars {
            output.push(format!("{}={}", k, v));
        }
        output.push("\n=== Aliases ===".to_string());
        for (name, cmd) in &self.aliases {
            output.push(format!("alias {}='{}'", name, cmd));
        }
        output
    }
}

