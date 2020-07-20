use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_cnd_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_msg_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_state_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::RowToStateEventGroup;

use std::collections::HashSet;

impl Fsm {
    pub fn genfsmclass(
        &mut self,
        _use_lower: &'static str,
        _use_upper: &'static str,
        _no_mess: &String,
    ) -> String {
        fn build_one_err_format_expression(
            st_id: &String,
            msg_id: &String,
            send_mess: &String,
        ) -> String {
            return format!("format!(\"In State={state}, InMsg={inmsg} the OutMsg must be of kind '{outmsg}', but was='{{was:?}}'\",was=outmsg)",
      state=st_id,
      inmsg=msg_id,
      outmsg = send_mess);
        }
        /*-------------------------------------*/
        fn build_one_cond(
            smallst: &String,
            smalleve: &String,
            user_guard: &String,
            synt_guard: &String,
            new_state: &String,
            send_mess: &String,
            err_format_expression: &String,
        ) -> String {
            return format!(
                "/* User Guard is: {user_guard} */
Ok((Conds_{smallstx}_{smallevex}::{synt_guard},outmsg)) => {{
  *st = States::{tostate};
  match outmsg {{
    EnumRealOutMessagesType::{outmsg}{{..}} =>{{return Ok(outmsg);}},
  _=>{{return Err({errformatexpression})}}
  }}}}",
                smallstx = smallst,
                smallevex = smalleve,
                synt_guard = synt_guard,
                user_guard = user_guard,
                tostate = new_state,
                outmsg = send_mess,
                errformatexpression = err_format_expression
            );
        } /*-------------------------------------*/
        fn build_one_cond_enum(
            rows: &Vec<RowToStateEventGroup>,
            smallst: &String,
            smalleve: &String,
        ) -> String {
            return format!(
                "\n#[derive(Debug)]\npub enum Conds_{state}_{event} {{\n {} }}\n",
                rows.iter()
                    .map(|a_row| build_cnd_id(&a_row.synt_guard_result))
                    .collect::<Vec<String>>()
                    .join(",\n"),
                state = smallst,
                event = smalleve,
            );
        }
        /*-------------------------------------*/
        /*-------------------------------------*/
        fn build_err_return(
            st_id: &String,
            msg_id: &String,
            smallst: &String,
            smalleve: &String,
        ) -> String {
            let err_format = format!(
          "In (State={state},InMsg={inmsg},GuardFunction={smallst}_{smalleve}) error from function was ={{}}\",err",
                      state = st_id,
                      inmsg= msg_id,
                      smallst =smallst,
                      smalleve=smalleve
                    );
            return format!("Err(err) => {{ return Err(format!(\"{})); }}", err_format);
        }
        /*-------------------------------------*/
        fn buid_one_body(
            st_id: &String,
            msg_id: &String,
            smallst: &String,
            smalleve: &String,
            allcond: &Vec<String>,
        ) -> String {
            return format!(
                "
(States::{st_id},EnumRealInMessagesType::{msg_id}(_))=> 
match self.the_guardobj.{smallst}_{smalleve}(an_event, business_object) {{\n {allconds} 
          }}",
                st_id = st_id,
                msg_id = msg_id,
                smallst = smallst,
                smalleve = smalleve,
                allconds = allcond.join("\n"),
            );
        }
        /*-------------------------------------*/
        /*-------------------------------------*/
        let mut all_cond_enums: Vec<String> = Vec::new();
        let mut all_inmsgs_in_initial_state: Vec<String> = Vec::new();

        let mut all_states: HashSet<String> = HashSet::new();
        all_states.insert(build_state_id(&self.get_final_state()));

        let grp_state_event = self.gen_state_event_group();

        let mut body: Vec<String> = Vec::new();
        let build_fall_out = || -> String {
            return "_=>{ let x = format!(\"ERROR 3: Inmessage= {:?} not accepted in state= {:?}\",an_event,st);
          return Err(x);}"
                .to_string();
        };

