use std::env;
use std::process;

extern crate greprs;
use greprs::Config;

fn main() {
	let args: Vec<String>= env::args().collect();

        let config =Config::new(&args).unwrap_or_else(|err|{
            println!("Problem is : {}",err);
            process::exit(1);
        });

        if let Err(e)=greprs::run(config){
            println!("Application error: {}",e);

            process::exit(1);
        }
}

