#![allow(non_camel_case_types)]
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub mod client_folder;
pub mod shared_folder;

use client_folder::business_class::BusinessObject;
use client_folder::fsmclass::Fsmclass;
use client_folder::guardclass::Guardclass;
use client_folder::msgfactoryclass::*;
use client_folder::prodmsgclass::Prodmsgclass;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let mut _seed: u16 = 0;
    if arg.len() > 2 {
        // pgm --seed 34
        _seed = arg[2].parse::<u16>().unwrap();
    }
    match go(_seed) {
        Ok(_) => {}
        Err(text) => {
            println!("{}", text);
        }
    }
}
/*--------------------------------------------*/
fn go(_seed: u16) -> Result<String, String> {
    let input_name = "stdinCLIENT.txt".to_string();
    //    let input_name = "stdinSERVER.txt".to_string();
    let path = Path::new(&input_name);
    let display = path.display();
    match File::open(&path) {
        Err(why) => {
            return Result::Err(format!(
                "couldn't open file '{}' , '{}'",
                display,
                why.to_string()
            ))
        }
        Ok(f2) => {
            let _client_msgfactory = Msgfactoryclass::new();
            let _client_productmsg: Prodmsgclass = Prodmsgclass::new(_client_msgfactory);
            let mut _client_guard: Guardclass = Guardclass::new(_client_productmsg, _seed);
            let mut client_fsm: Fsmclass = Fsmclass::new(_client_guard);
            let mut st = client_fsm.get_initial_state();
            let mut bo = BusinessObject::new();

            let bfrdr = BufReader::new(f2);
            for line in bfrdr.lines() {
                let aline = line.unwrap();
                let in_msg_as_vec_u8 = aline.as_bytes().to_vec();
                let in_msg = client_fsm
                    .the_guardobj
                    .the_prodmsgobj
                    .the_msgfactory
                    .wire2comp(&in_msg_as_vec_u8, in_msg_as_vec_u8.len());
                let out_msg = client_fsm.take_event(&in_msg, &mut st, &mut bo);
                match out_msg {
                    Ok(msg) => {
                        let out_msg_as_vec_u8 = client_fsm
                            .the_guardobj
                            .the_prodmsgobj
                            .the_msgfactory
                            .comp2wire(&msg);
                        eprintln!(
                    "Stdin  >>>>>> {out_msg:^50} >>>>>>> FSM\nStdout <<<<<< {in_msg:^50} <<<<<<< FSM",
                    out_msg =  String::from_utf8(in_msg_as_vec_u8.to_vec()).unwrap(),
                    in_msg = String::from_utf8(out_msg_as_vec_u8.to_vec()).unwrap());
                        if client_fsm.is_state_final(&mut st) {
                            eprintln!("stdinclient:: fsm went into final state");
                        }
                    }
                    Err(txt) => {
                        eprintln!("stdinclient::, err={}", txt,);
                        panic!()
                    }
                }
            }
            return Result::Ok("OK".to_string());
        }
    }
}
