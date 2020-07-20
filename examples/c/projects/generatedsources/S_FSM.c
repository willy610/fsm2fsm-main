#include "S_FSM.h"

enum states { St_keyexchage,
	St_final,
	St_awaitconfirm,
	St_normal,
	St_init,
	St_startup };


S_FSM * in_S_FSM(S_FSM * self)
{
  self->mallocated =0;
  self->the_gi = 0;
  self->deleteself_S_FSM = &deleteself_S_FSM;
  self->getinitialstate_S_FSM = &getinitialstate_S_FSM;
  self->getfinalstate_S_FSM = &getfinalstate_S_FSM;
  self->isinfinalstate_S_FSM = &isinfinalstate_S_FSM;
  self->get_possible_init_messages_S_FSM= &get_possible_init_messages_S_FSM;
  self->take_event_S_FSM = &take_event_S_FSM;
  return self;
}

S_FSM * nw_S_FSM(void)
{
  S_FSM * self = malloc(sizeof (S_FSM));
  self = in_S_FSM(self);
  self->mallocated =1; // Is in dynamic memory
  return self;
}

void deleteself_S_FSM(S_FSM * self)
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

int getinitialstate_S_FSM(S_FSM * self)
{
  (void)(self);
  return St_init;
}
int getfinalstate_S_FSM(S_FSM * self)
{
  (void)(self);
  return St_final;
}
int isinfinalstate_S_FSM(S_FSM * self, int state)
{
  (void)(self);
  return state == St_final ;
}

INIT_MESSAGES * get_possible_init_messages_S_FSM(S_FSM * self)
{
  static INIT_MESSAGES the_init_array;
  (void)(self);
  the_init_array.inits[0] = Msg_hello_alice;
the_init_array.inits[1] = Msg_ping;
  the_init_array.the_size_of_inits = 2;
  return &the_init_array;
}


