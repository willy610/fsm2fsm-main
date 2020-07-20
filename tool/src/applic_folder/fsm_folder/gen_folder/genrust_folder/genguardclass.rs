use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_cnd_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::RowToStateEventGroup;

impl Fsm {
    pub fn genguardclass(
        &mut self,
        _use_lower: &'static str,
        _use_upper: &'static str,
        _no_mess: &String,
    ) -> String {
        let mut _grp_state_event = self.gen_state_event_group();
        let mut each_function: Vec<String> = Vec::new();
        //////////////////////////////
        fn build_test_match(
            state: &String,
            inmsg: &String,
            rows: &Vec<RowToStateEventGroup>,
        ) -> String {
            //////////////////////////////
            let testvals = |rows: &Vec<RowToStateEventGroup>| -> String {
                rows.iter()
                    .enumerate()
                    .map(|(index, arow)| {
                        //    cond_to_set = Conds_{state}_{inmsg}::Cnd_{state}_{inmsg}_{condenum}; }}",

                        format!(
                            "{index}=>{{
    /* User Guard is: {user_guard} */
    cond_to_set = Conds_{state}_{inmsg}::{synt_guard}; }}",
                            index = index,
                            state = state,
                            inmsg = inmsg,
                            //              condenum=index+1,
                            user_guard = &arow.user_guard_result,
                            synt_guard = build_cnd_id(&arow.synt_guard_result)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(",\n\t")
            };
            return format!("// TEST/VERIFY STARTS
// Read NOTE on test / verify above
let i = self.myrand.in_lim({randmax});
match i {{
{testvals}
_=>{{return Result::Err(\"Could not make a proper decision in 'guard_{state}_{inmsg}' \".to_string());}}
}}
// TEST/VERIFY ENDS"
,
state=state,
inmsg=inmsg,
testvals=testvals(rows),
randmax=rows.len()
);
        }
        //////////////////////////////
        fn build_all_conds(
            state: &String,
            inmsg: &String,
            rows: &Vec<RowToStateEventGroup>,
        ) -> String {
            let mut each_conds_enumarated: Vec<String> = Vec::new();

            for arow in rows.iter() {
                let acond = build_cnd_id(&arow.synt_guard_result);
                let outmsg = &arow.out_msg.to_lowercase();
                each_conds_enumarated.push(format!(
                    "Conds_{state}_{inmsg}::{acond}=>{{
    match self.the_prodmsgobj.prod_{outmsg}(
      &OutMsg_{outmsg}::{acond},
      business_object,
      _inmsg
  ) {{
      Ok(msg_out) => {{
          return Result::Ok((Conds_{state}_{inmsg}::{acond}, msg_out));
      }}
      Err(txt) => {{
          return Result::Err(format!(
              \"Error ({{}}) from message producer(prod_{outmsg}) \",txt));
      }}
  }}
}}
",
                    state = state,
                    inmsg = inmsg,
                    acond = acond,
                    outmsg = outmsg
                ));
            }
            each_conds_enumarated.join(",")
        }
        //////////////////////////////
        fn new_build_all_cond_choices(
            state: &String,
            inmsg: &String,
            rows: &Vec<RowToStateEventGroup>,
        ) -> String {
            return format!(
                "let cond_to_set ;
{test_match}
match cond_to_set{{
  {all_conds}
}}
      ",
                test_match = build_test_match(state, inmsg, rows),
                all_conds = build_all_conds(state, inmsg, rows)
            );
        }
        //////////////////////////////
        let mut build_all_prd_msg_funcs = || {
            for ((_state, _event), _rows) in _grp_state_event.iter() {
                let state_as_lower = _state.to_lowercase();
                let event_as_lower = _event.to_lowercase();
                ////
                let fnheader = format!(
                    "///////////////////////////  {state}_{event}  /////////
pub fn {state}_{event} (&mut self,_inmsg:&EnumRealInMessagesType,
  business_object: &mut BusinessObject
) 
-> Result<(Conds_{state}_{event},EnumRealOutMessagesType),String> {{",
                    state = state_as_lower,
                    event = event_as_lower
                );
                //////////////////////////////
                let allcondchoices =
                    new_build_all_cond_choices(&state_as_lower, &event_as_lower, _rows);
                each_function.push(format!(
                    "{fnheader}
    if let EnumRealInMessagesType::Msg_{inmsglower}(ref _real_msg_{inmsglower}_type) = _inmsg
    {{ 
    {allcondchoices}
    }}
    else
    {{
      return Result::Err(\"Wrong message type into guard_{state_as_lower}_{event_as_lower}' \".to_string()); 
    }}
  }}",
state_as_lower = state_as_lower,
event_as_lower = event_as_lower,
                    fnheader = fnheader,
                    inmsglower=event_as_lower,
                    allcondchoices = allcondchoices
                ));
            }
            return each_function.join("\n");
        };

        return format!(
            "use super::prodmsgclass::Prodmsgclass;
use super::guardconditions::*;
use super::business_class::BusinessObject;
use super::prodconditions::*;
use super::messagesets::EnumRealOutMessagesType;
use super::messagesets::EnumRealInMessagesType;
use super::super::shared_folder::myrand::Myrand;

#[derive(Debug)]
pub struct Guardclass {{
    pub the_prodmsgobj: Prodmsgclass,
    pub myrand: Myrand,
}}
impl Guardclass {{
    pub fn new(
      the_prodmsgobj: Prodmsgclass,_seed: u16
    ) -> Guardclass {{
      Guardclass {{
          the_prodmsgobj: the_prodmsgobj,
          myrand: Myrand::new(_seed)
        }}
    }}
/*
Note on verify / test.
NOTE
====
Do a random decision here in TEST
Normally to make a decision
A. Peek into inmessage
B. Consult business object attribute 'something' via
 business_object.something
C. assign cond to 'cond_to_set'

*/
{all_guard_state_event_funcs}
}}",
            all_guard_state_event_funcs = build_all_prd_msg_funcs()
        );
    }
}
