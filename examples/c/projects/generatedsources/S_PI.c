#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "S_PI.h"

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

static int S_prod_43(S_PI * self,
TOMSG_43 given_cond, S_BusinessObject *business_object);

static int S_prod_blablabla(S_PI * self,
TOMSG_blablabla given_cond, S_BusinessObject *business_object);

static int S_prod_bye(S_PI * self,
TOMSG_bye given_cond, S_BusinessObject *business_object);

static int S_prod_bye_bob(S_PI * self,
TOMSG_bye_bob given_cond, S_BusinessObject *business_object);

static int S_prod_dontdisturb(S_PI * self,
TOMSG_dontdisturb given_cond, S_BusinessObject *business_object);

static int S_prod_hello_bob(S_PI * self,
TOMSG_hello_bob given_cond, S_BusinessObject *business_object);

static int S_prod_or(S_PI * self,
TOMSG_or given_cond, S_BusinessObject *business_object);

static int S_prod_pong(S_PI * self,
TOMSG_pong given_cond, S_BusinessObject *business_object);

static int S_prod_questionwas(S_PI * self,
TOMSG_questionwas given_cond, S_BusinessObject *business_object);

static int S_prod_rich(S_PI * self,
TOMSG_rich given_cond, S_BusinessObject *business_object);

static int S_prod_sorry(S_PI * self,
TOMSG_sorry given_cond, S_BusinessObject *business_object);

static int S_prod_tired(S_PI * self,
TOMSG_tired given_cond, S_BusinessObject *business_object);

static int S_prod_what(S_PI * self,
TOMSG_what given_cond, S_BusinessObject *business_object);
// ALL STATIC
                static S_PI_VTABLE S_prodmsg_vtable ={
&S_prod_43,
	&S_prod_blablabla,
	&S_prod_bye,
	&S_prod_bye_bob,
	&S_prod_dontdisturb,
	&S_prod_hello_bob,
	&S_prod_or,
	&S_prod_pong,
	&S_prod_questionwas,
	&S_prod_rich,
	&S_prod_sorry,
	&S_prod_tired,
	&S_prod_what,
};        

/*========== init of guard and prod function vtables ==========*/
S_PI * in_S_PI(S_PI * self)
{
self->mallocated =0;
self->deleteself_S_PI = &deleteself_S_PI;
self->vtable = &S_prodmsg_vtable;
return self;
}
//S_PI * nw_S_PI(S_PI * self)
S_PI * nw_S_PI(void)
{
//  self = malloc(sizeof (S_PI));
  S_PI * self = malloc(sizeof (S_PI));
  self = in_S_PI(self);
  self->mallocated =1; // Is in dynamic memory
  return self;
}
void deleteself_S_PI(S_PI * self)
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

/*- INIT Prod Interface -*/

/*--- prod_43 -------------------- */

static int S_prod_43(S_PI * self,
TOMSG_43 given_cond, S_BusinessObject *business_object){

struct Msg_43_real *a_43_out_real;// a_43_out_real  msgout_work_area
a_43_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_43);
struct Msg_42_in_real * a_42_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_keyexchage_42_1:
/* Original condition value: Goon */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_42_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_blablabla -------------------- */

static int S_prod_blablabla(S_PI * self,
TOMSG_blablabla given_cond, S_BusinessObject *business_object){

struct Msg_blablabla_real *a_blablabla_out_real;// a_blablabla_out_real  msgout_work_area
a_blablabla_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_blablabla);
struct Msg_hm_in_real * a_hm_in_real = 0;// avoid unused
struct Msg_well_in_real * a_well_in_real = 0;// avoid unused
struct Msg_yes_in_real * a_yes_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_awaitconfirm_yes_1:
/* Original condition value: Good */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_yes_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_normal_hm_2:
/* Original condition value: Lot */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_hm_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_normal_well_2:
/* Original condition value: Lot */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_well_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_bye -------------------- */

static int S_prod_bye(S_PI * self,
TOMSG_bye given_cond, S_BusinessObject *business_object){

struct Msg_bye_real *a_bye_out_real;// a_bye_out_real  msgout_work_area
a_bye_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_bye);
struct Msg_other_in_real * a_other_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_awaitconfirm_other_1:
/* Original condition value: Realbad */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_other_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_bye_bob -------------------- */

