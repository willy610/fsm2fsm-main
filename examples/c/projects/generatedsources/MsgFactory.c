#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "MsgFactory.h"

MSGFACTORY * in_MSGFACTORY(MSGFACTORY * self)
{
self->selfmallocated =0;
self->inallocated = 0;
self->outallocated = 0;

self->deleteself_MSGFACTORY = &deleteself_MSGFACTORY;
self->alloc_real_inmsg_MSGFACTORY = &alloc_real_inmsg_MSGFACTORY;
self->alloc_real_outmsg_MSGFACTORY = &alloc_real_outmsg_MSGFACTORY;
self->free_real_inmsg_MSGFACTORY = &free_real_inmsg_MSGFACTORY;
self->free_real_outmsg_MSGFACTORY = &free_real_outmsg_MSGFACTORY;
self->getadr_inmsg_MSGFACTORY = &getadr_inmsg_MSGFACTORY;
self->getadr_outmsg_MSGFACTORY = &getadr_outmsg_MSGFACTORY;
self->setup_return_error_MSGFACTORY = &setup_return_error_MSGFACTORY;
self->wire2comp_MSGFACTORY = &wire2comp_MSGFACTORY;
self->comp2wire_MSGFACTORY = &comp2wire_MSGFACTORY;

return self;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
MSGFACTORY * nw_MSGFACTORY(void)
{
MSGFACTORY * self = malloc(sizeof (MSGFACTORY));
self = in_MSGFACTORY(self);
self->selfmallocated =1; // Is in dynamic memory
return self;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void deleteself_MSGFACTORY(MSGFACTORY * self)
{
if (self->selfmallocated)
{
  free(self);
}
return;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
INMSG * wire2comp_MSGFACTORY(MSGFACTORY *self, unsigned char in_wire_as_vec_u8[],size_t in_wire_len)
{
// Find position of comma ','
// Isolate the msg tag
(void)in_wire_len;
char *comma_pos = strchr((char*)in_wire_as_vec_u8,',');
*comma_pos = 0;
if (strcmp((char*)in_wire_as_vec_u8,"Msg_24")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_24);
struct Msg_24_real *a_24_real = 0;
a_24_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_24_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_42")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_42);
struct Msg_42_real *a_42_real = 0;
a_42_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_42_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_43")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_43);
struct Msg_43_real *a_43_real = 0;
a_43_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_43_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_blablabla")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_blablabla);
struct Msg_blablabla_real *a_blablabla_real = 0;
a_blablabla_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_blablabla_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_bye")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_bye);
struct Msg_bye_real *a_bye_real = 0;
a_bye_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_bye_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_bye_bob")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_bye_bob);
struct Msg_bye_bob_real *a_bye_bob_real = 0;
a_bye_bob_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_bye_bob_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_callin")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_callin);
struct Msg_callin_real *a_callin_real = 0;
a_callin_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_callin_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_dontdisturb")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_dontdisturb);
struct Msg_dontdisturb_real *a_dontdisturb_real = 0;
a_dontdisturb_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_dontdisturb_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_hello_alice")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_hello_alice);
struct Msg_hello_alice_real *a_hello_alice_real = 0;
a_hello_alice_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_hello_alice_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_hello_bob")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_hello_bob);
struct Msg_hello_bob_real *a_hello_bob_real = 0;
a_hello_bob_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_hello_bob_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_hm")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_hm);
struct Msg_hm_real *a_hm_real = 0;
a_hm_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_hm_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_how_are_you")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_how_are_you);
struct Msg_how_are_you_real *a_how_are_you_real = 0;
a_how_are_you_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_how_are_you_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_no")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_no);
struct Msg_no_real *a_no_real = 0;
a_no_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_no_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_no_message_to_send")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_no_message_to_send);
struct Msg_no_message_to_send_real *a_no_message_to_send_real = 0;
a_no_message_to_send_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_no_message_to_send_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_nooutput")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_nooutput);
struct Msg_nooutput_real *a_nooutput_real = 0;
a_nooutput_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_nooutput_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_or")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_or);
struct Msg_or_real *a_or_real = 0;
a_or_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_or_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_other")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_other);
struct Msg_other_real *a_other_real = 0;
a_other_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_other_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_ping")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_ping);
struct Msg_ping_real *a_ping_real = 0;
a_ping_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_ping_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_pong")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_pong);
struct Msg_pong_real *a_pong_real = 0;
a_pong_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_pong_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_questionwas")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_questionwas);
struct Msg_questionwas_real *a_questionwas_real = 0;
a_questionwas_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_questionwas_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_rich")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_rich);
struct Msg_rich_real *a_rich_real = 0;
a_rich_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_rich_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_sorry")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_sorry);
struct Msg_sorry_real *a_sorry_real = 0;
a_sorry_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_sorry_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_tired")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_tired);
struct Msg_tired_real *a_tired_real = 0;
a_tired_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_tired_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_well")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_well);
struct Msg_well_real *a_well_real = 0;
a_well_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_well_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_what")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_what);
struct Msg_what_real *a_what_real = 0;
a_what_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_what_real));
return &self->theInMsg;
}
if (strcmp((char*)in_wire_as_vec_u8,"Msg_yes")==0){
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, Msg_yes);
struct Msg_yes_real *a_yes_real = 0;
a_yes_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct Msg_yes_real));
return &self->theInMsg;
}
fprintf(stderr,"ERROR:wire2comp_MSGFACTORY() Unknown inmessage '%s'",
(char *)in_wire_as_vec_u8);

