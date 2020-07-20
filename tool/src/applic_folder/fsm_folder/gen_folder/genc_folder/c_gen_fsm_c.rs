use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_cnd_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_msg_id;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_state_id;

use std::collections::HashSet;

impl Fsm {
    pub fn c_gen_fsm_c(&mut self, prepost: &'static str) -> String {
        /*---------------------------------------------------*/
        fn build_a_cond_test(
            tostate: &String,
            _fromstate: &String,
            _inmsg: &String,
            a_user_cond: &String,
            a_synt_cond: &String,
        ) -> String {
            format!(
                "case {a_synt_cond}:
  /* Original condition value: {a_user_cond} */
  *state = {tostate}; 
  return 1;
  ",
                a_synt_cond = a_synt_cond,
                a_user_cond = a_user_cond,
                tostate = tostate
            )
        };
        /*---------------------------------------------------*/
        fn build_one_inmsg_case(
            inmsg_id: &String,
            smallst: &String,
            smallinmsg: &String,
            all_conds_in_state_and_inmsg: &Vec<String>,
            prepost: &'static str,
        ) -> String {
            return format!(
                "
if (self->theMsgFactoryObj->theInMsg.thetype == {inmsgid})
{{
  how = the_gi->vtable->{prepost}{smallst}_{smallinmsg}(the_gi, &got_Cnd_{smallst}_{smallinmsg}, business_object);
  if (how == 1)
  {{
    {checkconds}
  }} else // how != 1
  {{
    return 0;
  }}
}} 
  ",
                prepost = prepost,
                inmsgid = inmsg_id,
                smallst = smallst,
                smallinmsg = smallinmsg,
                checkconds = all_conds_in_state_and_inmsg.join(""),
            );
        };
        /*---------------------------------------------------*/
        fn build_one_state(state_id: &String, all_inmsgs_in_this_state: &Vec<String>) -> String {
            format!(
                "case {stateid}:
     {allinmsgsinthisstate}
  ",
                stateid = state_id,
                allinmsgsinthisstate = all_inmsgs_in_this_state.join("\n")
            )
        };
        /*---------------------------------------------------*/
        let build_all_states = |allstates| {
            format!(
                "// *state = 123; // for verify error managment
switch (*state)
{{ {allstates} 
// FINAL DEFAULT HERE
default:
snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  \"{prepost}FSM:: This state '%i' is not defined\\n\",*state);
  self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
return 0;
}}
",
                allstates = allstates,
                prepost = prepost
            )
        };
        /*---------------------------------------------------*/
        let build_take_event_impl = |all_cnds_declarations: Vec<(String, String)>| {
            // list of Cnds_{state}_{inmsg} got_Cnd_{state}_{inmsg};
            // inside 'take_event_{UPPER}'
            let all_cnds_declare = all_cnds_declarations
                .iter()
                .map(|(state, inmsg)| {
                    format!(
                        "Cnds_{state}_{inmsg} got_Cnd_{state}_{inmsg};",
                        state = state,
                        inmsg = inmsg
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");
            return format!(
                "
int take_event_{prepost}FSM({prepost}FSM * self, int *state, void *business_object)
{{
{prepost}GI *the_gi;
the_gi = self->the_gi;
int how =0;
{allcndsdeclare}
          ",
                prepost = prepost,
                allcndsdeclare = all_cnds_declare
            );
        };
        /*---------------------------------------------------*/
        let build_init_states = |initstate: &String| {
            return format!(
                "int getinitialstate_{prepost}FSM({prepost}FSM * self)
{{
  (void)(self);
  return {};
}}
",
                build_state_id(initstate),
                prepost = prepost,
            );
        };
        /*---------------------------------------------------*/
        let build_finalstate = |finalstate: &String| {
            return format!(
                "int getfinalstate_{prepost}FSM({prepost}FSM * self)
{{
  (void)(self);
  return {};
}}
",
                build_state_id(finalstate),
                prepost = prepost,
            );
        };
        /*---------------------------------------------------*/
        let build_isinfinalstate = |finalstate: &String| {
            return format!(
                "int isinfinalstate_{prepost}FSM({prepost}FSM * self, int state)
{{
  (void)(self);
  return state == {finalstate} ;
}}
",
                finalstate = build_state_id(finalstate),
                prepost = prepost,
            );
        };
        /*---------------------------------------------------*/
        let build_possibelinitmsg = |all_inmsgs_in_initial_state: Vec<String>| {
            let init_array = all_inmsgs_in_initial_state
                .iter()
                .enumerate()
                .map(|(index, inmsg)| {
                    format!(
                        //                      "inits[{index}] = {inmsg};"
                        "the_init_array.inits[{index}] = {inmsg};",
                        index = index,
                        inmsg = build_msg_id(inmsg)
                    )
                })
                .collect::<Vec<String>>();

            return format!(
                "
INIT_MESSAGES * get_possible_init_messages_{prepost}FSM({prepost}FSM * self)
{{
  static INIT_MESSAGES the_init_array;
  (void)(self);
  {arrayinits}
  the_init_array.the_size_of_inits = {nrmsg};
  return &the_init_array;
}}
",
                nrmsg = all_inmsgs_in_initial_state.len(),
                arrayinits = init_array.join("\n"),
                prepost = prepost,
            );
        };
        /*---------------------------------------------------*/
        let build_publicfunctions =
            |init_state: &String, final_state: &String, all_inmsgs_in_initial_state| {
                return format!(
                    "{initstate}{finalstate}{isinfinalstate}{possibelinitmsg}",
                    initstate = build_init_states(init_state),
                    finalstate = build_finalstate(final_state),
                    isinfinalstate = build_isinfinalstate(final_state),
                    possibelinitmsg = build_possibelinitmsg(all_inmsgs_in_initial_state)
                );
            };
        /*---------------------------------------------------*/
        /*---------------------------------------------------*/
        let mut all_states_names: HashSet<String> = HashSet::new();
        all_states_names.insert(build_state_id(&self.get_final_state()));
        let mut all_state_body: Vec<String> = Vec::new();
        let mut all_cnds_declarations: Vec<(String, String)> = Vec::new();
        let mut all_inmsgs_in_initial_state: Vec<String> = Vec::new();
        for (_state, ref mut _inmsgs) in self.grouped_fsm.iter() {
            let smallst = _state.to_lowercase();
            let state_id = build_state_id(_state);
            all_states_names.insert(build_state_id(&smallst));
            let mut all_inmsgs_in_this_state: Vec<String> = Vec::new();
            for (_inmsg, conds) in _inmsgs.iter() {
                if *_state == self.initial_state {
                    all_inmsgs_in_initial_state.push(_inmsg.clone());
                }
                let smallinmsg = _inmsg.to_lowercase();
                all_cnds_declarations.push((smallst.clone(), smallinmsg.clone()));
                let inmsg_id = build_msg_id(_inmsg);
                let mut all_conds_in_state_and_inmsg: Vec<String> = Vec::new();
                all_conds_in_state_and_inmsg.push(format!(
                    "switch (got_Cnd_{fromstate}_{inmsg}) {{
",
                    fromstate = smallst,
                    inmsg = smallinmsg
                ));

                for (_acond, _outmsgandnextstate) in conds.iter() {
                    let a_user_cond = build_cnd_id(_acond);
                    let a_synt_cond = build_cnd_id(&_outmsgandnextstate.user_guard_result);
                    let tostate = build_state_id(&_outmsgandnextstate.to_state);
                    let code_acond = build_a_cond_test(
                        &tostate,
                        &smallst,
                        &smallinmsg,
                        &a_user_cond,
                        &a_synt_cond,
                    );
                    all_conds_in_state_and_inmsg.push(code_acond);
                }
                all_conds_in_state_and_inmsg.push("}".to_string());
                // one inmsg done
                let one_inmsg = build_one_inmsg_case(
                    &inmsg_id,
                    &smallst,
                    &smallinmsg,
                    &all_conds_in_state_and_inmsg,
                    &prepost,
                );
                all_inmsgs_in_this_state.push(one_inmsg);
            }
            let not_insmg = format!(
                "
snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  \"{prepost}FSM:: This inmessage '%s' is not meaningsful in the state '{state}'\\n\",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;
",
                state = state_id,
                prepost = prepost
            );
            all_inmsgs_in_this_state.push(not_insmg);
            let one_state = build_one_state(&state_id, &all_inmsgs_in_this_state);
            all_state_body.push(one_state);
        }
        let mut take_event_body_impl: Vec<String> = Vec::new();
        let x = build_all_states(all_state_body.join("\n"));
        take_event_body_impl.push(x);
        /////////////////////////////////
        let build_all_enum_states = || {
            let all_states_in_enum = all_states_names
                .iter()
                .map(|st| st.to_string())
                .collect::<Vec<String>>()
                .join(",\n\t");
            format!("enum states {{ {:} }};\n", all_states_in_enum)
        };
        let _includes = vec![
            "#include \"{LOWER}_fsm.h\"",
            "#include \"messages.h\"",
            "#include \"guardconditions_{LOWER}.h\"",
        ];
        let enumstates = build_all_enum_states();
        let build_object_declare = format!(
            "
{prepost}FSM * in_{prepost}FSM({prepost}FSM * self)
{{
  self->mallocated =0;
  self->the_gi = 0;
  self->deleteself_{prepost}FSM = &deleteself_{prepost}FSM;
  self->getinitialstate_{prepost}FSM = &getinitialstate_{prepost}FSM;
  self->getfinalstate_{prepost}FSM = &getfinalstate_{prepost}FSM;
  self->isinfinalstate_{prepost}FSM = &isinfinalstate_{prepost}FSM;
  self->get_possible_init_messages_{prepost}FSM= &get_possible_init_messages_{prepost}FSM;
  self->take_event_{prepost}FSM = &take_event_{prepost}FSM;
  return self;
}}

{prepost}FSM * nw_{prepost}FSM(void)
{{
  {prepost}FSM * self = malloc(sizeof ({prepost}FSM));
  self = in_{prepost}FSM(self);
  self->mallocated =1; // Is in dynamic memory
  return self;
}}

void deleteself_{prepost}FSM({prepost}FSM * self)
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
        let take_event_intro_impl = build_take_event_impl(all_cnds_declarations);
        /////////////////////////////
        return format!(
            "#include \"{prepost}FSM.h\"

{enumstates}
{objdeclare}
{publicfunctions}
{takeeventintroimpl}
{takeeventbodyimpl} 
}}
",
            prepost = prepost,
            enumstates = enumstates,
            objdeclare = build_object_declare,
            takeeventintroimpl = take_event_intro_impl,
            publicfunctions = build_publicfunctions(
                &self.initial_state,
                &self.final_state,
                all_inmsgs_in_initial_state
            ),
            takeeventbodyimpl = take_event_body_impl.join("\n")
        );
    }
}
