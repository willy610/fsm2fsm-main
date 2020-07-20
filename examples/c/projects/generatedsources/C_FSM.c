#include "C_FSM.h"

enum states { St_keyexchage_24,
	St_normal_well,
	St_awaitconfirm_other,
	St_keyexchage_42,
	St_awaitconfirm_no,
	St_normal_hm,
	St_awaitconfirm_yes,
	St_startup_how_are_you,
	St_init_hello_alice,
	St_init,
	St_init_ping,
	St_final };


C_FSM * in_C_FSM(C_FSM * self)
{
  self->mallocated =0;
  self->the_gi = 0;
  self->deleteself_C_FSM = &deleteself_C_FSM;
  self->getinitialstate_C_FSM = &getinitialstate_C_FSM;
  self->getfinalstate_C_FSM = &getfinalstate_C_FSM;
  self->isinfinalstate_C_FSM = &isinfinalstate_C_FSM;
  self->get_possible_init_messages_C_FSM= &get_possible_init_messages_C_FSM;
  self->take_event_C_FSM = &take_event_C_FSM;
  return self;
}

C_FSM * nw_C_FSM(void)
{
  C_FSM * self = malloc(sizeof (C_FSM));
  self = in_C_FSM(self);
  self->mallocated =1; // Is in dynamic memory
  return self;
}

void deleteself_C_FSM(C_FSM * self)
{
  // First delete owned objects 
  // self->otherObj->deleteself(self->otherObj);
  //
  if (self->mallocated)
  {
    free(self);
  }
  return;
}

int getinitialstate_C_FSM(C_FSM * self)
{
  (void)(self);
  return St_init;
}
int getfinalstate_C_FSM(C_FSM * self)
{
  (void)(self);
  return St_final;
}
int isinfinalstate_C_FSM(C_FSM * self, int state)
{
  (void)(self);
  return state == St_final ;
}

INIT_MESSAGES * get_possible_init_messages_C_FSM(C_FSM * self)
{
  static INIT_MESSAGES the_init_array;
  (void)(self);
  the_init_array.inits[0] = Msg_callin;
  the_init_array.the_size_of_inits = 1;
  return &the_init_array;
}


