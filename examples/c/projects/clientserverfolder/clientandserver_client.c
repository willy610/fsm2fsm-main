
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


#include "clientandserver_emulator.h"
#include "C_FSM.h"

// the client object FSM
static C_FSM client_fsm;
static C_FSM *self_fsm_client;

// the client object GI

static C_GI gi_client;
static C_GI *self_gi_client;

// the client object PI

static C_PI pi_client;
static C_PI *self_pi_client;

// client object BO
static C_BusinessObject bo_client;
static C_BusinessObject *self_bo_client;

/**#############################################*/
int init_client_main(MSGFACTORY *glob_mess_fact)
{
  // INITIALIZE THE INSTACES

  self_fsm_client = in_C_FSM(&client_fsm);
  self_gi_client = in_C_GI(&gi_client);
  self_pi_client = in_C_PI(&pi_client);
  self_bo_client = in_C_BusinessObject(&bo_client);


  // chain the client objects
  self_fsm_client->the_gi = &gi_client;
  self_gi_client->theProdObj = &pi_client;

  self_fsm_client->theMsgFactoryObj = glob_mess_fact;
  self_pi_client->theMsgFactoryObj = glob_mess_fact;
  self_gi_client->theMsgFactoryObj = glob_mess_fact;

  return 1;
};

/**#############################################*/
int client_main()
{
  unsigned char *wire_out;
  unsigned char *wire_in;
  size_t wire_out_len;
  size_t wire_read_len;

  int maxSteps = 25;
  int how_from_fsm;
  int how_from_server;
  int state; 
  // own copy ref to 'theMessageFactory'
  MSGFACTORY *theMessageFactory = self_pi_client->theMsgFactoryObj;

  INIT_MESSAGES *possible_init_message_to_client = get_possible_init_messages_C_FSM(self_fsm_client);
  // pick first one
  int which_start_message = rand() % possible_init_message_to_client->the_size_of_inits;
  MESSAGETYPES messagetype_first_message_to_client = possible_init_message_to_client->inits[which_start_message];
  state= self_fsm_client->getinitialstate_C_FSM(self_fsm_client);
  theMessageFactory->alloc_real_inmsg_MSGFACTORY(
      theMessageFactory, messagetype_first_message_to_client);
  // Send the in message to the FSM
  how_from_fsm = self_fsm_client->take_event_C_FSM(self_fsm_client, &state, self_bo_client);
  // We don't need the in message any more
  theMessageFactory->free_real_inmsg_MSGFACTORY(theMessageFactory);
  if (how_from_fsm == 0)
  {
    fprintf(stderr, "client_main() theMessageFactory->theOutMessage.msgout.result '%i' \n",
            theMessageFactory->theOutMessage.msgout.result);
    fprintf(stderr, "client_main() theMessageFactory->theOutMessage->errortext  %s \n",
            theMessageFactory->theOutMessage.errortext);
    free(self_pi_client->theMsgFactoryObj->theOutMessage.errortext);
    return (EXIT_FAILURE);
  }
  while (maxSteps > 0)
  {
    wire_out = theMessageFactory->comp2wire_MSGFACTORY(theMessageFactory,
                                                       &wire_out_len);
    // write a message and get one back
    how_from_server = client_wire_write_and_read_UDP(wire_out, wire_out_len, &wire_in, &wire_read_len);
    if (how_from_server == 0)
      return 0;
    if (wire_in == 0)
      return 0; // severe
                // convert wire to msg
    theMessageFactory->wire2comp_MSGFACTORY(theMessageFactory, wire_in, wire_read_len);

    free(wire_in);
    wire_in = 0;
    how_from_fsm = self_fsm_client->take_event_C_FSM(self_fsm_client, &state, self_bo_client);
    theMessageFactory->free_real_inmsg_MSGFACTORY(theMessageFactory);
    if (self_fsm_client->isinfinalstate_C_FSM(self_fsm_client, state))
    {
      fprintf(stderr, "Client is in finalstate\n");
      break;
    }
    --maxSteps;
  }
  return 1;
};

/**#############################################*/

int client_wire_write_and_read_UDP(unsigned char *wire_out, int outlen, unsigned char **wiren_in, size_t *wire_read_len)
{
  // shortcut
  return server_read_and_write_UDP(wire_out, outlen, wiren_in, wire_read_len);
}
