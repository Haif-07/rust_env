use std::env:: args;

mod zfile;
fn main() {
    let args:Vec<String> = args().collect();
    if args.len() < 2 {
       println!("Usage: zshrc <file>");
       return;
    }
    let filename= &args[1];
    let content = zfile::read_zfile(filename);
    match content {
         Ok(lines) => {
              for line in lines {
                println!("{}", line);
              }
         },
         Err(e) => {
              println!("Error: {}", e);
         }
    }
   
    

}