return 0;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
unsigned char * comp2wire_MSGFACTORY(MSGFACTORY *self,size_t *out_wire_len)
{
  *out_wire_len =0;
  char *ret = 0;
  size_t tag_len;
  switch (self->theOutMessage.msgout.thetype) {
    case Msg_24:
  tag_len = strlen("Msg_24,");
  *out_wire_len = tag_len +sizeof (struct Msg_24_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_24,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_24_real));
  break;
case Msg_42:
  tag_len = strlen("Msg_42,");
  *out_wire_len = tag_len +sizeof (struct Msg_42_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_42,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_42_real));
  break;
case Msg_43:
  tag_len = strlen("Msg_43,");
  *out_wire_len = tag_len +sizeof (struct Msg_43_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_43,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_43_real));
  break;
case Msg_blablabla:
  tag_len = strlen("Msg_blablabla,");
  *out_wire_len = tag_len +sizeof (struct Msg_blablabla_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_blablabla,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_blablabla_real));
  break;
case Msg_bye:
  tag_len = strlen("Msg_bye,");
  *out_wire_len = tag_len +sizeof (struct Msg_bye_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_bye,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_bye_real));
  break;
case Msg_bye_bob:
  tag_len = strlen("Msg_bye_bob,");
  *out_wire_len = tag_len +sizeof (struct Msg_bye_bob_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_bye_bob,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_bye_bob_real));
  break;
case Msg_callin:
  tag_len = strlen("Msg_callin,");
  *out_wire_len = tag_len +sizeof (struct Msg_callin_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_callin,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_callin_real));
  break;
case Msg_dontdisturb:
  tag_len = strlen("Msg_dontdisturb,");
  *out_wire_len = tag_len +sizeof (struct Msg_dontdisturb_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_dontdisturb,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_dontdisturb_real));
  break;
case Msg_hello_alice:
  tag_len = strlen("Msg_hello_alice,");
  *out_wire_len = tag_len +sizeof (struct Msg_hello_alice_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_hello_alice,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_hello_alice_real));
  break;
case Msg_hello_bob:
  tag_len = strlen("Msg_hello_bob,");
  *out_wire_len = tag_len +sizeof (struct Msg_hello_bob_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_hello_bob,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_hello_bob_real));
  break;
case Msg_hm:
  tag_len = strlen("Msg_hm,");
  *out_wire_len = tag_len +sizeof (struct Msg_hm_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_hm,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_hm_real));
  break;
case Msg_how_are_you:
  tag_len = strlen("Msg_how_are_you,");
  *out_wire_len = tag_len +sizeof (struct Msg_how_are_you_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_how_are_you,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_how_are_you_real));
  break;
case Msg_no:
  tag_len = strlen("Msg_no,");
  *out_wire_len = tag_len +sizeof (struct Msg_no_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_no,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_no_real));
  break;
case Msg_no_message_to_send:
  tag_len = strlen("Msg_no_message_to_send,");
  *out_wire_len = tag_len +sizeof (struct Msg_no_message_to_send_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_no_message_to_send,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_no_message_to_send_real));
  break;
case Msg_nooutput:
  tag_len = strlen("Msg_nooutput,");
  *out_wire_len = tag_len +sizeof (struct Msg_nooutput_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_nooutput,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_nooutput_real));
  break;