int take_event_C_FSM(C_FSM * self, int *state, void *business_object)
{
C_GI *the_gi;
the_gi = self->the_gi;
int how =0;
Cnds_awaitconfirm_no_what got_Cnd_awaitconfirm_no_what;
Cnds_awaitconfirm_other_bye got_Cnd_awaitconfirm_other_bye;
Cnds_awaitconfirm_yes_blablabla got_Cnd_awaitconfirm_yes_blablabla;
Cnds_init_callin got_Cnd_init_callin;
Cnds_init_hello_alice_dontdisturb got_Cnd_init_hello_alice_dontdisturb;
Cnds_init_hello_alice_hello_bob got_Cnd_init_hello_alice_hello_bob;
Cnds_init_ping_pong got_Cnd_init_ping_pong;
Cnds_init_ping_sorry got_Cnd_init_ping_sorry;
Cnds_keyexchage_24_questionwas got_Cnd_keyexchage_24_questionwas;
Cnds_keyexchage_42_43 got_Cnd_keyexchage_42_43;
Cnds_normal_hm_blablabla got_Cnd_normal_hm_blablabla;
Cnds_normal_hm_or got_Cnd_normal_hm_or;
Cnds_normal_well_blablabla got_Cnd_normal_well_blablabla;
Cnds_normal_well_bye_bob got_Cnd_normal_well_bye_bob;
Cnds_startup_how_are_you_rich got_Cnd_startup_how_are_you_rich;
Cnds_startup_how_are_you_tired got_Cnd_startup_how_are_you_tired;
          
// *state = 123; // for verify error managment
switch (*state)
{ case St_awaitconfirm_no:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_what)
{
  how = the_gi->vtable->C_awaitconfirm_no_what(the_gi, &got_Cnd_awaitconfirm_no_what, business_object);
  if (how == 1)
  {
    switch (got_Cnd_awaitconfirm_no_what) {
case Cnd_awaitconfirm_no_what_1:
  /* Original condition value: Cnd_awaitconfirm_no_what_25 */
  *state = St_awaitconfirm_yes; 
  return 1;
  case Cnd_awaitconfirm_no_what_2:
  /* Original condition value: Cnd_awaitconfirm_no_what_26 */
  *state = St_awaitconfirm_no; 
  return 1;
  case Cnd_awaitconfirm_no_what_3:
  /* Original condition value: Cnd_awaitconfirm_no_what_27 */
  *state = St_awaitconfirm_other; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_awaitconfirm_no'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_awaitconfirm_other:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_bye)
{
  how = the_gi->vtable->C_awaitconfirm_other_bye(the_gi, &got_Cnd_awaitconfirm_other_bye, business_object);
  if (how == 1)
  {
    switch (got_Cnd_awaitconfirm_other_bye) {
case Cnd_awaitconfirm_other_bye_1:
  /* Original condition value: Cnd_awaitconfirm_other_bye_18 */
  *state = St_final; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_awaitconfirm_other'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_awaitconfirm_yes:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_blablabla)
{
  how = the_gi->vtable->C_awaitconfirm_yes_blablabla(the_gi, &got_Cnd_awaitconfirm_yes_blablabla, business_object);
  if (how == 1)
  {
    switch (got_Cnd_awaitconfirm_yes_blablabla) {
case Cnd_awaitconfirm_yes_blablabla_1:
  /* Original condition value: Cnd_awaitconfirm_yes_blablabla_20 */
  *state = St_normal_well; 
  return 1;
  case Cnd_awaitconfirm_yes_blablabla_2:
  /* Original condition value: Cnd_awaitconfirm_yes_blablabla_24 */
  *state = St_normal_hm; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_awaitconfirm_yes'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_init:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_callin)
{
  how = the_gi->vtable->C_init_callin(the_gi, &got_Cnd_init_callin, business_object);
  if (how == 1)
  {
    switch (got_Cnd_init_callin) {
case Cnd_init_callin_1:
  /* Original condition value: Cnd_init_callin_21 */
  *state = St_init_hello_alice; 
  return 1;
  case Cnd_init_callin_2:
  /* Original condition value: Cnd_init_callin_6 */
  *state = St_init_ping; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_init'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_init_hello_alice:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_dontdisturb)
{
  how = the_gi->vtable->C_init_hello_alice_dontdisturb(the_gi, &got_Cnd_init_hello_alice_dontdisturb, business_object);
  if (how == 1)
  {
    switch (got_Cnd_init_hello_alice_dontdisturb) {
case Cnd_init_hello_alice_dontdisturb_1:
  /* Original condition value: Cnd_init_hello_alice_dontdisturb_13 */
  *state = St_final; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

if (self->theMsgFactoryObj->theInMsg.thetype == Msg_hello_bob)
{
  how = the_gi->vtable->C_init_hello_alice_hello_bob(the_gi, &got_Cnd_init_hello_alice_hello_bob, business_object);
  if (how == 1)
  {
    switch (got_Cnd_init_hello_alice_hello_bob) {
case Cnd_init_hello_alice_hello_bob_1:
  /* Original condition value: Cnd_init_hello_alice_hello_bob_1 */
  *state = St_startup_how_are_you; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_init_hello_alice'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_init_ping:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_pong)
{
  how = the_gi->vtable->C_init_ping_pong(the_gi, &got_Cnd_init_ping_pong, business_object);
  if (how == 1)
  {
    switch (got_Cnd_init_ping_pong) {
case Cnd_init_ping_pong_1:
  /* Original condition value: Cnd_init_ping_pong_15 */
  *state = St_startup_how_are_you; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

if (self->theMsgFactoryObj->theInMsg.thetype == Msg_sorry)
{
  how = the_gi->vtable->C_init_ping_sorry(the_gi, &got_Cnd_init_ping_sorry, business_object);
  if (how == 1)
  {
    switch (got_Cnd_init_ping_sorry) {
case Cnd_init_ping_sorry_1:
  /* Original condition value: Cnd_init_ping_sorry_14 */
  *state = St_final; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_init_ping'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_keyexchage_24:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_questionwas)
{
  how = the_gi->vtable->C_keyexchage_24_questionwas(the_gi, &got_Cnd_keyexchage_24_questionwas, business_object);
  if (how == 1)
  {
    switch (got_Cnd_keyexchage_24_questionwas) {
case Cnd_keyexchage_24_questionwas_1:
  /* Original condition value: Cnd_keyexchage_24_questionwas_12 */
  *state = St_keyexchage_42; 
  return 1;
  case Cnd_keyexchage_24_questionwas_2:
  /* Original condition value: Cnd_keyexchage_24_questionwas_16 */
  *state = St_keyexchage_24; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_keyexchage_24'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_keyexchage_42:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_43)
{
  how = the_gi->vtable->C_keyexchage_42_43(the_gi, &got_Cnd_keyexchage_42_43, business_object);
  if (how == 1)
  {
    switch (got_Cnd_keyexchage_42_43) {
case Cnd_keyexchage_42_43_1:
  /* Original condition value: Cnd_keyexchage_42_43_23 */
  *state = St_normal_hm; 
  return 1;
  case Cnd_keyexchage_42_43_2:
  /* Original condition value: Cnd_keyexchage_42_43_9 */
  *state = St_normal_well; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_keyexchage_42'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_normal_hm:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_blablabla)
{
  how = the_gi->vtable->C_normal_hm_blablabla(the_gi, &got_Cnd_normal_hm_blablabla, business_object);
  if (how == 1)
  {
    switch (got_Cnd_normal_hm_blablabla) {
case Cnd_normal_hm_blablabla_1:
  /* Original condition value: Cnd_normal_hm_blablabla_17 */
  *state = St_normal_hm; 
  return 1;
  case Cnd_normal_hm_blablabla_2:
  /* Original condition value: Cnd_normal_hm_blablabla_3 */
  *state = St_normal_well; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

if (self->theMsgFactoryObj->theInMsg.thetype == Msg_or)
{
  how = the_gi->vtable->C_normal_hm_or(the_gi, &got_Cnd_normal_hm_or, business_object);
  if (how == 1)
  {
    switch (got_Cnd_normal_hm_or) {
case Cnd_normal_hm_or_1:
  /* Original condition value: Cnd_normal_hm_or_19 */
  *state = St_awaitconfirm_yes; 
  return 1;
  case Cnd_normal_hm_or_2:
  /* Original condition value: Cnd_normal_hm_or_4 */
  *state = St_awaitconfirm_other; 
  return 1;
  case Cnd_normal_hm_or_3:
  /* Original condition value: Cnd_normal_hm_or_8 */
  *state = St_awaitconfirm_no; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_normal_hm'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_normal_well:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_blablabla)
{
  how = the_gi->vtable->C_normal_well_blablabla(the_gi, &got_Cnd_normal_well_blablabla, business_object);
  if (how == 1)
  {
    switch (got_Cnd_normal_well_blablabla) {
case Cnd_normal_well_blablabla_1:
  /* Original condition value: Cnd_normal_well_blablabla_2 */
  *state = St_normal_hm; 
  return 1;
  case Cnd_normal_well_blablabla_2:
  /* Original condition value: Cnd_normal_well_blablabla_22 */
  *state = St_normal_well; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

if (self->theMsgFactoryObj->theInMsg.thetype == Msg_bye_bob)
{
  how = the_gi->vtable->C_normal_well_bye_bob(the_gi, &got_Cnd_normal_well_bye_bob, business_object);
  if (how == 1)
  {
    switch (got_Cnd_normal_well_bye_bob) {
case Cnd_normal_well_bye_bob_1:
  /* Original condition value: Cnd_normal_well_bye_bob_5 */
  *state = St_final; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_normal_well'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_startup_how_are_you:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_rich)
{
  how = the_gi->vtable->C_startup_how_are_you_rich(the_gi, &got_Cnd_startup_how_are_you_rich, business_object);
  if (how == 1)
  {
    switch (got_Cnd_startup_how_are_you_rich) {
case Cnd_startup_how_are_you_rich_1:
  /* Original condition value: Cnd_startup_how_are_you_rich_10 */
  *state = St_keyexchage_42; 
  return 1;
  case Cnd_startup_how_are_you_rich_2:
  /* Original condition value: Cnd_startup_how_are_you_rich_7 */
  *state = St_keyexchage_24; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

if (self->theMsgFactoryObj->theInMsg.thetype == Msg_tired)
{
  how = the_gi->vtable->C_startup_how_are_you_tired(the_gi, &got_Cnd_startup_how_are_you_tired, business_object);
  if (how == 1)
  {
    switch (got_Cnd_startup_how_are_you_tired) {
case Cnd_startup_how_are_you_tired_1:
  /* Original condition value: Cnd_startup_how_are_you_tired_11 */
  *state = St_final; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This inmessage '%s' is not meaningsful in the state 'St_startup_how_are_you'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

   
// FINAL DEFAULT HERE
default:
snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "C_FSM:: This state '%i' is not defined\n",*state);
  self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
return 0;
}
 
}
