
#ifndef C_FSM_H
#define C_FSM_H
#include <stdio.h>
#include <stdlib.h>

#include "C_GI.h"

typedef struct C_FSM C_FSM;

struct init_messages
{
  int the_size_of_inits;
  MESSAGETYPES inits[1];
};
typedef struct init_messages INIT_MESSAGES;

void deleteself_C_FSM(C_FSM * self);

int getinitialstate_C_FSM(C_FSM * self);
int getfinalstate_C_FSM(C_FSM * self);
int isinfinalstate_C_FSM(C_FSM * self, int state);
INIT_MESSAGES * get_possible_init_messages_C_FSM(C_FSM * self);
int take_event_C_FSM(C_FSM * self, int *state, void *business_object);

struct C_FSM {
  int mallocated;
  MSGFACTORY * theMsgFactoryObj;
  C_GI *the_gi;
  void (*deleteself_C_FSM)(C_FSM * self);
  int (*getinitialstate_C_FSM)(C_FSM * self);
  int (*getfinalstate_C_FSM)(C_FSM * self);
  int (*isinfinalstate_C_FSM)(C_FSM * self, int state);
  INIT_MESSAGES * (*get_possible_init_messages_C_FSM)(C_FSM * self);
  int (*take_event_C_FSM)(C_FSM * self, int *state, void *business_object);
};

C_FSM * in_C_FSM(C_FSM * self);
C_FSM * nw_C_FSM(void);
#endif /* C_FSM_H */
