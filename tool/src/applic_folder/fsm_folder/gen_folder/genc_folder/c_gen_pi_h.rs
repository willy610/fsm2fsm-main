use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genc_folder::c_gen_gi_h::msg_prod_pattern;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_cnd_id;

impl Fsm {
    pub fn c_gen_pi_h(&mut self, prepost: &'static str) -> String {
        /*---------------------------------------------------*/
        /*------------ START HERE ---------------------------*/
        /*---------------------------------------------------*/
        let grp_on_outmsg = self.gen_outmsg_group();
        /*---------------------------------------------------*/
        let build_enumsforcond2prodmsg = || {
            let mut ut: Vec<String> = Vec::new();

            for (outmsgname, vals) in grp_on_outmsg.iter() {
                let state_outmsg = vals
                    .iter()
                    .map(|arow| format!("FROM_{}", build_cnd_id(&arow.synt_guard_result)))
                    .collect::<Vec<String>>()
                    .join(",\n");

                ut.push(format!(
                    "typedef enum {{ {stateoutmsg} \n}}TOMSG_{enumname} ;",
                    enumname = outmsgname,
                    stateoutmsg = state_outmsg
                ));
            }
            return ut.join("\n");
        };
        /*---------------------------------------------------*/
        let build_alltypdefsprods = || {
            let mut all_prodmsg_typedefs: Vec<String> = Vec::new();
            for (outmsgname, _vals) in grp_on_outmsg.iter() {
                all_prodmsg_typedefs.push(msg_prod_pattern(
                    "asptr",
                    &outmsgname,
                    &";".to_string(),
                    &prepost,
                ));
            }
            all_prodmsg_typedefs.join("\n")
        };
        /*---------------------------------------------------*/
        /*
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
        */
        /*---------------------------------------------------*/
        let ret = format!(
            "#include \"MsgFactory.h\"
#include \"{prepost}GuardConditions.h\"
#include \"{prepost}BusinessObject.h\"

#ifndef {prepost}PI_H
#define {prepost}PI_H

{enumsforcond2prodmsg}

typedef struct {prepost}PI {prepost}PI;
void deleteself_{prepost}PI({prepost}PI * self);
typedef struct {{ {alltypdefsprods} }}{prepost}PI_VTABLE;
struct {prepost}PI {{
int mallocated;
int something_in_PI;
void (*deleteself_{prepost}PI)({prepost}PI * self);

MSGFACTORY * theMsgFactoryObj;
{prepost}PI_VTABLE *vtable;
}};
{prepost}PI * in_{prepost}PI({prepost}PI * self);
//{prepost}PI * nw_{prepost}PI({prepost}PI * self);
{prepost}PI * nw_{prepost}PI(void);
{allprotosprod}
#endif /* {prepost}PI_H */
",
            enumsforcond2prodmsg = build_enumsforcond2prodmsg(),
            alltypdefsprods = build_alltypdefsprods(),
            //            allprotosprod = build_allprotosprod(),
            allprotosprod = "// MOVED TO BODY ",
            prepost = prepost
        );
        return ret;
    }
}
