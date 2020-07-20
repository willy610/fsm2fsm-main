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
            "Msg_43" => {
                return EnumRealInMessagesType::Msg_43(RealMsg_43Type {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_blablabla" => {
                return EnumRealInMessagesType::Msg_blablabla(RealMsg_blablablaType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_bye" => {
                return EnumRealInMessagesType::Msg_bye(RealMsg_byeType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_bye_bob" => {
                return EnumRealInMessagesType::Msg_bye_bob(RealMsg_bye_bobType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_callin" => {
                return EnumRealInMessagesType::Msg_callin(RealMsg_callinType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_dontdisturb" => {
                return EnumRealInMessagesType::Msg_dontdisturb(RealMsg_dontdisturbType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_hello_bob" => {
                return EnumRealInMessagesType::Msg_hello_bob(RealMsg_hello_bobType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_or" => {
                return EnumRealInMessagesType::Msg_or(RealMsg_orType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_pong" => {
                return EnumRealInMessagesType::Msg_pong(RealMsg_pongType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_questionwas" => {
                return EnumRealInMessagesType::Msg_questionwas(RealMsg_questionwasType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_rich" => {
                return EnumRealInMessagesType::Msg_rich(RealMsg_richType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_sorry" => {
                return EnumRealInMessagesType::Msg_sorry(RealMsg_sorryType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_tired" => {
                return EnumRealInMessagesType::Msg_tired(RealMsg_tiredType {
                    all_chars: rec_all_chars,
                });
            }
            "Msg_what" => {
                return EnumRealInMessagesType::Msg_what(RealMsg_whatType {
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
            EnumRealOutMessagesType::Msg_24(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                ["Msg_24".as_bytes().to_vec(), ",".as_bytes().to_vec(), _msg].concat()
            }
            EnumRealOutMessagesType::Msg_42(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                ["Msg_42".as_bytes().to_vec(), ",".as_bytes().to_vec(), _msg].concat()
            }
            EnumRealOutMessagesType::Msg_hello_alice(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_hello_alice".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_hm(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                ["Msg_hm".as_bytes().to_vec(), ",".as_bytes().to_vec(), _msg].concat()
            }
            EnumRealOutMessagesType::Msg_how_are_you(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_how_are_you".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_no(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                ["Msg_no".as_bytes().to_vec(), ",".as_bytes().to_vec(), _msg].concat()
            }
            EnumRealOutMessagesType::Msg_nooutput(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_nooutput".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_other(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_other".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_ping(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_ping".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_well(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                [
                    "Msg_well".as_bytes().to_vec(),
                    ",".as_bytes().to_vec(),
                    _msg,
                ]
                .concat()
            }
            EnumRealOutMessagesType::Msg_yes(the_real_outmsg) => {
                let _msg: Vec<u8> = the_real_outmsg.all_chars.iter().map(|x| *x as u8).collect();
                ["Msg_yes".as_bytes().to_vec(), ",".as_bytes().to_vec(), _msg].concat()
            }
        };
    }
    ////////////////////////
    pub fn gen_in_msg_skeleton(&mut self, msgname: &'static str) -> EnumRealInMessages {
        match msgname {
            "43" => {
                return EnumRealInMessages::Msg_43(RealMsg_43 {
                    //    last: 'x',
                    all_chars: ['a'; 8],
                });
            }
            "blablabla" => {
                return EnumRealInMessages::Msg_blablabla(RealMsg_blablabla {
                    //    last: 'x',
                    all_chars: ['b'; 8],
                });
            }
            "bye" => {
                return EnumRealInMessages::Msg_bye(RealMsg_bye {
                    //    last: 'x',
                    all_chars: ['c'; 8],
                });
            }
            "bye_bob" => {
                return EnumRealInMessages::Msg_bye_bob(RealMsg_bye_bob {
                    //    last: 'x',
                    all_chars: ['d'; 8],
                });
            }
            "callin" => {
                return EnumRealInMessages::Msg_callin(RealMsg_callin {
                    //    last: 'x',
                    all_chars: ['e'; 8],
                });
            }
            "dontdisturb" => {
                return EnumRealInMessages::Msg_dontdisturb(RealMsg_dontdisturb {
                    //    last: 'x',
                    all_chars: ['f'; 8],
                });
            }
            "hello_bob" => {
                return EnumRealInMessages::Msg_hello_bob(RealMsg_hello_bob {
                    //    last: 'x',
                    all_chars: ['g'; 8],
                });
            }
            "or" => {
                return EnumRealInMessages::Msg_or(RealMsg_or {
                    //    last: 'x',
                    all_chars: ['h'; 8],
                });
            }
            "pong" => {
                return EnumRealInMessages::Msg_pong(RealMsg_pong {
                    //    last: 'x',
                    all_chars: ['i'; 8],
                });
            }
            "questionwas" => {
                return EnumRealInMessages::Msg_questionwas(RealMsg_questionwas {
                    //    last: 'x',
                    all_chars: ['j'; 8],
                });
            }
            "rich" => {
                return EnumRealInMessages::Msg_rich(RealMsg_rich {
                    //    last: 'x',
                    all_chars: ['k'; 8],
                });
            }
            "sorry" => {
                return EnumRealInMessages::Msg_sorry(RealMsg_sorry {
                    //    last: 'x',
                    all_chars: ['l'; 8],
                });
            }
            "tired" => {
                return EnumRealInMessages::Msg_tired(RealMsg_tired {
                    //    last: 'x',
                    all_chars: ['m'; 8],
                });
            }
            "what" => {
                return EnumRealInMessages::Msg_what(RealMsg_what {
                    //    last: 'x',
                    all_chars: ['n'; 8],
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
            "24" => {
                return EnumRealOutMessages::Msg_24(RealMsg_24 {
                    all_chars: ['a'; 8],
                })
            }
            "42" => {
                return EnumRealOutMessages::Msg_42(RealMsg_42 {
                    all_chars: ['b'; 8],
                })
            }
            "hello_alice" => {
                return EnumRealOutMessages::Msg_hello_alice(RealMsg_hello_alice {
                    all_chars: ['c'; 8],
                })
            }
            "hm" => {
                return EnumRealOutMessages::Msg_hm(RealMsg_hm {
                    all_chars: ['d'; 8],
                })
            }
            "how_are_you" => {
                return EnumRealOutMessages::Msg_how_are_you(RealMsg_how_are_you {
                    all_chars: ['e'; 8],
                })
            }
            "no" => {
                return EnumRealOutMessages::Msg_no(RealMsg_no {
                    all_chars: ['f'; 8],
                })
            }
            "nooutput" => {
                return EnumRealOutMessages::Msg_nooutput(RealMsg_nooutput {
                    all_chars: ['g'; 8],
                })
            }
            "other" => {
                return EnumRealOutMessages::Msg_other(RealMsg_other {
                    all_chars: ['h'; 8],
                })
            }
            "ping" => {
                return EnumRealOutMessages::Msg_ping(RealMsg_ping {
                    all_chars: ['i'; 8],
                })
            }
            "well" => {
                return EnumRealOutMessages::Msg_well(RealMsg_well {
                    all_chars: ['j'; 8],
                })
            }
            "yes" => {
                return EnumRealOutMessages::Msg_yes(RealMsg_yes {
                    all_chars: ['k'; 8],
                })
            }
            _ => panic!(
                "Msgfactoryclass::gen_out_msg_skeleton() outmsg '{}' unknown",
                msgname
            ),
        }
    }
}
