mod core;
use core::info_pool::InfoPool;
use core::output::{OutputHandler, output_file_name};

use std::{
    env::{self}, 
    fs::remove_file, 
    path::Path, 
    process::exit
};

fn main() {
    let mut info_pool = InfoPool::new();
    info_pool.update();

    let file_name = output_file_name();
    if Path::new(&file_name).exists() {
        remove_file(&file_name).unwrap();
    }

    let handler = OutputHandler::new(info_pool);
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 && args[1] == "save" {
        if let Err(err) = handler.print_info_save(file_name) {
            eprintln!("Error: {}", err);
            exit(-1);
        }
    } else {
        handler.print_info();
    }
}