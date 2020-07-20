#ifndef MSGFACTORY_H
#define MSGFACTORY_H
#include "Messages.h"

typedef struct MSGFACTORY MSGFACTORY;

void deleteself_MSGFACTORY(MSGFACTORY * self);
INMSG * alloc_real_inmsg_MSGFACTORY(MSGFACTORY * self, MESSAGETYPES thetype);
void * alloc_real_outmsg_MSGFACTORY(MSGFACTORY * self, MESSAGETYPES thetype);
void free_real_inmsg_MSGFACTORY(MSGFACTORY * self);
void free_real_outmsg_MSGFACTORY(MSGFACTORY * self);
void * getadr_inmsg_MSGFACTORY(MSGFACTORY * self);
void * getadr_outmsg_MSGFACTORY(MSGFACTORY * self);
void setup_return_error_MSGFACTORY(MSGFACTORY * self, char *errtxt);
INMSG * wire2comp_MSGFACTORY(MSGFACTORY *self, unsigned char in_wire_as_vec_u8[],size_t in_wire_len);
unsigned char * comp2wire_MSGFACTORY(MSGFACTORY *self, size_t *out_wire_len);

struct MSGFACTORY {
  int selfmallocated;
  int inallocated;
  int outallocated;
  char sprint_dest_buff[SIZE_SPRINTF_DEST_BUFF];
  INMSG theInMsg;
  OUTMESSAGE theOutMessage;
  void (*deleteself_MSGFACTORY)(MSGFACTORY * self);
  INMSG *(* alloc_real_inmsg_MSGFACTORY)(MSGFACTORY * self, MESSAGETYPES thetype);
  void *(* alloc_real_outmsg_MSGFACTORY)(MSGFACTORY * self, MESSAGETYPES thetype);
  void (*free_real_inmsg_MSGFACTORY)(MSGFACTORY * self);
  void (*free_real_outmsg_MSGFACTORY)(MSGFACTORY * self);
  void *(* getadr_inmsg_MSGFACTORY)(MSGFACTORY * self);
  void *(* getadr_outmsg_MSGFACTORY)(MSGFACTORY * self);
  void (*setup_return_error_MSGFACTORY)(MSGFACTORY * self, char *errtxt);
  INMSG * (*wire2comp_MSGFACTORY)(MSGFACTORY *self, unsigned char in_wire_as_vec_u8[],size_t in_wire_len);
  unsigned char * (*comp2wire_MSGFACTORY)(MSGFACTORY *self, size_t *out_wire_len);
};
// Class methods
// For Instances as static or stack 
MSGFACTORY * in_MSGFACTORY(MSGFACTORY * self);
// For Instances in dynamic memory
MSGFACTORY * nw_MSGFACTORY(void);

#endif  /* MSGFACTORY_H */
