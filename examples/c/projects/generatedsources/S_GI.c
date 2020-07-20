#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "S_GI.h"

/*- INIT Guard Interface -*/
static int S_awaitconfirm_no(S_GI* self, Cnds_awaitconfirm_no *ret_choosen_cond, S_BusinessObject *business_object);
    
static int S_awaitconfirm_other(S_GI* self, Cnds_awaitconfirm_other *ret_choosen_cond, S_BusinessObject *business_object);
    
static int S_awaitconfirm_yes(S_GI* self, Cnds_awaitconfirm_yes *ret_choosen_cond, S_BusinessObject *business_object);
    
static int S_init_hello_alice(S_GI* self, Cnds_init_hello_alice *ret_choosen_cond, S_BusinessObject *business_object);
    
static int S_init_ping(S_GI* self, Cnds_init_ping *ret_choosen_cond, S_BusinessObject *business_object);
    
static int S_keyexchage_24(S_GI* self, Cnds_keyexchage_24 *ret_choosen_cond, S_BusinessObject *business_object);
    
static int S_keyexchage_42(S_GI* self, Cnds_keyexchage_42 *ret_choosen_cond, S_BusinessObject *business_object);
    
static int S_normal_hm(S_GI* self, Cnds_normal_hm *ret_choosen_cond, S_BusinessObject *business_object);
    
static int S_normal_well(S_GI* self, Cnds_normal_well *ret_choosen_cond, S_BusinessObject *business_object);
    
static int S_startup_how_are_you(S_GI* self, Cnds_startup_how_are_you *ret_choosen_cond, S_BusinessObject *business_object);
    

static S_GI_VTABLE S_vtable ={
	&S_awaitconfirm_no,
	&S_awaitconfirm_other,
	&S_awaitconfirm_yes,
	&S_init_hello_alice,
	&S_init_ping,
	&S_keyexchage_24,
	&S_keyexchage_42,
	&S_normal_hm,
	&S_normal_well,
	&S_startup_how_are_you
};