int take_event_S_FSM(S_FSM * self, int *state, void *business_object)
{
S_GI *the_gi;
the_gi = self->the_gi;
int how =0;
Cnds_awaitconfirm_no got_Cnd_awaitconfirm_no;
Cnds_awaitconfirm_other got_Cnd_awaitconfirm_other;
Cnds_awaitconfirm_yes got_Cnd_awaitconfirm_yes;
Cnds_init_hello_alice got_Cnd_init_hello_alice;
Cnds_init_ping got_Cnd_init_ping;
Cnds_keyexchage_24 got_Cnd_keyexchage_24;
Cnds_keyexchage_42 got_Cnd_keyexchage_42;
Cnds_normal_hm got_Cnd_normal_hm;
Cnds_normal_well got_Cnd_normal_well;
Cnds_startup_how_are_you got_Cnd_startup_how_are_you;
          
// *state = 123; // for verify error managment
switch (*state)
{ case St_awaitconfirm:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_no)
{
  how = the_gi->vtable->S_awaitconfirm_no(the_gi, &got_Cnd_awaitconfirm_no, business_object);
  if (how == 1)
  {
    switch (got_Cnd_awaitconfirm_no) {
case Cnd_awaitconfirm_no_1:
  /* Original condition value: Cnd_retry */
  *state = St_awaitconfirm; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

if (self->theMsgFactoryObj->theInMsg.thetype == Msg_other)
{
  how = the_gi->vtable->S_awaitconfirm_other(the_gi, &got_Cnd_awaitconfirm_other, business_object);
  if (how == 1)
  {
    switch (got_Cnd_awaitconfirm_other) {
case Cnd_awaitconfirm_other_1:
  /* Original condition value: Cnd_realbad */
  *state = St_final; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

if (self->theMsgFactoryObj->theInMsg.thetype == Msg_yes)
{
  how = the_gi->vtable->S_awaitconfirm_yes(the_gi, &got_Cnd_awaitconfirm_yes, business_object);
  if (how == 1)
  {
    switch (got_Cnd_awaitconfirm_yes) {
case Cnd_awaitconfirm_yes_1:
  /* Original condition value: Cnd_good */
  *state = St_normal; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "S_FSM:: This inmessage '%s' is not meaningsful in the state 'St_awaitconfirm'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_init:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_hello_alice)
{
  how = the_gi->vtable->S_init_hello_alice(the_gi, &got_Cnd_init_hello_alice, business_object);
  if (how == 1)
  {
    switch (got_Cnd_init_hello_alice) {
case Cnd_init_hello_alice_1:
  /* Original condition value: Cnd_alert */
  *state = St_startup; 
  return 1;
  case Cnd_init_hello_alice_2:
  /* Original condition value: Cnd_tired */
  *state = St_final; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

if (self->theMsgFactoryObj->theInMsg.thetype == Msg_ping)
{
  how = the_gi->vtable->S_init_ping(the_gi, &got_Cnd_init_ping, business_object);
  if (how == 1)
  {
    switch (got_Cnd_init_ping) {
case Cnd_init_ping_1:
  /* Original condition value: Cnd_continue */
  *state = St_startup; 
  return 1;
  case Cnd_init_ping_2:
  /* Original condition value: Cnd_done */
  *state = St_final; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "S_FSM:: This inmessage '%s' is not meaningsful in the state 'St_init'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_keyexchage:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_24)
{
  how = the_gi->vtable->S_keyexchage_24(the_gi, &got_Cnd_keyexchage_24, business_object);
  if (how == 1)
  {
    switch (got_Cnd_keyexchage_24) {
case Cnd_keyexchage_24_1:
  /* Original condition value: Cnd_well */
  *state = St_keyexchage; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

if (self->theMsgFactoryObj->theInMsg.thetype == Msg_42)
{
  how = the_gi->vtable->S_keyexchage_42(the_gi, &got_Cnd_keyexchage_42, business_object);
  if (how == 1)
  {
    switch (got_Cnd_keyexchage_42) {
case Cnd_keyexchage_42_1:
  /* Original condition value: Cnd_goon */
  *state = St_normal; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "S_FSM:: This inmessage '%s' is not meaningsful in the state 'St_keyexchage'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_normal:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_hm)
{
  how = the_gi->vtable->S_normal_hm(the_gi, &got_Cnd_normal_hm, business_object);
  if (how == 1)
  {
    switch (got_Cnd_normal_hm) {
case Cnd_normal_hm_1:
  /* Original condition value: Cnd_hardcheck */
  *state = St_awaitconfirm; 
  return 1;
  case Cnd_normal_hm_2:
  /* Original condition value: Cnd_lot */
  *state = St_normal; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

if (self->theMsgFactoryObj->theInMsg.thetype == Msg_well)
{
  how = the_gi->vtable->S_normal_well(the_gi, &got_Cnd_normal_well, business_object);
  if (how == 1)
  {
    switch (got_Cnd_normal_well) {
case Cnd_normal_well_1:
  /* Original condition value: Cnd_bad */
  *state = St_final; 
  return 1;
  case Cnd_normal_well_2:
  /* Original condition value: Cnd_lot */
  *state = St_normal; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "S_FSM:: This inmessage '%s' is not meaningsful in the state 'St_normal'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

  
case St_startup:
     
if (self->theMsgFactoryObj->theInMsg.thetype == Msg_how_are_you)
{
  how = the_gi->vtable->S_startup_how_are_you(the_gi, &got_Cnd_startup_how_are_you, business_object);
  if (how == 1)
  {
    switch (got_Cnd_startup_how_are_you) {
case Cnd_startup_how_are_you_1:
  /* Original condition value: Cnd_good */
  *state = St_keyexchage; 
  return 1;
  case Cnd_startup_how_are_you_2:
  /* Original condition value: Cnd_tired */
  *state = St_final; 
  return 1;
  }
  } else // how != 1
  {
    return 0;
  }
} 
  

snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "S_FSM:: This inmessage '%s' is not meaningsful in the state 'St_startup'\n",
  MESSAGES_LOOKUP[self->theMsgFactoryObj->theInMsg.thetype].msg_name
);
self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,self->theMsgFactoryObj->sprint_dest_buff);
return 0;

   
// FINAL DEFAULT HERE
default:
snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
  "S_FSM:: This state '%i' is not defined\n",*state);
  self->theMsgFactoryObj->
  setup_return_error_MSGFACTORY(self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
return 0;
}
 
}
