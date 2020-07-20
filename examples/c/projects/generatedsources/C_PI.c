#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "C_PI.h"

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

static int C_prod_24(C_PI * self,
TOMSG_24 given_cond, C_BusinessObject *business_object);

static int C_prod_42(C_PI * self,
TOMSG_42 given_cond, C_BusinessObject *business_object);

static int C_prod_hello_alice(C_PI * self,
TOMSG_hello_alice given_cond, C_BusinessObject *business_object);

static int C_prod_hm(C_PI * self,
TOMSG_hm given_cond, C_BusinessObject *business_object);

static int C_prod_how_are_you(C_PI * self,
TOMSG_how_are_you given_cond, C_BusinessObject *business_object);

static int C_prod_no(C_PI * self,
TOMSG_no given_cond, C_BusinessObject *business_object);

static int C_prod_nooutput(C_PI * self,
TOMSG_nooutput given_cond, C_BusinessObject *business_object);

static int C_prod_other(C_PI * self,
TOMSG_other given_cond, C_BusinessObject *business_object);

static int C_prod_ping(C_PI * self,
TOMSG_ping given_cond, C_BusinessObject *business_object);

static int C_prod_well(C_PI * self,
TOMSG_well given_cond, C_BusinessObject *business_object);

static int C_prod_yes(C_PI * self,
TOMSG_yes given_cond, C_BusinessObject *business_object);
// ALL STATIC
                static C_PI_VTABLE C_prodmsg_vtable ={
&C_prod_24,
	&C_prod_42,
	&C_prod_hello_alice,
	&C_prod_hm,
	&C_prod_how_are_you,
	&C_prod_no,
	&C_prod_nooutput,
	&C_prod_other,
	&C_prod_ping,
	&C_prod_well,
	&C_prod_yes,
};        

/*========== init of guard and prod function vtables ==========*/
C_PI * in_C_PI(C_PI * self)
{
self->mallocated =0;
self->deleteself_C_PI = &deleteself_C_PI;
self->vtable = &C_prodmsg_vtable;
return self;
}
//C_PI * nw_C_PI(C_PI * self)
C_PI * nw_C_PI(void)
{
//  self = malloc(sizeof (C_PI));
  C_PI * self = malloc(sizeof (C_PI));
  self = in_C_PI(self);
  self->mallocated =1; // Is in dynamic memory
  return self;
}
void deleteself_C_PI(C_PI * self)
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

/*--- prod_24 -------------------- */

static int C_prod_24(C_PI * self,
TOMSG_24 given_cond, C_BusinessObject *business_object){

struct Msg_24_real *a_24_out_real;// a_24_out_real  msgout_work_area
a_24_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_24);
struct Msg_questionwas_in_real * a_questionwas_in_real = 0;// avoid unused
struct Msg_rich_in_real * a_rich_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_keyexchage_24_questionwas_2:
/* Original condition value: KeyExchage_24_QuestionWas_16 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_questionwas_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_startup_how_are_you_rich_2:
/* Original condition value: StartUp_How_Are_You_Rich_7 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_rich_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_42 -------------------- */

static int C_prod_42(C_PI * self,
TOMSG_42 given_cond, C_BusinessObject *business_object){

struct Msg_42_real *a_42_out_real;// a_42_out_real  msgout_work_area
a_42_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_42);
struct Msg_questionwas_in_real * a_questionwas_in_real = 0;// avoid unused
struct Msg_rich_in_real * a_rich_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_keyexchage_24_questionwas_1:
/* Original condition value: KeyExchage_24_QuestionWas_12 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_questionwas_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_startup_how_are_you_rich_1:
/* Original condition value: StartUp_How_Are_You_Rich_10 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_rich_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_hello_alice -------------------- */

static int C_prod_hello_alice(C_PI * self,
TOMSG_hello_alice given_cond, C_BusinessObject *business_object){

struct Msg_hello_alice_real *a_hello_alice_out_real;// a_hello_alice_out_real  msgout_work_area
a_hello_alice_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_hello_alice);
struct Msg_callin_in_real * a_callin_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_init_callin_1:
/* Original condition value: INIT_Callin_21 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_callin_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_hm -------------------- */

static int C_prod_hm(C_PI * self,
TOMSG_hm given_cond, C_BusinessObject *business_object){

struct Msg_hm_real *a_hm_out_real;// a_hm_out_real  msgout_work_area
a_hm_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_hm);
struct Msg_43_in_real * a_43_in_real = 0;// avoid unused
struct Msg_blablabla_in_real * a_blablabla_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_awaitconfirm_yes_blablabla_2:
/* Original condition value: AwaitConfirm_Yes_Blablabla_24 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_blablabla_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_keyexchage_42_43_1:
/* Original condition value: KeyExchage_42_43_23 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_43_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_normal_hm_blablabla_1:
/* Original condition value: Normal_Hm_Blablabla_17 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_blablabla_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_normal_well_blablabla_1:
/* Original condition value: Normal_Well_Blablabla_2 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_blablabla_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_how_are_you -------------------- */