/*========== init of guard and prod function vtables ==========*/
S_GI * in_S_GI(S_GI * self)
{
  self->mallocated =0;
  self->deleteself_S_GI = &deleteself_S_GI;
  self->vtable = &S_vtable;
  return self;
}
S_GI * nw_S_GI(S_GI * self)
{
  self = malloc(sizeof (S_GI));
  self = in_S_GI(self);
  self->mallocated =1; // Is in dynamic memory
  return self;
}
void deleteself_S_GI(S_GI * self)
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
/*--- AwaitConfirm_No -------------------- */
static int S_awaitconfirm_no(S_GI* self, Cnds_awaitconfirm_no *ret_choosen_cond, S_BusinessObject *business_object)
{
Cnds_awaitconfirm_no the_in_cond_choice = 0;// avoid warnings
TOMSG_what to_what;
struct Msg_no_real * a_no_real = 0;// the inmessage. (avoid unused warning)
a_no_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Retry */
  the_in_cond_choice = Cnd_awaitconfirm_no_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "S_::Cnds_awaitconfirm_no out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_awaitconfirm_no_1:
/* Original condition value: Retry */
to_what = FROM_Cnd_awaitconfirm_no_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_what(self->theProdObj, to_what, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_what 
}
break;
}
return how;
}
                
    
/*--- AwaitConfirm_Other -------------------- */
static int S_awaitconfirm_other(S_GI* self, Cnds_awaitconfirm_other *ret_choosen_cond, S_BusinessObject *business_object)
{
Cnds_awaitconfirm_other the_in_cond_choice = 0;// avoid warnings
TOMSG_bye to_bye;
struct Msg_other_real * a_other_real = 0;// the inmessage. (avoid unused warning)
a_other_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Realbad */
  the_in_cond_choice = Cnd_awaitconfirm_other_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "S_::Cnds_awaitconfirm_other out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_awaitconfirm_other_1:
/* Original condition value: Realbad */
to_bye = FROM_Cnd_awaitconfirm_other_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_bye(self->theProdObj, to_bye, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_bye 
}
break;
}
return how;
}
                
    
/*--- AwaitConfirm_Yes -------------------- */
static int S_awaitconfirm_yes(S_GI* self, Cnds_awaitconfirm_yes *ret_choosen_cond, S_BusinessObject *business_object)
{
Cnds_awaitconfirm_yes the_in_cond_choice = 0;// avoid warnings
TOMSG_blablabla to_blablabla;
struct Msg_yes_real * a_yes_real = 0;// the inmessage. (avoid unused warning)
a_yes_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Good */
  the_in_cond_choice = Cnd_awaitconfirm_yes_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "S_::Cnds_awaitconfirm_yes out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_awaitconfirm_yes_1:
/* Original condition value: Good */
to_blablabla = FROM_Cnd_awaitconfirm_yes_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_blablabla(self->theProdObj, to_blablabla, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_blablabla 
}
break;
}
return how;
}
                
    
/*--- INIT_Hello_Alice -------------------- */
static int S_init_hello_alice(S_GI* self, Cnds_init_hello_alice *ret_choosen_cond, S_BusinessObject *business_object)
{
Cnds_init_hello_alice the_in_cond_choice = 0;// avoid warnings
TOMSG_dontdisturb to_dontdisturb;
TOMSG_hello_bob to_hello_bob;
struct Msg_hello_alice_real * a_hello_alice_real = 0;// the inmessage. (avoid unused warning)
a_hello_alice_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Alert */
  the_in_cond_choice = Cnd_init_hello_alice_1;
  break;
case 1:
  /* Original condition value: Tired */
  the_in_cond_choice = Cnd_init_hello_alice_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "S_::Cnds_init_hello_alice out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_init_hello_alice_1:
/* Original condition value: Alert */
to_hello_bob = FROM_Cnd_init_hello_alice_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_hello_bob(self->theProdObj, to_hello_bob, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_hello_bob 
}
break;
case Cnd_init_hello_alice_2:
/* Original condition value: Tired */
to_dontdisturb = FROM_Cnd_init_hello_alice_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_dontdisturb(self->theProdObj, to_dontdisturb, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_dontdisturb 
}
break;
}
return how;
}
                
    
/*--- INIT_Ping -------------------- */
static int S_init_ping(S_GI* self, Cnds_init_ping *ret_choosen_cond, S_BusinessObject *business_object)
{
Cnds_init_ping the_in_cond_choice = 0;// avoid warnings
TOMSG_pong to_pong;
TOMSG_sorry to_sorry;
struct Msg_ping_real * a_ping_real = 0;// the inmessage. (avoid unused warning)
a_ping_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Continue */
  the_in_cond_choice = Cnd_init_ping_1;
  break;
case 1:
  /* Original condition value: Done */
  the_in_cond_choice = Cnd_init_ping_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "S_::Cnds_init_ping out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_init_ping_1:
/* Original condition value: Continue */
to_pong = FROM_Cnd_init_ping_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_pong(self->theProdObj, to_pong, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_pong 
}
break;
case Cnd_init_ping_2:
/* Original condition value: Done */
to_sorry = FROM_Cnd_init_ping_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_sorry(self->theProdObj, to_sorry, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_sorry 
}
break;
}
return how;
}
                
    
/*--- KeyExchage_24 -------------------- */
static int S_keyexchage_24(S_GI* self, Cnds_keyexchage_24 *ret_choosen_cond, S_BusinessObject *business_object)
{
Cnds_keyexchage_24 the_in_cond_choice = 0;// avoid warnings
TOMSG_questionwas to_questionwas;
struct Msg_24_real * a_24_real = 0;// the inmessage. (avoid unused warning)
a_24_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Well */
  the_in_cond_choice = Cnd_keyexchage_24_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "S_::Cnds_keyexchage_24 out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_keyexchage_24_1:
/* Original condition value: Well */
to_questionwas = FROM_Cnd_keyexchage_24_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_questionwas(self->theProdObj, to_questionwas, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_questionwas 
}
break;
}
return how;
}
                
    
/*--- KeyExchage_42 -------------------- */
static int S_keyexchage_42(S_GI* self, Cnds_keyexchage_42 *ret_choosen_cond, S_BusinessObject *business_object)
{
Cnds_keyexchage_42 the_in_cond_choice = 0;// avoid warnings
TOMSG_43 to_43;
struct Msg_42_real * a_42_real = 0;// the inmessage. (avoid unused warning)
a_42_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 1;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Goon */
  the_in_cond_choice = Cnd_keyexchage_42_1;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "S_::Cnds_keyexchage_42 out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_keyexchage_42_1:
/* Original condition value: Goon */
to_43 = FROM_Cnd_keyexchage_42_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_43(self->theProdObj, to_43, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_43 
}
break;
}
return how;
}
                
    
/*--- Normal_Hm -------------------- */
static int S_normal_hm(S_GI* self, Cnds_normal_hm *ret_choosen_cond, S_BusinessObject *business_object)
{
Cnds_normal_hm the_in_cond_choice = 0;// avoid warnings
TOMSG_blablabla to_blablabla;
TOMSG_or to_or;
struct Msg_hm_real * a_hm_real = 0;// the inmessage. (avoid unused warning)
a_hm_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Hardcheck */
  the_in_cond_choice = Cnd_normal_hm_1;
  break;
case 1:
  /* Original condition value: Lot */
  the_in_cond_choice = Cnd_normal_hm_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "S_::Cnds_normal_hm out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_normal_hm_1:
/* Original condition value: Hardcheck */
to_or = FROM_Cnd_normal_hm_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_or(self->theProdObj, to_or, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_or 
}
break;
case Cnd_normal_hm_2:
/* Original condition value: Lot */
to_blablabla = FROM_Cnd_normal_hm_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_blablabla(self->theProdObj, to_blablabla, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_blablabla 
}
break;
}
return how;
}
                
    
/*--- Normal_Well -------------------- */
static int S_normal_well(S_GI* self, Cnds_normal_well *ret_choosen_cond, S_BusinessObject *business_object)
{
Cnds_normal_well the_in_cond_choice = 0;// avoid warnings
TOMSG_blablabla to_blablabla;
TOMSG_bye_bob to_bye_bob;
struct Msg_well_real * a_well_real = 0;// the inmessage. (avoid unused warning)
a_well_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Bad */
  the_in_cond_choice = Cnd_normal_well_1;
  break;
case 1:
  /* Original condition value: Lot */
  the_in_cond_choice = Cnd_normal_well_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "S_::Cnds_normal_well out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_normal_well_1:
/* Original condition value: Bad */
to_bye_bob = FROM_Cnd_normal_well_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_bye_bob(self->theProdObj, to_bye_bob, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_bye_bob 
}
break;
case Cnd_normal_well_2:
/* Original condition value: Lot */
to_blablabla = FROM_Cnd_normal_well_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_blablabla(self->theProdObj, to_blablabla, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_blablabla 
}
break;
}
return how;
}
                
    
/*--- StartUp_How_Are_You -------------------- */
static int S_startup_how_are_you(S_GI* self, Cnds_startup_how_are_you *ret_choosen_cond, S_BusinessObject *business_object)
{
Cnds_startup_how_are_you the_in_cond_choice = 0;// avoid warnings
TOMSG_rich to_rich;
TOMSG_tired to_tired;
struct Msg_how_are_you_real * a_how_are_you_real = 0;// the inmessage. (avoid unused warning)
a_how_are_you_real = self->theMsgFactoryObj->theInMsg.a_real_message;
int how = 0;
// 1. Assign value to 'the_in_cond_choice' as NOTE above
// 2. Or use the VERIFY / TEST block 
/*########## VERIFY / TEST STARTS HERE ######################*/
int nr_choices = 2;
int choice = rand() % nr_choices;
//choice=10000; // for verify purposes
switch (choice) {
case 0:
  /* Original condition value: Good */
  the_in_cond_choice = Cnd_startup_how_are_you_1;
  break;
case 1:
  /* Original condition value: Tired */
  the_in_cond_choice = Cnd_startup_how_are_you_2;
  break;
default:
  snprintf(self->theMsgFactoryObj->sprint_dest_buff,SIZE_SPRINTF_DEST_BUFF,
    "S_::Cnds_startup_how_are_you out of range. %i\n", choice);//log
  self->theMsgFactoryObj->setup_return_error_MSGFACTORY(
    self->theMsgFactoryObj,
    self->theMsgFactoryObj->sprint_dest_buff);
  return 0;// return now
  break;
}
/*############## VERIFY / TEST ENDS HERE ###################*/
switch (the_in_cond_choice) {
case Cnd_startup_how_are_you_1:
/* Original condition value: Good */
to_rich = FROM_Cnd_startup_how_are_you_1;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_rich(self->theProdObj, to_rich, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_rich 
}
break;
case Cnd_startup_how_are_you_2:
/* Original condition value: Tired */
to_tired = FROM_Cnd_startup_how_are_you_2;

// Then produce an out message and deliver it back
how =  self->theProdObj->vtable->S_prod_tired(self->theProdObj, to_tired, business_object);
if (how == 1)
{
  *ret_choosen_cond = the_in_cond_choice;
} else
{
  // content of error is set up in prod_tired 
}
break;
}
return how;
}
                
    
