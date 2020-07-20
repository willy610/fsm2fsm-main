#![allow(non_camel_case_types)]
use std::env;
use std::net::{Ipv4Addr, UdpSocket};
pub mod client_folder;
pub mod shared_folder;

use client_folder::business_class::BusinessObject;
use client_folder::fsmclass::Fsmclass;
use client_folder::guardclass::Guardclass;
use client_folder::messagesets::*;
use client_folder::msgfactoryclass::*;
use client_folder::prodmsgclass::Prodmsgclass;

fn main() {
    let arg: Vec<String> = env::args().collect();
    let mut _seed: u16 = 0;
    if arg.len() > 2 {
        // pgm --seed 61016
        _seed = arg[2].parse::<u16>().unwrap();
    }
    let mut maxet = 20;
    let mut client_in_final_state = false;
    let mut in_msg_as_vec_u8 = [0u8; 1025];

    let any: Ipv4Addr = "127.0.0.1".parse().unwrap();
    let port: u16 = 0;
    // Join localhost and use any given port during session
    let socket = UdpSocket::bind((any, port)).expect("Client could not bind to socket");
    // the server is listening  on 8081
    socket
        .connect("127.0.0.1:8081")
        .expect("Client could not bind to server port");
    eprintln!("Own socket={:?}", socket);

    let mut _client_msgfactory = Msgfactoryclass::new();
    let mut _client_productmsg: Prodmsgclass = Prodmsgclass::new(_client_msgfactory);
    let mut _client_guard: Guardclass = Guardclass::new(_client_productmsg, _seed);
    let mut client_fsm: Fsmclass = Fsmclass::new(_client_guard);
    let possible_init_messages_to_client = client_fsm.get_possible_init_messages();

    let mut st = client_fsm.get_initial_state();
    let mut bo = BusinessObject::new();

    let first_of_intitial_possible_init_messages_to_client: EnumRealInMessages = client_fsm
        .the_guardobj
        .the_prodmsgobj
        .the_msgfactory
        .gen_in_msg_skeleton(possible_init_messages_to_client[0]);

    let mut kick_start = client_fsm.take_event(
        &first_of_intitial_possible_init_messages_to_client,
        &mut st,
        &mut bo,
    );

    while maxet > 0 && !client_in_final_state {
        match kick_start {
            Ok(msg) => {
                if client_fsm.is_state_final(&mut st) {
                    eprintln!("udpclient:: fsm went into final state");
                    client_in_final_state = true;
                }
                if client_in_final_state {
                    break;
                }
                // let msgfactory convert (comp2wire) output msg from self fsm
                let out_msg_as_vec_u8 = client_fsm
                    .the_guardobj
                    .the_prodmsgobj
                    .the_msgfactory
                    .comp2wire(&msg);
                socket
                    .send(&out_msg_as_vec_u8)
                    .expect("Client failed to send to Server");

                // receive a wire form back)
                match socket.recv_from(&mut in_msg_as_vec_u8) {
                    Ok((read_len, _src)) => {
                        let in_msg = client_fsm
                            .the_guardobj
                            .the_prodmsgobj
                            .the_msgfactory
                            .wire2comp(&in_msg_as_vec_u8[0..read_len].to_vec(), read_len);
                        eprintln!(
                          "Client >>>>>> {msg_out:^25} >>>>>>> Server\nClient <<<<<< {msg_in:^25} <<<<<<< Server",
                          msg_in = String::from_utf8(in_msg_as_vec_u8[0..read_len].to_vec()).unwrap(),
                          msg_out = String::from_utf8(out_msg_as_vec_u8.to_vec()).unwrap()
                      );
                        kick_start = client_fsm.take_event(&in_msg, &mut st, &mut bo);
                    }
                    Err(text) => panic!(text),
                }
            }
            Err(txt) => {
                eprintln!("mainclient::, err={}", txt);
                panic!()
            }
        }
        maxet = maxet - 1;
    }
}
