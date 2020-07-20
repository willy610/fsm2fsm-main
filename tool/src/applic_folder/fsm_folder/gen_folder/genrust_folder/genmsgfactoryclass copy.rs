use crate::applic_folder::fsm_folder::fsm::Fsm;
impl Fsm {
    pub fn genmsgfactoryclass(&mut self) -> String {
        let grp_outmsg = self.gen_outmsg_group();
        let grp_inmsg = self.gen_inmsg_group();
        ///////////////////
        let vec_realinmsg:String = grp_inmsg.iter().map(|(inmsg,_)|
    format!("\"{inmsg}\" => {{return EnumRealInMessagesType::Msg_{inmsg}(RealMsg_{inmsg}Type {{last: 'x',most: the_most}});}}",
    inmsg=inmsg)
  ).collect::<Vec<String>>()
  .join(",\n");
        ///////////////////
        /*
        EnumRealOutMessagesType::Msg_open(the_real_outmsg) =>{
          let x = the_real_outmsg.last;
        */
        let vec_realoutmsg: String = grp_outmsg
            .iter()
            .map(|(outmsg, _)| {
                format!(
                    "EnumRealOutMessagesType::Msg_{outmsg}(the_real_outmsg) =>{{
  tag = \"{outmsg}\".to_string();// wire tag
  retvec.extend_from_slice(&the_real_outmsg.most);
  retvec.push(the_real_outmsg.last);
  }}",
                    outmsg = outmsg,
                )
            })
            .collect::<Vec<String>>()
            .join(",\n");
        ///////////////////
        let vec_outskeletons: String = grp_outmsg
            .iter()
            .map(|(outmsg, _)| {
                format!(
                    "\"{outmsg}\" =>{{
  return EnumRealOutMessages::Msg_{outmsg}(RealMsg_{outmsg} {{
    last: 'x',
    most: ['a'; 8],
}})  }}",
                    outmsg = outmsg,
                )
            })
            .collect::<Vec<String>>()
            .join(",\n");
        ///////////////////
        let vec_inskeletons: String = grp_inmsg
            .iter()
            .map(|(inmsg, _)| {
                format!(
                    "\"{inmsg}\" =>{{
  return EnumRealInMessages::Msg_{inmsg}(RealMsg_{inmsg} {{
    last: 'x',
    most: ['a'; 8],
}})  }}",
                    inmsg = inmsg,
                )
            })
            .collect::<Vec<String>>()
            .join(",\n");

        ///////////////////
        return format!(
            "use super::super::shared_folder::realmessages::*;
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
pub struct Msgfactoryclass {{
    pub something: String,
}}
impl Msgfactoryclass {{
    pub fn new() -> Msgfactoryclass {{
      Msgfactoryclass {{
            something: \"something\".to_string(),
        }}
    }}
    ////////////////////////
    pub fn wire2comp(&mut self, tag:String,_in_wire_bytes:&Vec<char>,
      _in_wire_len:usize// check lengt of input
    )
    ->EnumRealInMessagesType
    {{
      let tag_str = tag.as_str();
      let mut the_most: [char; 8] = [' '; 8];
      for i in 0..7 {{
        the_most[i] = _in_wire_bytes[i];
      }}
    match tag_str {{
        {vec_realinmsg},
      _=>panic!(\"Msgfactoryclass::wire2comp() wire form tag {{}} unknown\",tag_str)
      }}
    }}
    ////////////////////////
    pub fn comp2wire(&mut self,out_message:&EnumRealOutMessagesType)->
    (String,Vec<char>)
    {{
      let mut retvec:Vec<char> = Vec::new(); 
      let tag:String;
      match out_message
      {{
        {vec_realoutmsg}
      }}
      return (tag,retvec)
    }}
    ////////////////////////
     pub fn gen_in_msg_skeleton(&mut self, msgname: &'static str) ->
     EnumRealInMessages
     {{
      match msgname {{
        {vec_inskeletons}
        _=>panic!(\"Msgfactoryclass::gen_in_msg_skeleton() outmsg {{}} unknown\",msgname)
      }}
      }}
    ////////////////////////
    pub fn gen_out_msg_skeleton(&mut self, msgname: &'static str) -> 
    EnumRealOutMessages {{
      match msgname
      {{
        {vec_outskeletons}
        _=>panic!(\"Msgfactoryclass::gen_out_msg_skeleton() outmsg {{}} unknown\",msgname)
      }}
    }}
}}
",
            vec_realinmsg = vec_realinmsg,
            vec_realoutmsg = vec_realoutmsg,
            vec_outskeletons = vec_outskeletons,
            vec_inskeletons = vec_inskeletons
        );
    }
}
