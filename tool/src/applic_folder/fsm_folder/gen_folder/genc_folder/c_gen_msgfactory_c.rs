use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_msg_id;
use std::collections::BTreeSet;
pub fn c_gen_msgfactory_c(all_messages: &BTreeSet<String>) -> String {
    /*++++++++++++++++++++++++++++++++++++++++++++*/
    let build_allinmsgtypes = || {
        all_messages
            .iter()
            .map(|msgout| {
                format!(
                    "case {msgout}:
self->theInMsg.a_real_message  = 
malloc(sizeof(struct {msgout}_real));
memset (self->theInMsg.a_real_message , 'a' + (char){msgout}, sizeof(struct {msgout}_real));
self->theInMsg.thetype ={msgout};
self->inallocated = 1;
break;",
                    msgout = build_msg_id(msgout)
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    };
    /*++++++++++++++++++++++++++++++++++++++++++++*/
    let build_alloutmsgtypes = || {
        all_messages
          .iter()
          .map(|msgout| {
              format!(
                  "case {msgout}:
self->theOutMessage.msgout.a_real_message   =
malloc(sizeof (struct {msgout}_real));
memset (self->theOutMessage.msgout.a_real_message , 'a' + (char){msgout}, sizeof(struct {msgout}_real));
self->theOutMessage.msgout.thetype ={msgout};
break;",
                  msgout = build_msg_id(msgout)
              )
          })
          .collect::<Vec<String>>()
          .join("\n")
    };
    /*++++++++++++++++++++++++++++++++++++++++++++*/
    let build_comp2wire_all_out_messages = || {
        all_messages
            .iter()
            .map(|msgout| {
                format!(
                    "case {msgout}:
  tag_len = strlen(\"{msgout},\");
  *out_wire_len = tag_len +sizeof (struct {msgout}_real) +1;
  ret = malloc((size_t) *out_wire_len );
  strcpy(ret, \"{msgout},\");
  memcpy(ret + tag_len, self->theOutMessage.msgout.a_real_message, sizeof (struct {msgout}_real));
  break;",
                    msgout = build_msg_id(msgout)
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    };
    /*++++++++++++++++++++++++++++++++++++++++++++*/
    let build_wire2comp_all_out_messages = || {
        all_messages
            .iter()
            .map(|msgout| {
                format!(
                    "if (strcmp((char*)in_wire_as_vec_u8,\"{msgout}\")==0){{
self->theInMsg = *self->alloc_real_inmsg_MSGFACTORY(self, {msgout});
struct Msg_{smallmsg}_real *a_{smallmsg}_real = 0;
a_{smallmsg}_real = self->theInMsg.a_real_message;
memcpy(self->theInMsg.a_real_message, comma_pos + 1, sizeof(struct {msgout}_real));
return &self->theInMsg;
}}",
                    smallmsg = msgout.to_lowercase(),
                    msgout = build_msg_id(msgout)
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    };
    /*++++++++++++++++++++++++++++++++++++++++++++*/

    return format!(
        "#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include \"MsgFactory.h\"

MSGFACTORY * in_MSGFACTORY(MSGFACTORY * self)
{{
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
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
MSGFACTORY * nw_MSGFACTORY(void)
{{
MSGFACTORY * self = malloc(sizeof (MSGFACTORY));
self = in_MSGFACTORY(self);
self->selfmallocated =1; // Is in dynamic memory
return self;
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void deleteself_MSGFACTORY(MSGFACTORY * self)
{{
if (self->selfmallocated)
{{
  free(self);
}}
return;
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
INMSG * wire2comp_MSGFACTORY(MSGFACTORY *self, unsigned char in_wire_as_vec_u8[],size_t in_wire_len)
{{
// Find position of comma ','
// Isolate the msg tag
(void)in_wire_len;
char *comma_pos = strchr((char*)in_wire_as_vec_u8,',');
*comma_pos = 0;
{wire2comp_all_out_messages}
fprintf(stderr,\"ERROR:wire2comp_MSGFACTORY() Unknown inmessage '%s'\",
(char *)in_wire_as_vec_u8);

return 0;
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
unsigned char * comp2wire_MSGFACTORY(MSGFACTORY *self,size_t *out_wire_len)
{{
  *out_wire_len =0;
  char *ret = 0;
  size_t tag_len;
  switch (self->theOutMessage.msgout.thetype) {{
    {comp2wire_all_out_messages}
  }};
  return (unsigned char*) ret;
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
INMSG * alloc_real_inmsg_MSGFACTORY(MSGFACTORY * self, MESSAGETYPES thetype)
{{
switch (thetype) {{
  {allinmsgtypes}
  }};
  return &self->theInMsg;
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void * alloc_real_outmsg_MSGFACTORY(MSGFACTORY * self, MESSAGETYPES thetype)
{{
switch (thetype) {{
  {alloutmsgtypes}
  }};
  return self->theOutMessage.msgout.a_real_message;
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void free_real_inmsg_MSGFACTORY(MSGFACTORY * self)
{{
  if (self->theInMsg.a_real_message != 0)
    free(self->theInMsg.a_real_message);
  self->theInMsg.a_real_message = 0;
  return;
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void free_real_outmsg_MSGFACTORY(MSGFACTORY * self)
{{
  if (self->theOutMessage.msgout.a_real_message != 0 )
    free(self->theOutMessage.msgout.a_real_message);
  self->theOutMessage.msgout.a_real_message = 0;
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void * getadr_inmsg_MSGFACTORY(MSGFACTORY * self)
{{
return &self->theInMsg;
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void * getadr_outmsg_MSGFACTORY(MSGFACTORY * self)
{{
return &self->theOutMessage;
}}
/*++++++++++++++++++++++++++++++++++++++++++++++++++*/
void setup_return_error_MSGFACTORY(MSGFACTORY * self, char *errtxt)
{{
if (self->theOutMessage.msgout.result == ERR)
{{
  fprintf(stderr, \"Severe: Tried to set new error when having old error.\\n\");
  return;
}}
if (self->theOutMessage.msgout.result == OK)
{{
  free(self->theOutMessage.msgout.a_real_message);
}}
size_t errtxt_len = 1+strlen(errtxt);
self->theOutMessage.errortext = malloc(errtxt_len);
memcpy(self->theOutMessage.errortext,errtxt,errtxt_len);// safe
self->theOutMessage.msgout.result = ERR;
}}
",
        allinmsgtypes = build_allinmsgtypes(),
        alloutmsgtypes = build_alloutmsgtypes(),
        comp2wire_all_out_messages = build_comp2wire_all_out_messages(),
        wire2comp_all_out_messages = build_wire2comp_all_out_messages()
    );
}
