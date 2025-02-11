use std::{fs::File, io::{BufReader, BufRead, Error}, str};


// enum ConfigFile {
//     ZshEnv,
//     ZshProfile,
//     ZshRc,
//     ZshLogin,
//     ZshLogout,
// }

// impl ConfigFile {
//     fn path(&self) -> &str {
//         match self {
//             ConfigFile::ZshEnv => "~/.zshenv",
//             ConfigFile::ZshProfile => "~/.zprofile",
//             ConfigFile::ZshRc => "~/.zshrc",
//             ConfigFile::ZshLogin => "~/.zlogin",
//             ConfigFile::ZshLogout => "~/.zlogout",
//         }
//     }
// }
//启动顺序 .zshenv → .zprofile → .zshrc → .zlogin → .zlogout

pub fn read_zfile(str: &str) -> Result<Vec<String>, Error> {
    let  file = File::open(str)?;
    let reader = BufReader::new(file);
    // let mut lines = Vec::new();
    reader.lines().collect()
}


