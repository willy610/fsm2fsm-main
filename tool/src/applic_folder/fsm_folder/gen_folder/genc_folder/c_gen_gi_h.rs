use crate::applic_folder::fsm_folder::fsm::Fsm;
use std::collections::BTreeSet;

/*---------------------------------------------------*/
pub fn vtable_pattern(
    how: &'static str,
    state: &String,
    inmsg: &String,
    semicolon_or_body: &String,
    prepost: &'static str,
) -> String {
    let pref: String;
    if how == "asptr" {
        // int (*state)
        pref = format!(
            "int (*{prepost}{state}_{inmsg})",
            prepost = prepost,
            state = state,
            inmsg = inmsg
        );
    } else {
        // static int state
        pref = format!(
            "static int {prepost}{state}_{inmsg}",
            prepost = prepost,
            state = state,
            inmsg = inmsg
        );
    }
    let ret = format!(
        "{pref}({prepost}GI* self, Cnds_{state}_{inmsg} *ret_choosen_cond, {prepost}BusinessObject *business_object){semicolon}
    ",
        prepost = prepost,
        pref = pref,
        state = state,
        inmsg = inmsg,
        semicolon = semicolon_or_body
    );
    return ret;
}
/*---------------------------------------------------*/
pub fn msg_prod_pattern(
    how: &'static str,
    outmsg: &String,
    semicolon_or_body: &String,
    prepost: &'static str,
) -> String {
    let pref: String;
    if how == "asptr" {
        // int (*state)
        pref = format!(
            "int (*{prepost}prod_{outmsg})",
            prepost = prepost,
            outmsg = outmsg
        );
    } else {
        // static int state
        pref = format!(
            "static int {prepost}prod_{outmsg}",
            prepost = prepost,
            outmsg = outmsg
        );
    }

    let ret = format!(
        "
{pref}({prepost}PI * self,
TOMSG_{outmsg} given_cond, {prepost}BusinessObject *business_object){semicolon}",
        pref = pref,
        outmsg = outmsg,
        semicolon = semicolon_or_body,
        prepost = prepost
    );
    return ret;
}
/*---------------------------------------------------*/

impl Fsm {
    pub fn c_gen_gi_h(&mut self, prepost: &'static str) -> String {
        /*---------------------------------------------------*/
        /*---------------------------------------------------*/
        fn build_vtable(
            all_st_inmsg: BTreeSet<(String, String)>,
            all_outmsgs: BTreeSet<String>,
            all_vtable_typedefs: &mut Vec<String>,
            all_prodmsg_typedefs: &mut Vec<String>,
            all_vtable_signatures: &mut Vec<String>,
            all_prodmsg_signatures: &mut Vec<String>,
            prepost: &'static str,
        ) {
            for (state, inmsg) in all_st_inmsg.iter() {
                all_vtable_typedefs.push(vtable_pattern(
                    "asptr",
                    &state,
                    &inmsg,
                    &";".to_string(),
                    &prepost,
                ));
                all_vtable_signatures.push(vtable_pattern(
                    "signature",
                    &state,
                    &inmsg,
                    &";".to_string(),
                    &prepost,
                ))
            }
            for outmsg in all_outmsgs.iter() {
                all_prodmsg_typedefs.push(msg_prod_pattern(
                    "asptr",
                    &outmsg,
                    &";".to_string(),
                    &prepost,
                ));
                all_prodmsg_signatures.push(msg_prod_pattern(
                    "signature",
                    &outmsg,
                    &";".to_string(),
                    &prepost,
                ))
            }
        }
        /*---------------------------------------------------*/
        /*------------ START HERE ---------------------------*/
        /*---------------------------------------------------*/
        let mut _grp_state_inmsg = self.gen_state_event_group();
        let _grp_on_outmsg = self.gen_outmsg_group();

        let mut all_st_inmsg: BTreeSet<(String, String)> = BTreeSet::new();
        let mut all_outmsgs: BTreeSet<String> = BTreeSet::new();

        let mut all_vtable_typedefs: Vec<String> = Vec::new();
        let mut all_prodmsg_typedefs: Vec<String> = Vec::new();
        let mut all_vtable_signatures: Vec<String> = Vec::new();
        let mut all_prodmsg_signatures: Vec<String> = Vec::new();

        for ((_state, _inmsg), rows) in _grp_state_inmsg.iter() {
            all_st_inmsg.insert((_state.to_lowercase(), _inmsg.to_lowercase()));
            for arow in rows.iter() {
                all_outmsgs.insert(arow.out_msg.to_lowercase());
            }
        }
        build_vtable(
            all_st_inmsg,
            all_outmsgs,
            &mut all_vtable_typedefs,
            &mut all_prodmsg_typedefs,
            &mut all_vtable_signatures,
            &mut all_prodmsg_signatures,
            prepost,
        );
        //  eprintln!("{}",all_vtable_typedefs.join("\n"));
        let ret = format!(
            "#ifndef {prepost}GI_H
#define {prepost}GI_H
   
#include \"{prepost}PI.h\"
#include \"{prepost}BusinessObject.h\"

typedef struct {prepost}GI {prepost}GI;

void deleteself_{prepost}GI({prepost}GI * self);

typedef struct {{ {alltypdefsvtable} }}{prepost}GI_VTABLE;

struct {prepost}GI {{
  int mallocated;
  int something_in_GI;
  void (*deleteself_{prepost}GI)({prepost}GI * self);

  MSGFACTORY * theMsgFactoryObj;

  {prepost}GI_VTABLE *vtable;
  {prepost}PI *theProdObj;
}};
{prepost}GI * in_{prepost}GI({prepost}GI * self);
{prepost}GI * nw_{prepost}GI({prepost}GI * self);

{allprotosvtable}

#endif /* {prepost}GI_H */
",
            //            UPPER = _use_upper,
            //            LOWER = _use_lower,
            prepost = prepost,
            alltypdefsvtable = all_vtable_typedefs.join(""),
            //            allprotosvtable = all_vtable_signatures.join(""),
            allprotosvtable = "// MOVED TO BODY",
        );
        return ret;
    }
}
