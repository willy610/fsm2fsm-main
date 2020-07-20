use super::super::shared_folder::realmessages::*;
use super::messagesets::*;
/*
This is a skelton
1. gen_out_msg_skeleton => Is used in each prod_* method
   in order to create new messages.
2. gen_in_msg_skeleton => Is used when creating new message
   to be send into the fsm. At least for kicking in the first message
   at initial state.
3. Format on wire is (msgname, ',', message) as u8
4. wire2comp and comp2wire => Are pseudo for converting between
   computational format and wire format. To be used for transport message
   on some kind of serial media. Just very outlined here
*/
#[derive(Debug)]
pub struct Msgfactoryclass {
    pub something: String,
}
impl Msgfactoryclass {
    pub fn new() -> Msgfactoryclass {
        Msgfactoryclass {
            something: "something".to_string(),
        }
    }
    ////////////////////////
    pub fn wire2comp(
        &mut self,
        //      tag:String,_in_wire_bytes:&Vec<char>,
        in_wire_as_vec_u8: &Vec<u8>,
        _in_wire_len: usize, // check lengt of input
    ) -> EnumRealInMessagesType {
        let comma_pos = in_wire_as_vec_u8
            .iter()
            .position(|&x| x == (',' as u8))
            .unwrap();
        let mut rec_tag: String = String::new();
        for i in 0..comma_pos {
            rec_tag.push(in_wire_as_vec_u8[i] as char);
        }
        let mut rec_all_chars: [char; 8] = ['_'; 8];
        for pos in 0..8 {
            rec_all_chars[pos] = in_wire_as_vec_u8[pos + comma_pos + 1] as char;
        }

        //      let tag_str = tag.as_str();
        //      let mut the_most: [char; 8] = [' '; 8];
        //      for i in 0..7 {
        //        the_most[i] = _in_wire_bytes[i];
        //      }
        match rec_tag.as_str() {
            "Msg_24" => {
                return EnumRealInMessagesType::Msg_24(RealMsg_24Type {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_42" => {
                return EnumRealInMessagesType::Msg_42(RealMsg_42Type {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_hello_alice" => {
                return EnumRealInMessagesType::Msg_hello_alice(RealMsg_hello_aliceType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_hm" => {
                return EnumRealInMessagesType::Msg_hm(RealMsg_hmType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_how_are_you" => {
                return EnumRealInMessagesType::Msg_how_are_you(RealMsg_how_are_youType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_no" => {
                return EnumRealInMessagesType::Msg_no(RealMsg_noType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_other" => {
                return EnumRealInMessagesType::Msg_other(RealMsg_otherType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_ping" => {
                return EnumRealInMessagesType::Msg_ping(RealMsg_pingType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_well" => {
                return EnumRealInMessagesType::Msg_well(RealMsg_wellType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_yes" => {
                return EnumRealInMessagesType::Msg_yes(RealMsg_yesType {
                    all_chars: rec_all_chars,
                });
            }
            _ => panic!(
                "Msgfactoryclass::wire2comp() wire form tag '{}' unknown",
                rec_tag
            ),
        }
    }
    ////////////////////////
    pub fn comp2wire(&mut self, out_message: &EnumRealOutMessagesType) -> Vec<u8> {
        return match out_message {
            EnumRealOutMessagesType::Msg_43(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                ["Msg_43".as_bytes().to_vec(), ",".as_bytes().to_vec(), _msg].concat()
            }
            EnumRealOutMessagesType::Msg_blablabla(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_blablabla".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_bye(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                ["Msg_bye".as_bytes().to_vec(), ",".as_bytes().to_vec(), _msg].concat()
            }
            EnumRealOutMessagesType::Msg_bye_bob(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_bye_bob".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_dontdisturb(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_dontdisturb".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_hello_bob(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_hello_bob".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_or(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                ["Msg_or".as_bytes().to_vec(), ",".as_bytes().to_vec(), _msg].concat()
            }
            EnumRealOutMessagesType::Msg_pong(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_pong".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_questionwas(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_questionwas".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_rich(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_rich".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_sorry(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_sorry".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_tired(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_tired".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_what(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_what".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
        };
    }
    ////////////////////////
    pub fn gen_in_msg_skeleton(&mut self, msgname: &'static str) -> EnumRealInMessages {
        match msgname {
            "24" => {
                return EnumRealInMessages::Msg_24(RealMsg_24 {
                    //    last: 'x',
                    all_chars: ['a'; 8],
                });
            }
            "42" => {
                return EnumRealInMessages::Msg_42(RealMsg_42 {
                    //    last: 'x',
                    all_chars: ['b'; 8],
                });
            }
            "hello_alice" => {
                return EnumRealInMessages::Msg_hello_alice(RealMsg_hello_alice {
                    //    last: 'x',
                    all_chars: ['c'; 8],
                });
            }
            "hm" => {
                return EnumRealInMessages::Msg_hm(RealMsg_hm {
                    //    last: 'x',
                    all_chars: ['d'; 8],
                });
            }
            "how_are_you" => {
                return EnumRealInMessages::Msg_how_are_you(RealMsg_how_are_you {
                    //    last: 'x',
                    all_chars: ['e'; 8],
                });
            }
            "no" => {
                return EnumRealInMessages::Msg_no(RealMsg_no {
                    //    last: 'x',
                    all_chars: ['f'; 8],
                });
            }
            "other" => {
                return EnumRealInMessages::Msg_other(RealMsg_other {
                    //    last: 'x',
                    all_chars: ['g'; 8],
                });
            }
            "ping" => {
                return EnumRealInMessages::Msg_ping(RealMsg_ping {
                    //    last: 'x',
                    all_chars: ['h'; 8],
                });
            }
            "well" => {
                return EnumRealInMessages::Msg_well(RealMsg_well {
                    //    last: 'x',
                    all_chars: ['i'; 8],
                });
            }
            "yes" => {
                return EnumRealInMessages::Msg_yes(RealMsg_yes {
                    //    last: 'x',
                    all_chars: ['j'; 8],
                });
            }
            _ => panic!(
                "Msgfactoryclass::gen_in_msg_skeleton() outmsg '{}' unknown",
                msgname
            ),
        }
    }
    ////////////////////////
    pub fn gen_out_msg_skeleton(&mut self, msgname: &'static str) -> EnumRealOutMessages {
        match msgname {
            "43" => {
                return EnumRealOutMessages::Msg_43(RealMsg_43 {
                    all_chars: ['a'; 8],
                })
            }
            "blablabla" => {
                return EnumRealOutMessages::Msg_blablabla(RealMsg_blablabla {
                    all_chars: ['b'; 8],
                })
            }
            "bye" => {
                return EnumRealOutMessages::Msg_bye(RealMsg_bye {
                    all_chars: ['c'; 8],
                })
            }
            "bye_bob" => {
                return EnumRealOutMessages::Msg_bye_bob(RealMsg_bye_bob {
                    all_chars: ['d'; 8],
                })
            }
            "dontdisturb" => {
                return EnumRealOutMessages::Msg_dontdisturb(RealMsg_dontdisturb {
                    all_chars: ['e'; 8],
                })
            }
            "hello_bob" => {
                return EnumRealOutMessages::Msg_hello_bob(RealMsg_hello_bob {
                    all_chars: ['f'; 8],
                })
            }
            "or" => {
                return EnumRealOutMessages::Msg_or(RealMsg_or {
                    all_chars: ['g'; 8],
                })
            }
            "pong" => {
                return EnumRealOutMessages::Msg_pong(RealMsg_pong {
                    all_chars: ['h'; 8],
                })
            }
            "questionwas" => {
                return EnumRealOutMessages::Msg_questionwas(RealMsg_questionwas {
                    all_chars: ['i'; 8],
                })
            }
            "rich" => {
                return EnumRealOutMessages::Msg_rich(RealMsg_rich {
                    all_chars: ['j'; 8],
                })
            }
            "sorry" => {
                return EnumRealOutMessages::Msg_sorry(RealMsg_sorry {
                    all_chars: ['k'; 8],
                })
            }
            "tired" => {
                return EnumRealOutMessages::Msg_tired(RealMsg_tired {
                    all_chars: ['l'; 8],
                })
            }
            "what" => {
                return EnumRealOutMessages::Msg_what(RealMsg_what {
                    all_chars: ['m'; 8],
                })
            }
            _ => panic!(
                "Msgfactoryclass::gen_out_msg_skeleton() outmsg '{}' unknown",
                msgname
            ),
        }
    }
}
