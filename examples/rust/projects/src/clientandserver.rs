#![allow(non_camel_case_types)]
use std::env;

pub mod client_folder;
pub mod clientandserver_folder;
pub mod server_folder;
pub mod shared_folder;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let mut _seed: u16 = 0;
    if arg.len() > 2 {
        // try different seed
        // pgm --seed 610
        _seed = arg[2].parse::<u16>().unwrap();
    }
    clientandserver_folder::mainimpl::mainimpl(_seed);
}
