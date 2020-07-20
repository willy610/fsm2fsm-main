use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genc_folder::c_gen_gi_h::msg_prod_pattern;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_cnd_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_msg_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::RowToOutmsgGroup;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl Fsm {
    pub fn c_gen_pi_c(&mut self, prepost: &'static str) -> String {
        /*---------------------------------------------------*/
        /*------------ START HERE ---------------------------*/
        /*---------------------------------------------------*/
        let grp_on_outmsg: BTreeMap<String, Vec<RowToOutmsgGroup>> = self.gen_outmsg_group();
        /*---------------------------------------------------*/
        let build_outmsgdeclarations = |one_outmsg: &String| {
            format!(
                "
struct {outmsg_id}_real *a_{outmsg}_out_real;// a_{outmsg}_out_real  msgout_work_area
a_{outmsg}_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, {outmsg_id});",
                outmsg = one_outmsg.to_lowercase(),
                outmsg_id = build_msg_id(&one_outmsg),
            )
        };
        /*---------------------------------------------------*/
        /*
                let build_default = |one_outmsg: &String| {
                    return format!(
                        "
        default:
            /* error wrong cond from guard function */
            snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
              \"wrong guard_condition '%i'  into 'prod_{one_outmsg}'\",given_cond
            );//log

            self->theMsgFactoryObj->
            setup_return_error_MSGFACTORY(self->theMsgFactoryObj,
              self->theMsgFactoryObj->sprint_dest_buff);
            break;
                    ",
                        one_outmsg = one_outmsg
                    );
                };
                */
        /*---------------------------------------------------*/
        let build_inmsgdeclarations = |rows: &Vec<RowToOutmsgGroup>| -> String {
            let mut all_in_messages_4_this_prodmsg: BTreeSet<String> = BTreeSet::new();
            for a_row in rows {
                all_in_messages_4_this_prodmsg.insert(a_row.in_msg.to_lowercase());
            }
            all_in_messages_4_this_prodmsg
                .iter()
                .map(|an_imsg| {
                    format!(
                        "struct Msg_{inmsg}_in_real * a_{inmsg}_in_real = 0;// avoid unused",
                        inmsg = an_imsg.to_lowercase()
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        };
        /*---------------------------------------------------*/
        let build_all_cond = |rows: &Vec<RowToOutmsgGroup>| -> String {
            return rows
                .iter()
                .map(|arow| {
                    format!(
                        "case FROM_{synt_cond}:
/* Original condition value: {user_cond} */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_{inmsg}_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;",
                        synt_cond = build_cnd_id(&arow.synt_guard_result),
                        user_cond = &arow.user_guard_result,
                        inmsg = &arow.in_msg.to_lowercase()
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");
        };
        /*---------------------------------------------------*/
        let six_build_one_prod =
            |prepost: &'static str, one_outmsg, rows: &Vec<RowToOutmsgGroup>| -> String {
                /*
                {header}
                {signature}
                {declarations}
                {body}
                return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
                */

                return format!(
                    "
/*--- prod_{outmsg} -------------------- */
{signature}
{outmsgdeclarations}
{inmsgdeclarations}
switch (given_cond)
  {{
    {allfromcond}
  }}// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}}// end one method",
                    outmsg = one_outmsg,
                    signature =
                        msg_prod_pattern("signature", &one_outmsg, &"{".to_string(), &prepost),
                    outmsgdeclarations = build_outmsgdeclarations(&one_outmsg),
                    inmsgdeclarations = build_inmsgdeclarations(rows),
                    allfromcond = build_all_cond(rows),
                );
            };
        /*---------------------------------------------------*/
        let four_build_msg_prod_bodies = |prepost: &'static str| -> String {
            let mut all_prod_bodies: Vec<String> = Vec::new();
            for (outmsg, rows) in grp_on_outmsg.iter() {
                let x = six_build_one_prod(prepost, outmsg.clone(), rows);
                all_prod_bodies.push(x);
            }

            return all_prod_bodies.join("\n");
        };
        /*---------------------------------------------------*/
        let one_buildinitpi = || {
            let ret_prodmsg = format!(
                "// ALL STATIC
                static {prepost}PI_VTABLE {prepost}prodmsg_vtable ={{
{},
}};        
",
                grp_on_outmsg
                    .iter()
                    .map(|(outmsg, _)| {
                        format!(
                            "&{prepost}prod_{outmsg}",
                            prepost = prepost,
                            outmsg = outmsg
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(",\n\t"),
                prepost = prepost
            );
            /*---------------------------------------------------*/
            let ret_obj_ini = format!(
                "/*========== init of guard and prod function vtables ==========*/
{prepost}PI * in_{prepost}PI({prepost}PI * self)
{{
self->mallocated =0;
self->deleteself_{prepost}PI = &deleteself_{prepost}PI;
self->vtable = &{prepost}prodmsg_vtable;
return self;
}}
//{prepost}PI * nw_{prepost}PI({prepost}PI * self)
{prepost}PI * nw_{prepost}PI(void)
{{
//  self = malloc(sizeof ({prepost}PI));
  {prepost}PI * self = malloc(sizeof ({prepost}PI));
  self = in_{prepost}PI(self);
  self->mallocated =1; // Is in dynamic memory
  return self;
}}
void deleteself_{prepost}PI({prepost}PI * self)
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
                prepost = prepost
            );
            return vec![ret_prodmsg, ret_obj_ini].join("\n");
        };
        /*++++++++++++++++++++++++++++++++++++++++++++*/
        /*---------------------------------------------------*/
        let build_allprotosprod = || {
            let mut all_prodmsg_signatures: Vec<String> = Vec::new();
            for (outmsgname, _vals) in grp_on_outmsg.iter() {
                all_prodmsg_signatures.push(msg_prod_pattern(
                    "signature",
                    &outmsgname,
                    &";".to_string(),
                    &prepost,
                ));
            }
            all_prodmsg_signatures.join("\n")
        };
        /*++++++++++++++++++++++++++++++++++++++++++++*/
        return format!(
            "#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include \"{three_prepost}PI.h\"

/*===================================================================*/
/*- INIT Prod Interface -*/
// For each prod method:
// In case of failure then back out own object
// Back out updates to the_BO_* 
// and return an error with:
//  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
//    self->theMsgFactoryObj,
//    self->theMsgFactoryObj->sprint_dest_buff);
//
{allprotosprod}
{one_initpi}
/*- INIT Prod Interface -*/
{four_prodbodies}
        ",
            allprotosprod = build_allprotosprod(),
            one_initpi = one_buildinitpi(),
            four_prodbodies = four_build_msg_prod_bodies(&prepost),
            three_prepost = prepost
        );
    }
}
