// Server side. Alice
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "S_FSM.h"
/******************************/

struct wide_variabels
{
  // the server object FSM
  S_FSM server_fsm;
  S_FSM *that_fsm_server;
  int state;

  // the server object GI
  S_GI gi_server;
  S_GI *that_gi_server;

  // the server object PI
  S_PI pi_server;
  S_PI *that_pi_server;

  // server object BO
  S_BusinessObject bo_server;
  S_BusinessObject *that_bo_server;

  // messagefactory
  MSGFACTORY message_factory;
  MSGFACTORY *that_message_factory_C;
};
typedef struct wide_variabels WIDES;

/******************************/
int init_objects(WIDES *theWides)
{
  // INITIALIZE THE INSTACES

  theWides->that_message_factory_C = in_MSGFACTORY(&theWides->message_factory);

  theWides->that_fsm_server = in_S_FSM(&theWides->server_fsm);
  theWides->that_gi_server = in_S_GI(&theWides->gi_server);
  theWides->that_pi_server = in_S_PI(&theWides->pi_server);
  theWides->that_bo_server = in_S_BusinessObject(&theWides->bo_server);

  // chain the server objects
  theWides->that_fsm_server->the_gi = &theWides->gi_server;
  theWides->that_gi_server->theProdObj = &theWides->pi_server;

  theWides->that_fsm_server->theMsgFactoryObj = theWides->that_message_factory_C;
  theWides->that_pi_server->theMsgFactoryObj = theWides->that_message_factory_C;
  theWides->that_gi_server->theMsgFactoryObj = theWides->that_message_factory_C;

  theWides->state = getinitialstate_S_FSM(theWides->that_fsm_server);
  return 1;
}
/******************************/
int delete_objects(WIDES *theWides)
{
  deleteself_MSGFACTORY(theWides->that_message_factory_C);
  deleteself_S_FSM(theWides->that_fsm_server);
  deleteself_S_GI(theWides->that_gi_server);
  deleteself_S_PI(theWides->that_pi_server);
  deleteself_S_BusinessObject(theWides->that_bo_server);

  deleteself_MSGFACTORY(theWides->that_message_factory_C);
  return 1;
}
/******************************/
int main(int argc, char **argv)
{
  WIDES theWides;

  int seed = 0;
  int how;
  INMSG *an_inmsg;
#define MAXLINE 1023
  char wire_in[MAXLINE];
  size_t wire_in_len;

  unsigned char *wire_out;
  size_t wire_out_len;

  if (argc > 2)
  { //main --seed 61061
    sscanf(argv[2], "%i", &seed);
  }
  srand(seed);

  init_objects(&theWides);
  FILE *infile;
  infile = fopen("stdinSERVER.txt", "r");
  while (fgets(wire_in, MAXLINE, infile) != 0)
  {
    // stdin/out happens to be the same format as comp
    wire_in_len = strlen(wire_in) - 1;
    an_inmsg = theWides.that_message_factory_C->wire2comp_MSGFACTORY(theWides.that_message_factory_C,
                                                                     (unsigned char *)wire_in,
                                                                     wire_in_len);
    if (an_inmsg == 0)
      return 0;
    how = theWides.that_fsm_server->take_event_S_FSM(
        theWides.that_fsm_server, &theWides.state, theWides.that_bo_server);
    if (how == 0)
    {
      fprintf(stderr, "ERROR: take_event_S_FSM()::2:how=%s\n",
              theWides.that_message_factory_C->theOutMessage.errortext);
      free(theWides.that_message_factory_C->theOutMessage.errortext);
      break;
    }
    wire_out = theWides.that_message_factory_C->comp2wire_MSGFACTORY(
        theWides.that_message_factory_C, &wire_out_len);

    const char *inmsg =
        MESSAGES_LOOKUP[theWides.that_message_factory_C->theInMsg.thetype].msg_name;
    const char *outmsg =
        MESSAGES_LOOKUP[theWides.that_message_factory_C->theOutMessage.msgout.thetype].msg_name;
    printf("Server >>>>>%*s%*s>>> Server\n",
           10 + (int)strlen(inmsg) / 2, inmsg, 10 - (int)strlen(inmsg) / 2, "");
    printf("Server <<<<<%*s%*s<<< Server\n",
           10 + (int)strlen(outmsg) / 2, outmsg, 10 - (int)strlen(outmsg) / 2, "");

    free_real_outmsg_MSGFACTORY(theWides.that_message_factory_C);
    if (theWides.that_fsm_server->isinfinalstate_S_FSM(theWides.that_fsm_server, theWides.state))
    {
      fprintf(stderr, "The server went into final state. No more In Messages are allowed.\n");
      break;
    }
//  show out message to stdout
//    fputs((char *)wire_out, stdout);
//    fputs("\n", stdout);
  };
  return 1;
}