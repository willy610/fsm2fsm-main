use crate::applic_folder::fsm_folder::fsm::Fsm;
impl Fsm {
    pub fn c_gen_fsm_h(&mut self, prepost: &'static str) -> String {
        let all_inmsgs_in_initial_state: Vec<String> = Vec::new();

        format!(
            "
#ifndef {prepost}FSM_H
#define {prepost}FSM_H
#include <stdio.h>
#include <stdlib.h>

#include \"{prepost}GI.h\"

typedef struct {prepost}FSM {prepost}FSM;

struct init_messages
{{
  int the_size_of_inits;
  MESSAGETYPES inits[{nrinits}];
}};
typedef struct init_messages INIT_MESSAGES;

void deleteself_{prepost}FSM({prepost}FSM * self);

int getinitialstate_{prepost}FSM({prepost}FSM * self);
int getfinalstate_{prepost}FSM({prepost}FSM * self);
int isinfinalstate_{prepost}FSM({prepost}FSM * self, int state);
INIT_MESSAGES * get_possible_init_messages_{prepost}FSM({prepost}FSM * self);
int take_event_{prepost}FSM({prepost}FSM * self, int *state, void *business_object);

struct {prepost}FSM {{
  int mallocated;
  MSGFACTORY * theMsgFactoryObj;
  {prepost}GI *the_gi;
  void (*deleteself_{prepost}FSM)({prepost}FSM * self);
  int (*getinitialstate_{prepost}FSM)({prepost}FSM * self);
  int (*getfinalstate_{prepost}FSM)({prepost}FSM * self);
  int (*isinfinalstate_{prepost}FSM)({prepost}FSM * self, int state);
  INIT_MESSAGES * (*get_possible_init_messages_{prepost}FSM)({prepost}FSM * self);
  int (*take_event_{prepost}FSM)({prepost}FSM * self, int *state, void *business_object);
}};

{prepost}FSM * in_{prepost}FSM({prepost}FSM * self);
{prepost}FSM * nw_{prepost}FSM(void);
#endif /* {prepost}FSM_H */
",
            prepost = prepost,
            nrinits = 1 + all_inmsgs_in_initial_state.len()
        )
    }
}
