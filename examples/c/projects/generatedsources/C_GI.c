#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "C_GI.h"

/*- INIT Guard Interface -*/
static int C_awaitconfirm_no_what(C_GI* self, Cnds_awaitconfirm_no_what *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_awaitconfirm_other_bye(C_GI* self, Cnds_awaitconfirm_other_bye *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_awaitconfirm_yes_blablabla(C_GI* self, Cnds_awaitconfirm_yes_blablabla *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_init_callin(C_GI* self, Cnds_init_callin *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_init_hello_alice_dontdisturb(C_GI* self, Cnds_init_hello_alice_dontdisturb *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_init_hello_alice_hello_bob(C_GI* self, Cnds_init_hello_alice_hello_bob *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_init_ping_pong(C_GI* self, Cnds_init_ping_pong *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_init_ping_sorry(C_GI* self, Cnds_init_ping_sorry *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_keyexchage_24_questionwas(C_GI* self, Cnds_keyexchage_24_questionwas *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_keyexchage_42_43(C_GI* self, Cnds_keyexchage_42_43 *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_normal_hm_blablabla(C_GI* self, Cnds_normal_hm_blablabla *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_normal_hm_or(C_GI* self, Cnds_normal_hm_or *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_normal_well_blablabla(C_GI* self, Cnds_normal_well_blablabla *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_normal_well_bye_bob(C_GI* self, Cnds_normal_well_bye_bob *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_startup_how_are_you_rich(C_GI* self, Cnds_startup_how_are_you_rich *ret_choosen_cond, C_BusinessObject *business_object);
    
static int C_startup_how_are_you_tired(C_GI* self, Cnds_startup_how_are_you_tired *ret_choosen_cond, C_BusinessObject *business_object);
    

static C_GI_VTABLE C_vtable ={
	&C_awaitconfirm_no_what,
	&C_awaitconfirm_other_bye,
	&C_awaitconfirm_yes_blablabla,
	&C_init_callin,
	&C_init_hello_alice_dontdisturb,
	&C_init_hello_alice_hello_bob,
	&C_init_ping_pong,
	&C_init_ping_sorry,
	&C_keyexchage_24_questionwas,
	&C_keyexchage_42_43,
	&C_normal_hm_blablabla,
	&C_normal_hm_or,
	&C_normal_well_blablabla,
	&C_normal_well_bye_bob,
	&C_startup_how_are_you_rich,
	&C_startup_how_are_you_tired
};

/*========== init of guard and prod function vtables ==========*/
C_GI * in_C_GI(C_GI * self)
{
  self->mallocated =0;
  self->deleteself_C_GI = &deleteself_C_GI;
  self->vtable = &C_vtable;
  return self;
}
C_GI * nw_C_GI(C_GI * self)
{
  self = malloc(sizeof (C_GI));
  self = in_C_GI(self);
  self->mallocated =1; // Is in dynamic memory
  return self;
}
void deleteself_C_GI(C_GI * self)
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

/*======== guard ***() Implementations ========*/
/* NOTE
Peek into the inmessage and your own business_object
in order to assign a choice to 'the_in_cond_choice'
if (self->inmessage ...)
if (business_object-> ...)

Here (in test/verify) we do a random choice

*/
/*=============================================*/
/*--- AwaitConfirm_No_What -------------------- */
static int C_awaitconfirm_no_what(C_GI* self, Cnds_awaitconfirm_no_what *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_awaitconfirm_no_what the_in_cond_choice = 0;// avoid warnings
TOMSG_no to_no;
TOMSG_other to_other;
TOMSG_yes to_yes;
struct Msg_what_real * a_what_real = 0;// the inmessage. (avoid unused warning)
a_what_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 3;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: AwaitConfirm_No_What_25 */
  the_in_cond_choice = Cnd_awaitconfirm_no_what_1;
  break;
case 1:
  /* Original condition value: AwaitConfirm_No_What_26 */
  the_in_cond_choice = Cnd_awaitconfirm_no_what_2;
  break;
case 2:
  /* Original condition value: AwaitConfirm_No_What_27 */
  the_in_cond_choice = Cnd_awaitconfirm_no_what_3;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_awaitconfirm_no_what out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_awaitconfirm_no_what_1:
/* Original condition value: AwaitConfirm_No_What_25 */
to_yes = FROM_Cnd_awaitconfirm_no_what_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_yes(self->theProdObj, to_yes, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_yes 
}
break;
case Cnd_awaitconfirm_no_what_2:
/* Original condition value: AwaitConfirm_No_What_26 */
to_no = FROM_Cnd_awaitconfirm_no_what_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_no(self->theProdObj, to_no, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_no 
}
break;
case Cnd_awaitconfirm_no_what_3:
/* Original condition value: AwaitConfirm_No_What_27 */
to_other = FROM_Cnd_awaitconfirm_no_what_3;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_other(self->theProdObj, to_other, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_other 
}
break;
}
return how;
}
                
    
/*--- AwaitConfirm_Other_Bye -------------------- */
static int C_awaitconfirm_other_bye(C_GI* self, Cnds_awaitconfirm_other_bye *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_awaitconfirm_other_bye the_in_cond_choice = 0;// avoid warnings
TOMSG_nooutput to_nooutput;
struct Msg_bye_real * a_bye_real = 0;// the inmessage. (avoid unused warning)
a_bye_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: AwaitConfirm_Other_Bye_18 */
  the_in_cond_choice = Cnd_awaitconfirm_other_bye_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_awaitconfirm_other_bye out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_awaitconfirm_other_bye_1:
/* Original condition value: AwaitConfirm_Other_Bye_18 */
to_nooutput = FROM_Cnd_awaitconfirm_other_bye_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_nooutput(self->theProdObj, to_nooutput, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_nooutput 
}
break;
}
return how;
}
                
    
/*--- AwaitConfirm_Yes_Blablabla -------------------- */
static int C_awaitconfirm_yes_blablabla(C_GI* self, Cnds_awaitconfirm_yes_blablabla *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_awaitconfirm_yes_blablabla the_in_cond_choice = 0;// avoid warnings
TOMSG_hm to_hm;
TOMSG_well to_well;
struct Msg_blablabla_real * a_blablabla_real = 0;// the inmessage. (avoid unused warning)
a_blablabla_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: AwaitConfirm_Yes_Blablabla_20 */
  the_in_cond_choice = Cnd_awaitconfirm_yes_blablabla_1;
  break;
case 1:
  /* Original condition value: AwaitConfirm_Yes_Blablabla_24 */
  the_in_cond_choice = Cnd_awaitconfirm_yes_blablabla_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_awaitconfirm_yes_blablabla out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_awaitconfirm_yes_blablabla_1:
/* Original condition value: AwaitConfirm_Yes_Blablabla_20 */
to_well = FROM_Cnd_awaitconfirm_yes_blablabla_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_well(self->theProdObj, to_well, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_well 
}
break;
case Cnd_awaitconfirm_yes_blablabla_2:
/* Original condition value: AwaitConfirm_Yes_Blablabla_24 */
to_hm = FROM_Cnd_awaitconfirm_yes_blablabla_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_hm(self->theProdObj, to_hm, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_hm 
}
break;
}
return how;
}
                
    
/*--- INIT_Callin -------------------- */
static int C_init_callin(C_GI* self, Cnds_init_callin *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_init_callin the_in_cond_choice = 0;// avoid warnings
TOMSG_hello_alice to_hello_alice;
TOMSG_ping to_ping;
struct Msg_callin_real * a_callin_real = 0;// the inmessage. (avoid unused warning)
a_callin_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: INIT_Callin_21 */
  the_in_cond_choice = Cnd_init_callin_1;
  break;
case 1:
  /* Original condition value: INIT_Callin_6 */
  the_in_cond_choice = Cnd_init_callin_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_init_callin out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_init_callin_1:
/* Original condition value: INIT_Callin_21 */
to_hello_alice = FROM_Cnd_init_callin_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_hello_alice(self->theProdObj, to_hello_alice, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_hello_alice 
}
break;
case Cnd_init_callin_2:
/* Original condition value: INIT_Callin_6 */
to_ping = FROM_Cnd_init_callin_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_ping(self->theProdObj, to_ping, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_ping 
}
break;
}
return how;
}
                
    
/*--- INIT_Hello_Alice_DontDisturb -------------------- */
static int C_init_hello_alice_dontdisturb(C_GI* self, Cnds_init_hello_alice_dontdisturb *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_init_hello_alice_dontdisturb the_in_cond_choice = 0;// avoid warnings
TOMSG_nooutput to_nooutput;
struct Msg_dontdisturb_real * a_dontdisturb_real = 0;// the inmessage. (avoid unused warning)
a_dontdisturb_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: INIT_Hello_Alice_DontDisturb_13 */
  the_in_cond_choice = Cnd_init_hello_alice_dontdisturb_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_init_hello_alice_dontdisturb out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_init_hello_alice_dontdisturb_1:
/* Original condition value: INIT_Hello_Alice_DontDisturb_13 */
to_nooutput = FROM_Cnd_init_hello_alice_dontdisturb_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_nooutput(self->theProdObj, to_nooutput, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_nooutput 
}
break;
}
return how;
}
                
    
/*--- INIT_Hello_Alice_Hello_Bob -------------------- */
static int C_init_hello_alice_hello_bob(C_GI* self, Cnds_init_hello_alice_hello_bob *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_init_hello_alice_hello_bob the_in_cond_choice = 0;// avoid warnings
TOMSG_how_are_you to_how_are_you;
struct Msg_hello_bob_real * a_hello_bob_real = 0;// the inmessage. (avoid unused warning)
a_hello_bob_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: INIT_Hello_Alice_Hello_Bob_1 */
  the_in_cond_choice = Cnd_init_hello_alice_hello_bob_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_init_hello_alice_hello_bob out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_init_hello_alice_hello_bob_1:
/* Original condition value: INIT_Hello_Alice_Hello_Bob_1 */
to_how_are_you = FROM_Cnd_init_hello_alice_hello_bob_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_how_are_you(self->theProdObj, to_how_are_you, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_how_are_you 
}
break;
}
return how;
}
                
    
/*--- INIT_Ping_Pong -------------------- */
static int C_init_ping_pong(C_GI* self, Cnds_init_ping_pong *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_init_ping_pong the_in_cond_choice = 0;// avoid warnings
TOMSG_how_are_you to_how_are_you;
struct Msg_pong_real * a_pong_real = 0;// the inmessage. (avoid unused warning)
a_pong_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: INIT_Ping_Pong_15 */
  the_in_cond_choice = Cnd_init_ping_pong_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_init_ping_pong out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_init_ping_pong_1:
/* Original condition value: INIT_Ping_Pong_15 */
to_how_are_you = FROM_Cnd_init_ping_pong_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_how_are_you(self->theProdObj, to_how_are_you, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_how_are_you 
}
break;
}
return how;
}
                
    
/*--- INIT_Ping_Sorry -------------------- */
static int C_init_ping_sorry(C_GI* self, Cnds_init_ping_sorry *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_init_ping_sorry the_in_cond_choice = 0;// avoid warnings
TOMSG_nooutput to_nooutput;
struct Msg_sorry_real * a_sorry_real = 0;// the inmessage. (avoid unused warning)
a_sorry_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: INIT_Ping_Sorry_14 */
  the_in_cond_choice = Cnd_init_ping_sorry_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_init_ping_sorry out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_init_ping_sorry_1:
/* Original condition value: INIT_Ping_Sorry_14 */
to_nooutput = FROM_Cnd_init_ping_sorry_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_nooutput(self->theProdObj, to_nooutput, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_nooutput 
}
break;
}
return how;
}
                
    
/*--- KeyExchage_24_QuestionWas -------------------- */
static int C_keyexchage_24_questionwas(C_GI* self, Cnds_keyexchage_24_questionwas *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_keyexchage_24_questionwas the_in_cond_choice = 0;// avoid warnings
TOMSG_24 to_24;
TOMSG_42 to_42;
struct Msg_questionwas_real * a_questionwas_real = 0;// the inmessage. (avoid unused warning)
a_questionwas_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: KeyExchage_24_QuestionWas_12 */
  the_in_cond_choice = Cnd_keyexchage_24_questionwas_1;
  break;
case 1:
  /* Original condition value: KeyExchage_24_QuestionWas_16 */
  the_in_cond_choice = Cnd_keyexchage_24_questionwas_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_keyexchage_24_questionwas out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_keyexchage_24_questionwas_1:
/* Original condition value: KeyExchage_24_QuestionWas_12 */
to_42 = FROM_Cnd_keyexchage_24_questionwas_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_42(self->theProdObj, to_42, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_42 
}
break;
case Cnd_keyexchage_24_questionwas_2:
/* Original condition value: KeyExchage_24_QuestionWas_16 */
to_24 = FROM_Cnd_keyexchage_24_questionwas_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_24(self->theProdObj, to_24, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_24 
}
break;
}
return how;
}
                
    
/*--- KeyExchage_42_43 -------------------- */
static int C_keyexchage_42_43(C_GI* self, Cnds_keyexchage_42_43 *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_keyexchage_42_43 the_in_cond_choice = 0;// avoid warnings
TOMSG_hm to_hm;
TOMSG_well to_well;
struct Msg_43_real * a_43_real = 0;// the inmessage. (avoid unused warning)
a_43_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: KeyExchage_42_43_23 */
  the_in_cond_choice = Cnd_keyexchage_42_43_1;
  break;
case 1:
  /* Original condition value: KeyExchage_42_43_9 */
  the_in_cond_choice = Cnd_keyexchage_42_43_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_keyexchage_42_43 out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_keyexchage_42_43_1:
/* Original condition value: KeyExchage_42_43_23 */
to_hm = FROM_Cnd_keyexchage_42_43_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_hm(self->theProdObj, to_hm, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_hm 
}
break;
case Cnd_keyexchage_42_43_2:
/* Original condition value: KeyExchage_42_43_9 */
to_well = FROM_Cnd_keyexchage_42_43_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_well(self->theProdObj, to_well, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_well 
}
break;
}
return how;
}
                
    
/*--- Normal_Hm_Blablabla -------------------- */
static int C_normal_hm_blablabla(C_GI* self, Cnds_normal_hm_blablabla *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_normal_hm_blablabla the_in_cond_choice = 0;// avoid warnings
TOMSG_hm to_hm;
TOMSG_well to_well;
struct Msg_blablabla_real * a_blablabla_real = 0;// the inmessage. (avoid unused warning)
a_blablabla_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Normal_Hm_Blablabla_17 */
  the_in_cond_choice = Cnd_normal_hm_blablabla_1;
  break;
case 1:
  /* Original condition value: Normal_Hm_Blablabla_3 */
  the_in_cond_choice = Cnd_normal_hm_blablabla_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_normal_hm_blablabla out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_normal_hm_blablabla_1:
/* Original condition value: Normal_Hm_Blablabla_17 */
to_hm = FROM_Cnd_normal_hm_blablabla_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_hm(self->theProdObj, to_hm, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_hm 
}
break;
case Cnd_normal_hm_blablabla_2:
/* Original condition value: Normal_Hm_Blablabla_3 */
to_well = FROM_Cnd_normal_hm_blablabla_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_well(self->theProdObj, to_well, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_well 
}
break;
}
return how;
}
                
    
/*--- Normal_Hm_Or -------------------- */
static int C_normal_hm_or(C_GI* self, Cnds_normal_hm_or *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_normal_hm_or the_in_cond_choice = 0;// avoid warnings
TOMSG_no to_no;
TOMSG_other to_other;
TOMSG_yes to_yes;
struct Msg_or_real * a_or_real = 0;// the inmessage. (avoid unused warning)
a_or_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 3;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Normal_Hm_Or_19 */
  the_in_cond_choice = Cnd_normal_hm_or_1;
  break;
case 1:
  /* Original condition value: Normal_Hm_Or_4 */
  the_in_cond_choice = Cnd_normal_hm_or_2;
  break;
case 2:
  /* Original condition value: Normal_Hm_Or_8 */
  the_in_cond_choice = Cnd_normal_hm_or_3;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_normal_hm_or out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_normal_hm_or_1:
/* Original condition value: Normal_Hm_Or_19 */
to_yes = FROM_Cnd_normal_hm_or_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_yes(self->theProdObj, to_yes, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_yes 
}
break;
case Cnd_normal_hm_or_2:
/* Original condition value: Normal_Hm_Or_4 */
to_other = FROM_Cnd_normal_hm_or_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_other(self->theProdObj, to_other, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_other 
}
break;
case Cnd_normal_hm_or_3:
/* Original condition value: Normal_Hm_Or_8 */
to_no = FROM_Cnd_normal_hm_or_3;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_no(self->theProdObj, to_no, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_no 
}
break;
}
return how;
}
                
    
/*--- Normal_Well_Blablabla -------------------- */
static int C_normal_well_blablabla(C_GI* self, Cnds_normal_well_blablabla *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_normal_well_blablabla the_in_cond_choice = 0;// avoid warnings
TOMSG_hm to_hm;
TOMSG_well to_well;
struct Msg_blablabla_real * a_blablabla_real = 0;// the inmessage. (avoid unused warning)
a_blablabla_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Normal_Well_Blablabla_2 */
  the_in_cond_choice = Cnd_normal_well_blablabla_1;
  break;
case 1:
  /* Original condition value: Normal_Well_Blablabla_22 */
  the_in_cond_choice = Cnd_normal_well_blablabla_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_normal_well_blablabla out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_normal_well_blablabla_1:
/* Original condition value: Normal_Well_Blablabla_2 */
to_hm = FROM_Cnd_normal_well_blablabla_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_hm(self->theProdObj, to_hm, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_hm 
}
break;
case Cnd_normal_well_blablabla_2:
/* Original condition value: Normal_Well_Blablabla_22 */
to_well = FROM_Cnd_normal_well_blablabla_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_well(self->theProdObj, to_well, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_well 
}
break;
}
return how;
}
                
    
/*--- Normal_Well_Bye_Bob -------------------- */
static int C_normal_well_bye_bob(C_GI* self, Cnds_normal_well_bye_bob *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_normal_well_bye_bob the_in_cond_choice = 0;// avoid warnings
TOMSG_nooutput to_nooutput;
struct Msg_bye_bob_real * a_bye_bob_real = 0;// the inmessage. (avoid unused warning)
a_bye_bob_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Normal_Well_Bye_Bob_5 */
  the_in_cond_choice = Cnd_normal_well_bye_bob_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_normal_well_bye_bob out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_normal_well_bye_bob_1:
/* Original condition value: Normal_Well_Bye_Bob_5 */
to_nooutput = FROM_Cnd_normal_well_bye_bob_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_nooutput(self->theProdObj, to_nooutput, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_nooutput 
}
break;
}
return how;
}
                
    
/*--- StartUp_How_Are_You_Rich -------------------- */
static int C_startup_how_are_you_rich(C_GI* self, Cnds_startup_how_are_you_rich *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_startup_how_are_you_rich the_in_cond_choice = 0;// avoid warnings
TOMSG_24 to_24;
TOMSG_42 to_42;
struct Msg_rich_real * a_rich_real = 0;// the inmessage. (avoid unused warning)
a_rich_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: StartUp_How_Are_You_Rich_10 */
  the_in_cond_choice = Cnd_startup_how_are_you_rich_1;
  break;
case 1:
  /* Original condition value: StartUp_How_Are_You_Rich_7 */
  the_in_cond_choice = Cnd_startup_how_are_you_rich_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_startup_how_are_you_rich out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_startup_how_are_you_rich_1:
/* Original condition value: StartUp_How_Are_You_Rich_10 */
to_42 = FROM_Cnd_startup_how_are_you_rich_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_42(self->theProdObj, to_42, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_42 
}
break;
case Cnd_startup_how_are_you_rich_2:
/* Original condition value: StartUp_How_Are_You_Rich_7 */
to_24 = FROM_Cnd_startup_how_are_you_rich_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_24(self->theProdObj, to_24, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_24 
}
break;
}
return how;
}
                
    
/*--- StartUp_How_Are_You_Tired -------------------- */
static int C_startup_how_are_you_tired(C_GI* self, Cnds_startup_how_are_you_tired *ret_choosen_cond, C_BusinessObject *business_object)
{
Cnds_startup_how_are_you_tired the_in_cond_choice = 0;// avoid warnings
TOMSG_nooutput to_nooutput;
struct Msg_tired_real * a_tired_real = 0;// the inmessage. (avoid unused warning)
a_tired_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: StartUp_How_Are_You_Tired_11 */
  the_in_cond_choice = Cnd_startup_how_are_you_tired_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "C_::Cnds_startup_how_are_you_tired out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_startup_how_are_you_tired_1:
/* Original condition value: StartUp_How_Are_You_Tired_11 */
to_nooutput = FROM_Cnd_startup_how_are_you_tired_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->C_prod_nooutput(self->theProdObj, to_nooutput, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_nooutput 
}
break;
}
return how;
}
                
    