case Msg_or:
  tag_len = strlen("Msg_or,");
  *out_wire_len = tag_len +sizeof (struct Msg_or_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_or,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_or_real));
  break;
case Msg_other:
  tag_len = strlen("Msg_other,");
  *out_wire_len = tag_len +sizeof (struct Msg_other_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_other,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_other_real));
  break;
case Msg_ping:
  tag_len = strlen("Msg_ping,");
  *out_wire_len = tag_len +sizeof (struct Msg_ping_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_ping,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_ping_real));
  break;
case Msg_pong:
  tag_len = strlen("Msg_pong,");
  *out_wire_len = tag_len +sizeof (struct Msg_pong_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_pong,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_pong_real));
  break;
case Msg_questionwas:
  tag_len = strlen("Msg_questionwas,");
  *out_wire_len = tag_len +sizeof (struct Msg_questionwas_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_questionwas,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_questionwas_real));
  break;
case Msg_rich:
  tag_len = strlen("Msg_rich,");
  *out_wire_len = tag_len +sizeof (struct Msg_rich_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_rich,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_rich_real));
  break;
case Msg_sorry:
  tag_len = strlen("Msg_sorry,");
  *out_wire_len = tag_len +sizeof (struct Msg_sorry_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_sorry,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_sorry_real));
  break;
case Msg_tired:
  tag_len = strlen("Msg_tired,");
  *out_wire_len = tag_len +sizeof (struct Msg_tired_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_tired,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_tired_real));
  break;
case Msg_well:
  tag_len = strlen("Msg_well,");
  *out_wire_len = tag_len +sizeof (struct Msg_well_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_well,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_well_real));
  break;
case Msg_what:
  tag_len = strlen("Msg_what,");
  *out_wire_len = tag_len +sizeof (struct Msg_what_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_what,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_what_real));
  break;
case Msg_yes:
  tag_len = strlen("Msg_yes,");
  *out_wire_len = tag_len +sizeof (struct Msg_yes_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, "Msg_yes,");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct Msg_yes_real));
  break;
  };
  return (unsigned char*) ret;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
