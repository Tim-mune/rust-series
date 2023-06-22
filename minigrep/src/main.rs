use core::panic;
use minigrep::Config;

use std::{env,fs};
use std::error::Error;
use std::process;

fn main() {
    let args:Vec<String>=env::args().collect();
   
//    let config=Config::build(&args);
//    let mut query="".to_string();
//    let mut file_path="".to_string();
//    match config {
//        Ok(config)=>{
//         println!("result is {:?}",config.query);
// query=config.query;
// file_path=config.file_path;
//        }
//        Err(er)=>println!("theres an error {}",er),
//    }
let config=Config::build(&args).unwrap_or_else(|err|{
    println!("Problem parsing  arguments: {err}");
    process::exit(1);
});
// match read(config) {
//     Ok(value)=>{println!("Contents are \n {:?}",value)},
//     Err(er)=>println!("{} error was found",er)
// }
if let Err(e) =minigrep::read(config)  {
    println!("application error: {e}");
    process::exit(1)
}
}


