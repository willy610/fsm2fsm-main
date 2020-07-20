use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genc_folder::c_gen_gi_h::vtable_pattern;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_cnd_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_msg_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::RowToStateEventGroup;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl Fsm {
    pub fn c_gen_gi_c(&mut self, prepost: &'static str) -> String {
        /*---------------------------------------------------*/
        let build_allprotosvtable = |grp_state_inmsg: &BTreeSet<(String, String)>| {
            let mut all_vtable_signatures: Vec<String> = Vec::new();
            for (state, inmsg) in grp_state_inmsg.iter() {
                all_vtable_signatures.push(vtable_pattern(
                    "signature",
                    &state,
                    &inmsg,
                    &";".to_string(),
                    &prepost,
                ))
            }
            return all_vtable_signatures.join("\n");
        };
        /*---------------------------------------------------*/
        let prodiniguard =
//        |grp_state_inmsg: BTreeMap<(String, String), Vec<RowToStateEventGroup>>| {
          |grp_state_inmsg: &BTreeSet<(String, String)>| {
            let ret_vtable = format!(
                    "
static {prepost}GI_VTABLE {prepost}vtable ={{
{}
}};
",
                    grp_state_inmsg
                        .iter()
                        .map(|(state, inmsg)| {
                            format!(
                                "\t&{prepost}{state}_{inmsg}",
                                state = state.to_lowercase(),
                                inmsg = inmsg.to_lowercase(),
                                prepost = prepost,
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(",\n"),
                    prepost = prepost,
                );
                let ret_obj_ini = format!(
                    "/*========== init of guard and prod function vtables ==========*/
{prepost}GI * in_{prepost}GI({prepost}GI * self)
{{
  self->mallocated =0;
  self->deleteself_{prepost}GI = &deleteself_{prepost}GI;
  self->vtable = &{prepost}vtable;
  return self;
}}
{prepost}GI * nw_{prepost}GI({prepost}GI * self)
{{
  self = malloc(sizeof ({prepost}GI));
  self = in_{prepost}GI(self);
  self->mallocated =1; // Is in dynamic memory
  return self;
}}
void deleteself_{prepost}GI({prepost}GI * self)
{{
  // First delete owned objects 
  // self->otherObj->deleteself(self->otherObj);
  //
  if (self->mallocated)
  {{
    free(self);
  }}
  return;
}}                                      
",
                    prepost = prepost,
                );
                return vec![ret_vtable, ret_obj_ini].join("\n");
            };
        /*---------------------------------------------------*/
        /*---------------------------------------------------*/
        fn build_guard_impl_bodies(
            grp_state_inmsg: &mut BTreeMap<(String, String), Vec<RowToStateEventGroup>>,
            prepost: &'static str,
        ) -> String {
            /*++++++++++++++++++++++++++++++++++++++++++++*/
            let build_one_case = |arow: &RowToStateEventGroup| {
                format!(
                    "case {synt_guard}:
/* Original condition value: {user_guard} */
to_{outmsg} = FROM_{synt_guard};

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->{prepost}prod_{outmsg}(self->theProdObj, to_{outmsg}, business_object);
if (how == 1)
{{
  *ret_choosen_cond = the_in_cond_choice;
}} else
{{
  // content of error is set up in prod_{outmsg} 
}}
break;",
                    outmsg = arow.out_msg.to_lowercase(),
                    synt_guard = build_cnd_id(&arow.synt_guard_result),
                    prepost = prepost,
                    user_guard = arow.user_guard_result
                )
            };
            /*++++++++++++++++++++++++++++++++++++++++++++*/
            let build_one_random_case = |synt_guard, index, user_guard| {
                format!(
                    "case {index}:
  /* Original condition value: {user_guard} */
  the_in_cond_choice = {synt_guard};
  break;",
                    synt_guard = synt_guard,
                    index = index,
                    user_guard = user_guard
                )
            };
            let mut all_guard_bodies: Vec<String> = Vec::new();
            // state,inmsg->[state,inmsg,user_guard,out_msg,to_state]
            for ((state, inmsg), rows) in grp_state_inmsg.iter() {
                all_guard_bodies.push(format!("/*--- {}_{} -------------------- */", state, inmsg));
                let mut all_outmsgs_in_this_state: BTreeSet<String> = BTreeSet::new();
                for arow in rows.iter() {
                    all_outmsgs_in_this_state.insert(arow.out_msg.clone());
                }
                let dcl_tomsg = all_outmsgs_in_this_state
                    .iter()
                    .map(|outmsg| {
                        format!(
                            "TOMSG_{outmsg} to_{outmsg};",
                            outmsg = outmsg.to_lowercase()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                let mut all_cases: Vec<String> = Vec::new();
                let mut all_cases_for_random_choice: Vec<String> = Vec::new();
                for (index, arow) in rows.iter().enumerate() {
                    all_cases.push(build_one_case(&arow));
                    all_cases_for_random_choice.push(build_one_random_case(
                        build_cnd_id(&arow.synt_guard_result),
                        index,
                        arow.user_guard_result.clone(),
                    ));
                }
                let the_in_cond_choice = format!(
                    "Cnds_{state}_{inmsg}",
                    state = state.to_lowercase(),
                    inmsg = inmsg.to_lowercase()
                );
                let one_body = format!(
                    "
{{
{cndsstateinmsg} the_in_cond_choice = 0;// avoid warnings
{dcltomsg}
struct {nameinmsg}_real * a_{inmsg}_real = 0;// the inmessage. (avoid unused warning)
a_{inmsg}_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = {nrchoices};
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {{
{allrandomcases}
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    \"{prepost}::Cnds_{state}_{inmsg} out of range. %i\\n\", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {{
{allcases}
}}
return how;
}}
                ",
                    prepost = prepost,
                    cndsstateinmsg = the_in_cond_choice,
                    dcltomsg = dcl_tomsg,
                    state = state.to_lowercase(),
                    nrchoices = all_cases.len() - 0,
                    inmsg = inmsg.to_lowercase(),
                    allcases = all_cases.join("\n"),
                    nameinmsg = build_msg_id(inmsg),
                    allrandomcases = all_cases_for_random_choice.join("\n"),
                );
                all_guard_bodies.push(vtable_pattern(
                    "",
                    &state.to_lowercase(),
                    &inmsg.to_lowercase(),
                    &one_body,
                    &prepost,
                ));
            }
            return all_guard_bodies.join("\n");
        }
        /*---------------------------------------------------*/
        /*------------ START HERE ---------------------------*/
        /*---------------------------------------------------*/
        let mut grp_state_inmsg: BTreeMap<(String, String), Vec<RowToStateEventGroup>> =
            self.gen_state_event_group();
        let mut all_outmsgs: BTreeSet<String> = BTreeSet::new();
        let mut all_st_inmsg: BTreeSet<(String, String)> = BTreeSet::new();
        for ((_state, _inmsg), rows) in grp_state_inmsg.iter() {
            all_st_inmsg.insert((_state.to_lowercase(), _inmsg.to_lowercase()));
            for arow in rows.iter() {
                all_outmsgs.insert(arow.out_msg.to_lowercase());
            }
        }
        /*++++++++++++++++++++++++++++++++++++++++++++*/
        /*++++++++++++++++++++++++++++++++++++++++++++*/
        let allguardbodies = build_guard_impl_bodies(&mut grp_state_inmsg, prepost);
        return format!(
            "#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include \"{prepost}GI.h\"

/*- INIT Guard Interface -*/
{allprotosvtable}
{initguard}
/*======== guard ***() Implementations ========*/
/* NOTE
Peek into the inmessage and your own business_object
in order to assign a choice to 'the_in_cond_choice'
if (self->inmessage ...)
if (business_object-> ...)

Here (in test/verify) we do a random choice

*/
/*=============================================*/
{allguardbodies}
",
            prepost = prepost,
            allprotosvtable = build_allprotosvtable(&all_st_inmsg),
            initguard = prodiniguard(&all_st_inmsg),
            allguardbodies = allguardbodies,
        );
    }
}