INMSG * alloc_real_inmsg_MSGFACTORY(MSGFACTORY * self, MESSAGETYPES thetype)
{
switch (thetype) {
  case Msg_24:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_24_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_24, sizeof(struct Msg_24_real));
self->theInMsg.thetype =Msg_24;
self->inallocated = 1;
break;
case Msg_42:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_42_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_42, sizeof(struct Msg_42_real));
self->theInMsg.thetype =Msg_42;
self->inallocated = 1;
break;
case Msg_43:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_43_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_43, sizeof(struct Msg_43_real));
self->theInMsg.thetype =Msg_43;
self->inallocated = 1;
break;
case Msg_blablabla:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_blablabla_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_blablabla, sizeof(struct Msg_blablabla_real));
self->theInMsg.thetype =Msg_blablabla;
self->inallocated = 1;
break;
case Msg_bye:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_bye_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_bye, sizeof(struct Msg_bye_real));
self->theInMsg.thetype =Msg_bye;
self->inallocated = 1;
break;
case Msg_bye_bob:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_bye_bob_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_bye_bob, sizeof(struct Msg_bye_bob_real));
self->theInMsg.thetype =Msg_bye_bob;
self->inallocated = 1;
break;
case Msg_callin:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_callin_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_callin, sizeof(struct Msg_callin_real));
self->theInMsg.thetype =Msg_callin;
self->inallocated = 1;
break;
case Msg_dontdisturb:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_dontdisturb_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_dontdisturb, sizeof(struct Msg_dontdisturb_real));
self->theInMsg.thetype =Msg_dontdisturb;
self->inallocated = 1;
break;
case Msg_hello_alice:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_hello_alice_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_hello_alice, sizeof(struct Msg_hello_alice_real));
self->theInMsg.thetype =Msg_hello_alice;
self->inallocated = 1;
break;
case Msg_hello_bob:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_hello_bob_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_hello_bob, sizeof(struct Msg_hello_bob_real));
self->theInMsg.thetype =Msg_hello_bob;
self->inallocated = 1;
break;
case Msg_hm:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_hm_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_hm, sizeof(struct Msg_hm_real));
self->theInMsg.thetype =Msg_hm;
self->inallocated = 1;
break;
case Msg_how_are_you:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_how_are_you_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_how_are_you, sizeof(struct Msg_how_are_you_real));
self->theInMsg.thetype =Msg_how_are_you;
self->inallocated = 1;
break;
case Msg_no:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_no_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_no, sizeof(struct Msg_no_real));
self->theInMsg.thetype =Msg_no;
self->inallocated = 1;
break;
case Msg_no_message_to_send:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_no_message_to_send_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_no_message_to_send, sizeof(struct Msg_no_message_to_send_real));
self->theInMsg.thetype =Msg_no_message_to_send;
self->inallocated = 1;
break;
case Msg_nooutput:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_nooutput_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_nooutput, sizeof(struct Msg_nooutput_real));
self->theInMsg.thetype =Msg_nooutput;
self->inallocated = 1;
break;
case Msg_or:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_or_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_or, sizeof(struct Msg_or_real));
self->theInMsg.thetype =Msg_or;
self->inallocated = 1;
break;
case Msg_other:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_other_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_other, sizeof(struct Msg_other_real));
self->theInMsg.thetype =Msg_other;
self->inallocated = 1;
break;
case Msg_ping:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_ping_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_ping, sizeof(struct Msg_ping_real));
self->theInMsg.thetype =Msg_ping;
self->inallocated = 1;
break;
case Msg_pong:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_pong_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_pong, sizeof(struct Msg_pong_real));
self->theInMsg.thetype =Msg_pong;
self->inallocated = 1;
break;
case Msg_questionwas:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_questionwas_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_questionwas, sizeof(struct Msg_questionwas_real));
self->theInMsg.thetype =Msg_questionwas;
self->inallocated = 1;
break;
case Msg_rich:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_rich_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_rich, sizeof(struct Msg_rich_real));
self->theInMsg.thetype =Msg_rich;
self->inallocated = 1;
break;
case Msg_sorry:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_sorry_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_sorry, sizeof(struct Msg_sorry_real));
self->theInMsg.thetype =Msg_sorry;
self->inallocated = 1;
break;
case Msg_tired:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_tired_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_tired, sizeof(struct Msg_tired_real));
self->theInMsg.thetype =Msg_tired;
self->inallocated = 1;
break;
case Msg_well:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_well_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_well, sizeof(struct Msg_well_real));
self->theInMsg.thetype =Msg_well;
self->inallocated = 1;
break;
case Msg_what:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_what_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_what, sizeof(struct Msg_what_real));
self->theInMsg.thetype =Msg_what;
self->inallocated = 1;
break;
case Msg_yes:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct Msg_yes_real));
memset (self->theInMsg.a_real_message , 'a' + (char)Msg_yes, sizeof(struct Msg_yes_real));
self->theInMsg.thetype =Msg_yes;
self->inallocated = 1;
break;
  };
  return &self->theInMsg;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void * alloc_real_outmsg_MSGFACTORY(MSGFACTORY * self, MESSAGETYPES thetype)
{
switch (thetype) {
  case Msg_24:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_24_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_24, sizeof(struct Msg_24_real));
self->theOutMessage.msgout.thetype =Msg_24;
break;
case Msg_42:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_42_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_42, sizeof(struct Msg_42_real));
self->theOutMessage.msgout.thetype =Msg_42;
break;
case Msg_43:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_43_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_43, sizeof(struct Msg_43_real));
self->theOutMessage.msgout.thetype =Msg_43;
break;
case Msg_blablabla:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_blablabla_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_blablabla, sizeof(struct Msg_blablabla_real));
self->theOutMessage.msgout.thetype =Msg_blablabla;
break;
case Msg_bye:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_bye_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_bye, sizeof(struct Msg_bye_real));
self->theOutMessage.msgout.thetype =Msg_bye;
break;
case Msg_bye_bob:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_bye_bob_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_bye_bob, sizeof(struct Msg_bye_bob_real));
self->theOutMessage.msgout.thetype =Msg_bye_bob;
break;
case Msg_callin:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_callin_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_callin, sizeof(struct Msg_callin_real));
self->theOutMessage.msgout.thetype =Msg_callin;
break;
case Msg_dontdisturb:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_dontdisturb_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_dontdisturb, sizeof(struct Msg_dontdisturb_real));
self->theOutMessage.msgout.thetype =Msg_dontdisturb;
break;
case Msg_hello_alice:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_hello_alice_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_hello_alice, sizeof(struct Msg_hello_alice_real));
self->theOutMessage.msgout.thetype =Msg_hello_alice;
break;
case Msg_hello_bob:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_hello_bob_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_hello_bob, sizeof(struct Msg_hello_bob_real));
self->theOutMessage.msgout.thetype =Msg_hello_bob;
break;
case Msg_hm:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_hm_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_hm, sizeof(struct Msg_hm_real));
self->theOutMessage.msgout.thetype =Msg_hm;
break;
case Msg_how_are_you:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_how_are_you_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_how_are_you, sizeof(struct Msg_how_are_you_real));
self->theOutMessage.msgout.thetype =Msg_how_are_you;
break;
case Msg_no:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_no_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_no, sizeof(struct Msg_no_real));
self->theOutMessage.msgout.thetype =Msg_no;
break;
case Msg_no_message_to_send:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_no_message_to_send_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_no_message_to_send, sizeof(struct Msg_no_message_to_send_real));
self->theOutMessage.msgout.thetype =Msg_no_message_to_send;
break;
case Msg_nooutput:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_nooutput_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_nooutput, sizeof(struct Msg_nooutput_real));
self->theOutMessage.msgout.thetype =Msg_nooutput;
break;
case Msg_or:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_or_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_or, sizeof(struct Msg_or_real));
self->theOutMessage.msgout.thetype =Msg_or;
break;
case Msg_other:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_other_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_other, sizeof(struct Msg_other_real));
self->theOutMessage.msgout.thetype =Msg_other;
break;
case Msg_ping:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_ping_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_ping, sizeof(struct Msg_ping_real));
self->theOutMessage.msgout.thetype =Msg_ping;
break;
case Msg_pong:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_pong_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_pong, sizeof(struct Msg_pong_real));
self->theOutMessage.msgout.thetype =Msg_pong;
break;
case Msg_questionwas:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_questionwas_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_questionwas, sizeof(struct Msg_questionwas_real));
self->theOutMessage.msgout.thetype =Msg_questionwas;
break;
case Msg_rich:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_rich_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_rich, sizeof(struct Msg_rich_real));
self->theOutMessage.msgout.thetype =Msg_rich;
break;
case Msg_sorry:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_sorry_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_sorry, sizeof(struct Msg_sorry_real));
self->theOutMessage.msgout.thetype =Msg_sorry;
break;
case Msg_tired:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_tired_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_tired, sizeof(struct Msg_tired_real));
self->theOutMessage.msgout.thetype =Msg_tired;
break;
case Msg_well:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_well_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_well, sizeof(struct Msg_well_real));
self->theOutMessage.msgout.thetype =Msg_well;
break;
case Msg_what:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_what_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_what, sizeof(struct Msg_what_real));
self->theOutMessage.msgout.thetype =Msg_what;
break;
case Msg_yes:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct Msg_yes_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char)Msg_yes, sizeof(struct Msg_yes_real));
self->theOutMessage.msgout.thetype =Msg_yes;
break;
  };
  return self->theOutMessage.msgout.a_real_message;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void free_real_inmsg_MSGFACTORY(MSGFACTORY * self)
{
  if (self->theInMsg.a_real_message != 0)
    free(self->theInMsg.a_real_message);
  self->theInMsg.a_real_message = 0;
  return;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void free_real_outmsg_MSGFACTORY(MSGFACTORY * self)
{
  if (self->theOutMessage.msgout.a_real_message != 0 )
    free(self->theOutMessage.msgout.a_real_message);
  self->theOutMessage.msgout.a_real_message = 0;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void * getadr_inmsg_MSGFACTORY(MSGFACTORY * self)
{
return &self->theInMsg;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void * getadr_outmsg_MSGFACTORY(MSGFACTORY * self)
{
return &self->theOutMessage;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void setup_return_error_MSGFACTORY(MSGFACTORY * self, char *errtxt)
{
if (self->theOutMessage.msgout.result == ERR)
{
  fprintf(stderr, "Severe: Tried to set new error when having old error.\n");
  return;
}
if (self->theOutMessage.msgout.result == OK)
{
  free(self->theOutMessage.msgout.a_real_message);
}
size_t errtxt_len = 1+strlen(errtxt);
self->theOutMessage.errortext = malloc(errtxt_len);
memcpy(self->theOutMessage.errortext,errtxt,errtxt_len);// safe
self->theOutMessage.msgout.result = ERR;
}