static int S_prod_bye_bob(S_PI * self,
TOMSG_bye_bob given_cond, S_BusinessObject *business_object){

struct Msg_bye_bob_real *a_bye_bob_out_real;// a_bye_bob_out_real  msgout_work_area
a_bye_bob_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_bye_bob);
struct Msg_well_in_real * a_well_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_normal_well_1:
/* Original condition value: Bad */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_well_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_dontdisturb -------------------- */

static int S_prod_dontdisturb(S_PI * self,
TOMSG_dontdisturb given_cond, S_BusinessObject *business_object){

struct Msg_dontdisturb_real *a_dontdisturb_out_real;// a_dontdisturb_out_real  msgout_work_area
a_dontdisturb_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_dontdisturb);
struct Msg_hello_alice_in_real * a_hello_alice_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_init_hello_alice_2:
/* Original condition value: Tired */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_hello_alice_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_hello_bob -------------------- */

static int S_prod_hello_bob(S_PI * self,
TOMSG_hello_bob given_cond, S_BusinessObject *business_object){

struct Msg_hello_bob_real *a_hello_bob_out_real;// a_hello_bob_out_real  msgout_work_area
a_hello_bob_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_hello_bob);
struct Msg_hello_alice_in_real * a_hello_alice_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_init_hello_alice_1:
/* Original condition value: Alert */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_hello_alice_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_or -------------------- */

static int S_prod_or(S_PI * self,
TOMSG_or given_cond, S_BusinessObject *business_object){

struct Msg_or_real *a_or_out_real;// a_or_out_real  msgout_work_area
a_or_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_or);
struct Msg_hm_in_real * a_hm_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_normal_hm_1:
/* Original condition value: Hardcheck */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_hm_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_pong -------------------- */

static int S_prod_pong(S_PI * self,
TOMSG_pong given_cond, S_BusinessObject *business_object){

struct Msg_pong_real *a_pong_out_real;// a_pong_out_real  msgout_work_area
a_pong_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_pong);
struct Msg_ping_in_real * a_ping_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_init_ping_1:
/* Original condition value: Continue */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_ping_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_questionwas -------------------- */

static int S_prod_questionwas(S_PI * self,
TOMSG_questionwas given_cond, S_BusinessObject *business_object){

struct Msg_questionwas_real *a_questionwas_out_real;// a_questionwas_out_real  msgout_work_area
a_questionwas_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_questionwas);
struct Msg_24_in_real * a_24_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_keyexchage_24_1:
/* Original condition value: Well */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_24_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_rich -------------------- */

static int S_prod_rich(S_PI * self,
TOMSG_rich given_cond, S_BusinessObject *business_object){

struct Msg_rich_real *a_rich_out_real;// a_rich_out_real  msgout_work_area
a_rich_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_rich);
struct Msg_how_are_you_in_real * a_how_are_you_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_startup_how_are_you_1:
/* Original condition value: Good */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_how_are_you_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_sorry -------------------- */

static int S_prod_sorry(S_PI * self,
TOMSG_sorry given_cond, S_BusinessObject *business_object){

struct Msg_sorry_real *a_sorry_out_real;// a_sorry_out_real  msgout_work_area
a_sorry_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_sorry);
struct Msg_ping_in_real * a_ping_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_init_ping_2:
/* Original condition value: Done */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_ping_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_tired -------------------- */

static int S_prod_tired(S_PI * self,
TOMSG_tired given_cond, S_BusinessObject *business_object){

struct Msg_tired_real *a_tired_out_real;// a_tired_out_real  msgout_work_area
a_tired_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_tired);
struct Msg_how_are_you_in_real * a_how_are_you_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_startup_how_are_you_2:
/* Original condition value: Tired */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_how_are_you_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_what -------------------- */

static int S_prod_what(S_PI * self,
TOMSG_what given_cond, S_BusinessObject *business_object){

struct Msg_what_real *a_what_out_real;// a_what_out_real  msgout_work_area
a_what_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_what);
struct Msg_no_in_real * a_no_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_awaitconfirm_no_1:
/* Original condition value: Retry */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_no_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method
        