static int C_prod_how_are_you(C_PI * self,
TOMSG_how_are_you given_cond, C_BusinessObject *business_object){

struct Msg_how_are_you_real *a_how_are_you_out_real;// a_how_are_you_out_real  msgout_work_area
a_how_are_you_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_how_are_you);
struct Msg_hello_bob_in_real * a_hello_bob_in_real = 0;// avoid unused
struct Msg_pong_in_real * a_pong_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_init_hello_alice_hello_bob_1:
/* Original condition value: INIT_Hello_Alice_Hello_Bob_1 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_hello_bob_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_init_ping_pong_1:
/* Original condition value: INIT_Ping_Pong_15 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_pong_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_no -------------------- */

static int C_prod_no(C_PI * self,
TOMSG_no given_cond, C_BusinessObject *business_object){

struct Msg_no_real *a_no_out_real;// a_no_out_real  msgout_work_area
a_no_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_no);
struct Msg_or_in_real * a_or_in_real = 0;// avoid unused
struct Msg_what_in_real * a_what_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_awaitconfirm_no_what_2:
/* Original condition value: AwaitConfirm_No_What_26 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_what_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_normal_hm_or_3:
/* Original condition value: Normal_Hm_Or_8 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_or_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_nooutput -------------------- */

static int C_prod_nooutput(C_PI * self,
TOMSG_nooutput given_cond, C_BusinessObject *business_object){

struct Msg_nooutput_real *a_nooutput_out_real;// a_nooutput_out_real  msgout_work_area
a_nooutput_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_nooutput);
struct Msg_bye_in_real * a_bye_in_real = 0;// avoid unused
struct Msg_bye_bob_in_real * a_bye_bob_in_real = 0;// avoid unused
struct Msg_dontdisturb_in_real * a_dontdisturb_in_real = 0;// avoid unused
struct Msg_sorry_in_real * a_sorry_in_real = 0;// avoid unused
struct Msg_tired_in_real * a_tired_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_awaitconfirm_other_bye_1:
/* Original condition value: AwaitConfirm_Other_Bye_18 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_bye_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_init_hello_alice_dontdisturb_1:
/* Original condition value: INIT_Hello_Alice_DontDisturb_13 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_dontdisturb_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_init_ping_sorry_1:
/* Original condition value: INIT_Ping_Sorry_14 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_sorry_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_normal_well_bye_bob_1:
/* Original condition value: Normal_Well_Bye_Bob_5 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_bye_bob_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_startup_how_are_you_tired_1:
/* Original condition value: StartUp_How_Are_You_Tired_11 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_tired_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_other -------------------- */

static int C_prod_other(C_PI * self,
TOMSG_other given_cond, C_BusinessObject *business_object){

struct Msg_other_real *a_other_out_real;// a_other_out_real  msgout_work_area
a_other_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_other);
struct Msg_or_in_real * a_or_in_real = 0;// avoid unused
struct Msg_what_in_real * a_what_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_awaitconfirm_no_what_3:
/* Original condition value: AwaitConfirm_No_What_27 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_what_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_normal_hm_or_2:
/* Original condition value: Normal_Hm_Or_4 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_or_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_ping -------------------- */

static int C_prod_ping(C_PI * self,
TOMSG_ping given_cond, C_BusinessObject *business_object){

struct Msg_ping_real *a_ping_out_real;// a_ping_out_real  msgout_work_area
a_ping_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_ping);
struct Msg_callin_in_real * a_callin_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_init_callin_2:
/* Original condition value: INIT_Callin_6 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_callin_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_well -------------------- */

static int C_prod_well(C_PI * self,
TOMSG_well given_cond, C_BusinessObject *business_object){

struct Msg_well_real *a_well_out_real;// a_well_out_real  msgout_work_area
a_well_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_well);
struct Msg_43_in_real * a_43_in_real = 0;// avoid unused
struct Msg_blablabla_in_real * a_blablabla_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_awaitconfirm_yes_blablabla_1:
/* Original condition value: AwaitConfirm_Yes_Blablabla_20 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_blablabla_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_keyexchage_42_43_2:
/* Original condition value: KeyExchage_42_43_9 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_43_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_normal_hm_blablabla_2:
/* Original condition value: Normal_Hm_Blablabla_3 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_blablabla_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_normal_well_blablabla_2:
/* Original condition value: Normal_Well_Blablabla_22 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_blablabla_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method

/*--- prod_yes -------------------- */

static int C_prod_yes(C_PI * self,
TOMSG_yes given_cond, C_BusinessObject *business_object){

struct Msg_yes_real *a_yes_out_real;// a_yes_out_real  msgout_work_area
a_yes_out_real= self->theMsgFactoryObj->
alloc_real_outmsg_MSGFACTORY(
self->theMsgFactoryObj, Msg_yes);
struct Msg_or_in_real * a_or_in_real = 0;// avoid unused
struct Msg_what_in_real * a_what_in_real = 0;// avoid unused
switch (given_cond)
  {
    case FROM_Cnd_awaitconfirm_no_what_1:
/* Original condition value: AwaitConfirm_No_What_25 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_what_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
case FROM_Cnd_normal_hm_or_1:
/* Original condition value: Normal_Hm_Or_19 */
// build out msg
// update BusinessObject
business_object->mythings +=1;
a_or_in_real = self->theMsgFactoryObj->theInMsg.a_real_message;
// and more...
// Commit or back out here
self->theMsgFactoryObj->theOutMessage.msgout.result = OK;
break;
  }// end switch
  /* SET UP RETURN */
  return self->theMsgFactoryObj->theOutMessage.msgout.result == OK;
}// end one method
        