#![allow(non_camel_case_types)]
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::net::UdpSocket;
pub mod server_folder;
pub mod shared_folder;
use std::{thread, time};

use server_folder::business_class::BusinessObject;
use server_folder::fsmclass::Fsmclass;
use server_folder::fsmclass::States;
use server_folder::guardclass::Guardclass;
use server_folder::msgfactoryclass::*;
use server_folder::prodmsgclass::Prodmsgclass;
fn main() {
    let arg: Vec<String> = env::args().collect();
    let mut _seed: u16 = 0;
    if arg.len() > 2 {
        // pgm --seed 1232
        _seed = arg[2].parse::<u16>().unwrap();
    }

    let mut client_sessions: HashMap<SocketAddr, (BusinessObject, States)> = HashMap::new();
    let socket = UdpSocket::bind("127.0.0.1:8081").expect("Server could not bind to server port");
    let mut _server_msgfactory = Msgfactoryclass::new();
    let mut _server_productmsg: Prodmsgclass = Prodmsgclass::new(_server_msgfactory);
    let mut _server_guard: Guardclass = Guardclass::new(_server_productmsg, _seed);

    let mut server_fsm: Fsmclass = Fsmclass::new(_server_guard);
    let initial_state = server_fsm.get_initial_state();
    loop {
        let mut in_msg_as_vec_u8 = [0u8; 1025];
        match socket.recv_from(&mut in_msg_as_vec_u8) {
            Ok((read_len, ref src)) => {
                // Is this the first in message from an unknown client
                if !client_sessions.contains_key(src) {
                    // first call. create a business obj and an state with propoer value
                    client_sessions.insert(src.clone(), (BusinessObject::new(), initial_state));
                    eprintln!("udpserver:: New session for client port {:?}\n", src);
                }
                let (bo, st) = client_sessions.get_mut(src).unwrap();
                let in_msg = server_fsm
                    .the_guardobj
                    .the_prodmsgobj
                    .the_msgfactory
                    .wire2comp(&in_msg_as_vec_u8[0..read_len].to_vec(), read_len);
                // forward state and busines obj
                let out_msg = server_fsm.take_event(&in_msg, st, bo);
                // service time
                thread::sleep(time::Duration::from_secs(1));
                match out_msg {
                    Ok(msg) => {
                        let out_msg_as_vec_u8 = server_fsm
                            .the_guardobj
                            .the_prodmsgobj
                            .the_msgfactory
                            .comp2wire(&msg);

                        eprintln!(
                            "Client >>>>>> {msg_in:^25} >>>>>>> Server",
                            msg_in =
                                String::from_utf8(in_msg_as_vec_u8[0..read_len].to_vec()).unwrap()
                        );
                        eprintln!(
                            "Client <<<<<< {msg_out:^25} <<<<<<< Server",
                            msg_out = String::from_utf8(out_msg_as_vec_u8.to_vec()).unwrap()
                        );
                        if server_fsm.is_state_final(st) {
                            eprintln!("udpserver:: Session ended for client port {:?}\n", src);
                            // delete the session
                            let _how = client_sessions.remove(src);
                        }
                        match socket.send_to(&out_msg_as_vec_u8, &src) {
                            Ok(_) => {}
                            Err(txt) => {
                                eprintln!("mainserver::send_to, err={}", txt);
                                panic!()
                            }
                        }
                    }
                    Err(txt) => {
                        eprintln!("mainserver::, err={}", txt);
                        panic!()
                    }
                }
            }
            Err(text) => panic!(text),
        }
    }
}
