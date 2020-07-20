use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genc_folder::c_gen_gi_h::msg_prod_pattern;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_cnd_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_msg_id;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl Fsm {
    pub fn c_gen_pi_c(&mut self, prepost: &'static str) -> String {
        /*---------------------------------------------------*/
        let one_buildinitpi = |all_outmsgs: BTreeMap<String, Vec<Vec<String>>>| {
            let ret_prodmsg = format!(
                "static {prepost}PI_VTABLE {prepost}prodmsg_vtable ={{
  {},
}};        
",
                all_outmsgs
                    .iter()
                    .map(|outmsg| {
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
            let ret_obj_ini = format!(
                "/*========== init of guard and prod function vtables ==========*/
{prepost}PI * in_{prepost}PI({prepost}PI * self)
{{
self->vtable = &{prepost}prodmsg_vtable;
return self;
}}
",
                prepost = prepost
            );
            return vec![ret_prodmsg, ret_obj_ini].join("\n");
        };
        /*---------------------------------------------------*/
        /*---------------------------------------------------*/
        /*---------------------------------------------------*/
        fn five_build_dummy_prod_body(
            outmsg: &String,
            grp_on_outmsg: &BTreeMap<String, Vec<Vec<String>>>,
        ) -> String {
            /*---------------------------------------------------*/
            //           let build_log =|outmsg: &String| format!("//fprintf(stderr, \"enter prod_{}\\n\");", outmsg);
            /*---------------------------------------------------*/
            let build_declarations = |outmsg: &String| {
                format!(
                    "struct {outmsg}_real *msgout_work_area;
msgout_work_area= self->theMsgFactoryObj->
  alloc_real_outmsg_MSGFACTORY(
  self->theMsgFactoryObj, {outmsg});",
                    outmsg = build_msg_id(&outmsg),
                )
            };
            /*---------------------------------------------------*/
            let build_case_body = || {
                return "// build out msg
msgout_work_area->seconds =rand();
// update BusinessObject
self->the_BusinessObject->mythings +=1;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
"
                .to_string();
            };
            /*---------------------------------------------------*/
            let build_conds =
                |outmsg: &String, grp_on_outmsg: &BTreeMap<String, Vec<Vec<String>>>| {
                  eprintln!("outmsg={}",outmsg);
                    let _vals = grp_on_outmsg.get(outmsg).unwrap();
                    let cases = _vals
                        .iter()
                        .map(|arow| {
                            format!(
                                "case FROM_{acond}:
  /* Original condition value: {usercondvalue} */
  {casebody}
  break;",
                                acond = build_cnd_id(&arow[2]),
                                casebody = build_case_body(),
                                usercondvalue = &arow[5]
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("\n");
                    format!(
                        "switch (given_cond)
  {{
  {cases}
  default:
    /* error wrong cond from guard function */
    snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
      \"wrong guard_condition '%i'  into 'prod_{outmsg}'\",given_cond
    );//log
  
    self->theMsgFactoryObj->
    setup_return_error_MSGFACTORY(self->theMsgFactoryObj,
      self->theMsgFactoryObj->sprint_dest_buff);
    break;
  }}",
                        cases = cases,
                        outmsg = outmsg,
                    )
                };
            /*---------------------------------------------------*/
            let build_body =
                |outmsg:&String,grp_on_outmsg: &BTreeMap<String, Vec<Vec<String>>>| {
                    format!(
                        "
// given_cond = -1;// for verify/test purposes
{buildconds}",
                        buildconds = build_conds(outmsg,grp_on_outmsg)
                    )
                };
            /*---------------------------------------------------*/
            /*---------------------------------------------------*/
            let build_buildoutmsg = |_outmsg: &String| "/* SET UP RETURN */";
            /*---------------------------------------------------*/
            //            {log}
            let ret = format!(
                "{{
{declarations}
{body}
{buildoutmsg}
return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}}",
                declarations = build_declarations(outmsg),
                body = build_body(&outmsg,grp_on_outmsg),
                buildoutmsg = build_buildoutmsg(&"".to_string())
            );

            return ret;
        }
        /*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
        fn four_build_msg_prod_bodies(
            grp_on_outmsg: &BTreeMap<String, Vec<Vec<String>>>,
            prepost: &'static str,
        ) -> String {
            let mut all_prod_bodies: Vec<String> = Vec::new();
            for (outmsg,_) in grp_on_outmsg.iter() {
                all_prod_bodies.push(format!("/*--- prod_{} -------------------- */", outmsg));
                all_prod_bodies.push(msg_prod_pattern(
                    "signature",
                    &outmsg,
            grp_on_outmsg: &BTreeMap<String, Vec<Vec<String>>>,
                    &five_build_dummy_prod_body(&outmsg,grp_on_outmsg),
                    &prepost,
                ));
            }
            return all_prod_bodies.join("\n");
        }
        /*---------------------------------------------------*/
        /*------------ START HERE ---------------------------*/
        /*---------------------------------------------------*/
        let grp_on_outmsg: BTreeMap<String, Vec<Vec<String>>> = self.gen_outmsg_group();
        
        /*++++++++++++++++++++++++++++++++++++++++++++*/
        let two_allprodbodies = four_build_msg_prod_bodies(
            &grp_on_outmsg,
            &prepost,
        );
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
{one_initpi}
/*- INIT Prod Interface -*/
{two_prodbodies}
        ",
            one_initpi = one_buildinitpi(grp_on_outmsg),
            two_prodbodies = two_allprodbodies,
            three_prepost = prepost
        );
        //        return ut;
    }
}
