use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_cnd_id;

impl Fsm {
    pub fn genprodmsgclass(
        &mut self,
        _use_lower: &'static str,
        _use_upper: &'static str,
        _no_mess: &String,
    ) -> String {
        let grp_outmsg = self.gen_outmsg_group();
        let mut all_prd_msg_funcs: Vec<String> = Vec::new();

        for (key, vals) in grp_outmsg.iter() {
            let intro = format!(
                "/*--------------  prod_{outmsg}  -------------------------------*/
pub fn prod_{outmsg}(&mut self,
_acond:&OutMsg_{outmsg},
business_object: &mut BusinessObject,
_inmsg:&EnumRealInMessagesType,
)-> 
Result<EnumRealOutMessagesType,String>{{
// A. In case of error return Err(\"..some reason (DO NO COMMIT BUT DO ROLLBACK)...\".to_string());
// B. Create an outmsg template
let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton(\"{outmsg}\");
if let EnumRealOutMessagesType::Msg_{outmsg}(mut the_real_outmsg) = outmsgskeleton {{
  // Got a skeleton the_real_outmsg",
                outmsg = key.to_lowercase()
            );
            all_prd_msg_funcs.push(intro); ///// push

            let xovan = vals
                .iter()
                .map(|arow| {
                  let outmsg = &key.to_lowercase();
                  let inmsg = &arow.in_msg.to_lowercase();
                  let cond = build_cnd_id(&arow.synt_guard_result);
                format!(
                        "(OutMsg_{outmsg}::{cond}, EnumRealInMessagesType::Msg_{inmsg}(the_real_inmsg)) => {{
  // reference around like this
  the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
  the_real_outmsg.all_chars[0] = business_object.something;
  business_object.lastlast =the_real_inmsg.all_chars[7];
  return Ok(EnumRealOutMessages::Msg_{outmsg}(the_real_outmsg))

}}
",
                        outmsg = outmsg,
                        inmsg = inmsg,
                        cond = cond
                    )
                })
                .collect::<Vec<String>>()
                .join(",\n\t");

            let nymitten = format!(
                "match (_acond, _inmsg) {{
{each_cond_and_inmsg}
_=> return Err(format!(\"Condition '{{:?}}' and inmsg '{{:?}}' not allowed here\",_acond,_inmsg)),
      }}
    
      ",
                each_cond_and_inmsg = xovan
            );
            all_prd_msg_funcs.push(nymitten); ///// push
            let slutet = format!(
                "
  }} else {{
    return Err(\"Could not create an outmsg '{outmsg}' in prod_{outmsg}\".to_string());
}}
}}
",
                outmsg = key.to_lowercase()
            );
            all_prd_msg_funcs.push(slutet); ///// push
        }
        ////////
        return format!(
            "use super::business_class::BusinessObject;
use super::messagesets::*;
use super::prodconditions::*;
use super::msgfactoryclass::*;

            
#[derive(Debug)]
pub struct Prodmsgclass {{
//    pub business_object: BusinessObject,
    pub the_msgfactory: Msgfactoryclass,
}}
impl Prodmsgclass {{
    pub fn new(//business_object:BusinessObject,
    the_msgfactory: Msgfactoryclass
    ) -> Prodmsgclass {{
      Prodmsgclass {{
            the_msgfactory: the_msgfactory,
//            business_object : business_object
        }}
    }}
    pub fn get_whatever(&mut self) -> String {{
        return  \"\".to_string()
    }}
    {allprodfunctions}
}}",
            allprodfunctions = all_prd_msg_funcs.join("\n")
        );
    }
}
