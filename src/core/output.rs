use std::{fs::OpenOptions, io::{self, Write}};
use chrono::{Local, Datelike};

use crate::core::info_pool::InfoPool;

pub fn output_file_name() -> String {
    let now = Local::now();
    format!("SIN{}-{}-{}.genv", now.year(), now.month(), now.day())
}

pub struct OutputHandler {
    info_pool: InfoPool
}

impl OutputHandler {
    pub fn new(info_pool: InfoPool) -> Self {
        Self { info_pool }
    }

    pub fn print_info(&self) {
        self.info_pool.print_info(|info| {
            println!("{}", info);
        });
    }
    
    pub fn print_info_save(&self, filename: String) -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(filename)?;
    
        self.info_pool.print_info(|info| {
            if let Err(err) = writeln!(file, "{}", info) {
                eprintln!("Couldn't write to file: {}", err);
            }
    
            println!("{}", info);
        });
    
        Ok(())
    }
}