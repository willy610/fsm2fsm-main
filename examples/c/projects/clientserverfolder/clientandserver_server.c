#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "clientandserver_emulator.h"
#include "S_FSM.h"

static S_FSM server_fsm;
static S_FSM *self_fsm_server;

// the server object GI

static S_GI gi_server;
static S_GI *self_gi_server;

// the server object PI

static S_PI pi_server;
static S_PI *self_pi_server;

// server object BO

static S_BusinessObject bo_server;
static S_BusinessObject *self_bo_server;
static int state;

/**#############################################*/
int init_server_main(MSGFACTORY *glob_mess_fact)
{
  // INITIALIZE THE INSTACES

  self_fsm_server = in_S_FSM(&server_fsm);
  self_gi_server = in_S_GI(&gi_server);
  self_pi_server = in_S_PI(&pi_server);
  self_bo_server = in_S_BusinessObject(&bo_server);

  // chain the server objects
  self_fsm_server->the_gi = &gi_server;    // THIS IMPLEMENTATION
  self_gi_server->theProdObj = &pi_server; // THIS IMPLEMENTATION

  self_fsm_server->theMsgFactoryObj = glob_mess_fact;
  self_pi_server->theMsgFactoryObj = glob_mess_fact;
  self_gi_server->theMsgFactoryObj = glob_mess_fact;

  state = getinitialstate_S_FSM(self_fsm_server);
  return 1;
};

/**#############################################*/
int server_read_and_write_UDP(unsigned char *wire_in, int outlen, unsigned char **wire_out, size_t *wire_read_len)
{
  *wire_read_len = 0;
  int how;
  INMSG *an_inmsg;
  MSGFACTORY *theMessageFactory = self_pi_server->theMsgFactoryObj;

  an_inmsg = theMessageFactory->wire2comp_MSGFACTORY(theMessageFactory,
                                                     wire_in, *wire_read_len);
  if (an_inmsg == 0)
    return 0;
  how = self_fsm_server->take_event_S_FSM(self_fsm_server, &state, self_bo_server);

  if (how == 0)
  {
    fprintf(stderr, "ERROR: server_read_write_UDP()::2:how=%s\n",
            theMessageFactory->theOutMessage.errortext);
    free(theMessageFactory->theOutMessage.errortext);
    *wire_out = 0;
    *wire_read_len = 0;
    return 0;
  }

  *wire_out = theMessageFactory->comp2wire_MSGFACTORY(theMessageFactory,
                                                      wire_read_len);
  const char *inmsg = MESSAGES_LOOKUP[theMessageFactory->theInMsg.thetype].msg_name;
  const char *outmsg = MESSAGES_LOOKUP[theMessageFactory->theOutMessage.msgout.thetype].msg_name;
  printf("Client >>>>>%*s%*s>>> Server\n", 10 + (int)strlen(inmsg) / 2, inmsg, 10 - (int)strlen(inmsg) / 2, "");
  printf("Client <<<<<%*s%*s<<< Server\n", 10 + (int)strlen(outmsg) / 2, outmsg, 10 - (int)strlen(outmsg) / 2, "");

  free_real_outmsg_MSGFACTORY(theMessageFactory);

  if (isinfinalstate_S_FSM(self_fsm_server, state))
  {
    fprintf(stderr, "Server is in finalstate\n");
  }
  return 1;
};
