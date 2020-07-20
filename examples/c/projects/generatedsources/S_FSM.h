
#ifndef S_FSM_H
#define S_FSM_H
#include <stdio.h>
#include <stdlib.h>

#include "S_GI.h"

typedef struct S_FSM S_FSM;

struct init_messages
{
  int the_size_of_inits;
  MESSAGETYPES inits[1];
};
typedef struct init_messages INIT_MESSAGES;

void deleteself_S_FSM(S_FSM * self);

int getinitialstate_S_FSM(S_FSM * self);
int getfinalstate_S_FSM(S_FSM * self);
int isinfinalstate_S_FSM(S_FSM * self, int state);
INIT_MESSAGES * get_possible_init_messages_S_FSM(S_FSM * self);
int take_event_S_FSM(S_FSM * self, int *state, void *business_object);

struct S_FSM {
  int mallocated;
  MSGFACTORY * theMsgFactoryObj;
  S_GI *the_gi;
  void (*deleteself_S_FSM)(S_FSM * self);
  int (*getinitialstate_S_FSM)(S_FSM * self);
  int (*getfinalstate_S_FSM)(S_FSM * self);
  int (*isinfinalstate_S_FSM)(S_FSM * self, int state);
  INIT_MESSAGES * (*get_possible_init_messages_S_FSM)(S_FSM * self);
  int (*take_event_S_FSM)(S_FSM * self, int *state, void *business_object);
};

S_FSM * in_S_FSM(S_FSM * self);
S_FSM * nw_S_FSM(void);
#endif /* S_FSM_H */