        for ((state, event), rows) in grp_state_event.iter() {
            let smallst = state.to_lowercase();
            let smalleve = event.to_lowercase();

            let st_id = build_state_id(state);
            let msg_id = build_msg_id(event);
            all_states.insert(st_id.clone());
            if *state == self.initial_state {
                all_inmsgs_in_initial_state.push(event.clone());
            }
            let mut conds_in_this_state_eve: Vec<String> = Vec::new();
            for arow in rows.iter() {
                let user_guard = &arow.user_guard_result;
                let synt_guard = build_cnd_id(&arow.synt_guard_result);
                let new_state = build_state_id(&arow.to_state);
                let out_msg = &arow.out_msg;
                let _send_mess = build_msg_id(out_msg);
                let err_format_expression =
                    build_one_err_format_expression(&st_id, &msg_id, &_send_mess);
                let ett_cond = build_one_cond(
                    &smallst,
                    &smalleve,
                    &user_guard,
                    &synt_guard,
                    &new_state,
                    &_send_mess,
                    &err_format_expression,
                );
                conds_in_this_state_eve.push(ett_cond);
            }
            ///// COND_enums
            all_cond_enums.push(build_one_cond_enum(&rows, &smallst, &smalleve));
            conds_in_this_state_eve.push(build_err_return(&st_id, &msg_id, &smallst, &smalleve));
            body.push(buid_one_body(
                &st_id,
                &msg_id,
                &smallst,
                &smalleve,
                &conds_in_this_state_eve,
            ));
        }
        body.push(build_fall_out());

        //eprintln!("all_states={:?}",all_states);
        let build_all_states = || {
            let all_states_in_enum = all_states
                .iter()
                .map(|st| st.to_string())
                .collect::<Vec<String>>()
                .join("\n,\t");
            format!(
                "#[derive(Debug,PartialEq,Clone,Copy)]\npub enum States {{ {:} }}\n",
                all_states_in_enum
            )
        };

        let states = build_all_states();
        let mut cover = || -> String {
            format!(
                "impl Fsmclass {{
pub fn is_state_final(&mut self, st: &mut States) -> bool {{
return *st == States::{final_state};
}}
pub fn take_event(&mut self, 
  an_event: &EnumRealInMessagesType,
  st: &mut States,
  business_object: &mut BusinessObject
) -> Result<EnumRealOutMessagesType,String>{{
",
                final_state = build_state_id(&self.get_final_state())
            )
        };
        let impl_fn_take_body = format!(
            "match (*st, an_event){{\n{body}\n}} }}",
            body = body.join("\n")
        );
        /*---------------------------------------------------*/
        let build_possibelinitmsg = |all_inmsgs_in_initial_state: Vec<String>| {
            let vector_content = all_inmsgs_in_initial_state
                .iter()
                .map(|msg| format!("\"{}\"", msg.to_lowercase()))
                .collect::<Vec<String>>();
            return format!(
                "
pub fn get_possible_init_messages(&mut self)->Vec<&'static str>{{
  return vec![{allmsgs}]
}}
",
                allmsgs = vector_content.join(",")
            );
        };
        /*---------------------------------------------------*/
        let getinitialstate = || {
            return format!(
                "
pub fn get_initial_state(&mut self) -> States {{
  return States::St_init;
}}
        "
            );
        };
        /*---------------------------------------------------*/
        return format!(
            "use super::guardclass::Guardclass;
use super::business_class::BusinessObject;
use super::guardconditions::*;
use super::messagesets::*;
            
{states}
#[derive(Debug)]
pub struct Fsmclass {{
pub the_guardobj: Guardclass,
}}
impl Fsmclass {{
    pub fn new(the_guardobj: Guardclass) -> Fsmclass {{
      Fsmclass {{
        the_guardobj: the_guardobj
    }}
  }}
  {getpossibleinicalls}
  {getinitialstate}
}}
{cover}
{body}
}}
",
            states = states,
            cover = cover(),
            body = impl_fn_take_body,
            getpossibleinicalls = build_possibelinitmsg(all_inmsgs_in_initial_state),
            getinitialstate = getinitialstate()
        );
    }
}
