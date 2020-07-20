// Client side. Bob

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <sys/types.h>

#if defined(_WIN32)
#include <winsock2.h>
typedef int socklen_t;
#pragma comment(lib, "ws2_32.lib")
#else
#include <unistd.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <netinet/in.h>
#define SOCKET_ERROR -1
#endif

#include "C_FSM.h"

#define PORT 8081
#define MAXLINE 1024

struct wide_variabels
{
  int state;
  // the client object FSM
  C_FSM client_fsm;
  C_FSM *that_fsm_client;

  // the client object GI
  C_GI gi_client;
  C_GI *that_gi_client;

  // the client object PI
  C_PI pi_client;
  C_PI *that_pi_client;

  // client object BO
  C_BusinessObject bo_client;
  C_BusinessObject *that_bo_client;

  // messagefactory
  MSGFACTORY message_factory;
  MSGFACTORY *that_mess_fact;

  // udp
  int sockfd;
  struct sockaddr_in servaddr;
};
typedef struct wide_variabels WIDES;
/******************************/
int init_udp(WIDES *theWides)
{
#if defined(_WIN32)
  WSADATA wsa;
  if (WSAStartup(MAKEWORD(2, 2), &wsa) != 0)
  {
    printf("Failed. Error Code : %d", WSAGetLastError());
    exit(EXIT_FAILURE);
  }
#endif
  if ((theWides->sockfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP)) < 0)
  {
#if defined(_WIN32)
    printf("socked() failed. Error Code : %d", WSAGetLastError());
#else
    perror("socket creation failed");
#endif
    return 0;
  }
  memset(&theWides->servaddr, 0, sizeof(theWides->servaddr));
  // Filling server information
  theWides->servaddr.sin_family = AF_INET;
  theWides->servaddr.sin_port = htons(PORT);
#if defined(_WIN32)
  theWides->servaddr.sin_addr.S_un.S_addr = inet_addr("127.0.0.1");
#else
  theWides->servaddr.sin_addr.s_addr = htonl(INADDR_ANY);
#endif
  return 1;
}
/******************************/
int init_objects(WIDES *theWides)
{
  // INITIALIZE THE INSTACES

  theWides->that_mess_fact = in_MSGFACTORY(&theWides->message_factory);

  theWides->that_fsm_client = in_C_FSM(&theWides->client_fsm);
  theWides->that_gi_client = in_C_GI(&theWides->gi_client);
  theWides->that_pi_client = in_C_PI(&theWides->pi_client);

  theWides->that_bo_client = in_C_BusinessObject(&theWides->bo_client);

  // chain the client objects
  theWides->that_fsm_client->the_gi = &theWides->gi_client;
  theWides->that_gi_client->theProdObj = &theWides->pi_client;

  theWides->that_fsm_client->theMsgFactoryObj = theWides->that_mess_fact;
  theWides->that_pi_client->theMsgFactoryObj = theWides->that_mess_fact;
  theWides->that_gi_client->theMsgFactoryObj = theWides->that_mess_fact;

  theWides->state = getinitialstate_C_FSM(theWides->that_fsm_client);

  return 1;
}
/******************************/
int delete_objects(WIDES *theWides)
{
  deleteself_MSGFACTORY(theWides->that_mess_fact);
  deleteself_C_FSM(theWides->that_fsm_client);
  deleteself_C_GI(theWides->that_gi_client);
  deleteself_C_PI(theWides->that_pi_client);
  deleteself_C_BusinessObject(theWides->that_bo_client);
  deleteself_MSGFACTORY(theWides->that_mess_fact);
  return 1;
}
/******************************/
int main(int argc, char **argv)
{
  int seed = 0;
  if (argc > 2)
  { //main --seed value
    sscanf(argv[2], "%i", &seed);
  }
  srand(seed);
  WIDES theWides;

  int maxSteps = 25;
  int how;
  INMSG *an_inmsg;
  unsigned char *wire_out;
  size_t wire_out_len;
  char buffer4wire[MAXLINE];
  size_t nrbytesreceivedorsent;
  socklen_t sizeof_servaddr;
  sizeof_servaddr = sizeof(theWides.servaddr);

  if (init_udp(&theWides) != 1)
    return 0;
  init_objects(&theWides);

  // What inmessage to start the client?
  INIT_MESSAGES *possible_init_messages_to_client =
      get_possible_init_messages_C_FSM(theWides.that_fsm_client);
  // Pick a random one
  int which_start_message = rand() % possible_init_messages_to_client->the_size_of_inits;

  MESSAGETYPES messagetype_first_message_to_client =
      possible_init_messages_to_client->inits[which_start_message];

  // Allocate the initial in messsage and assign to the FSM (via the that_mess_fact)
  theWides.that_mess_fact->alloc_real_inmsg_MSGFACTORY(
      theWides.that_mess_fact, messagetype_first_message_to_client);

  while (maxSteps > 0)
  {
    how = theWides.that_fsm_client->take_event_C_FSM(theWides.that_fsm_client,
                                                     &theWides.state, theWides.that_bo_client);
    // We don't need the in message any more
    theWides.that_mess_fact->free_real_inmsg_MSGFACTORY(theWides.that_mess_fact);
    if (how == 0)
    {
      fprintf(stderr, "client_main() theMessageFactory->theOutMessage.msgout.result '%i' \n",
              theWides.that_mess_fact->theOutMessage.msgout.result);
      fprintf(stderr, "client_main() theMessageFactory->theOutMessage->errortext  %s \n",
              theWides.that_mess_fact->theOutMessage.errortext);
      free(theWides.that_pi_client->theMsgFactoryObj->theOutMessage.errortext);
      return 0;
    }
    if (theWides.that_fsm_client->isinfinalstate_C_FSM(theWides.that_fsm_client, theWides.state))
    {
      fprintf(stderr, "Client is now in finalstate\n");
      break;
    }
    // Pick out message generated by the FSM and store in the that_mess_fact.
    const char *outmsg = MESSAGES_LOOKUP[theWides.that_mess_fact->theOutMessage.msgout.thetype].msg_name;
    printf("Client >>>>>%*s%*s>>> Server\n", 10 + (int)strlen(outmsg) / 2, outmsg, 10 - (int)strlen(outmsg) / 2, "");
    // Convert the generated out message from computational for to wire form
    wire_out = theWides.that_pi_client->theMsgFactoryObj->comp2wire_MSGFACTORY(
        theWides.that_pi_client->theMsgFactoryObj, &wire_out_len);

    // Send message to server
    if (
        (nrbytesreceivedorsent =
            sendto(
                theWides.sockfd,
                (const char *)wire_out,
                strlen((char *)wire_out),
                0,
                (const struct sockaddr *)&theWides.servaddr,
                sizeof(theWides.servaddr))) == SOCKET_ERROR)
    {
#if defined(_WIN32)
      fprintf(stderr, "Client failed to sendto (error %d)\n", WSAGetLastError());
#else
      fprintf(stderr, "Client failed to sendto (error %d)\n", errno);
#endif
      return 0;
    }

    theWides.that_mess_fact->free_real_outmsg_MSGFACTORY(theWides.that_mess_fact);

    // ...and read an answer
    if (
        (nrbytesreceivedorsent =
            recvfrom(
                theWides.sockfd,
                (char *)buffer4wire,
                MAXLINE,
                0,
                (struct sockaddr *)&theWides.servaddr,
                &sizeof_servaddr)) == SOCKET_ERROR)
    {
#if defined(_WIN32)
      fprintf(stderr, "Client failed to recvfrom (error %d)\n", WSAGetLastError());
#else
      fprintf(stderr, "Client failed to recvfrom (error %d)\n", errno);
#endif
      return 0;
    }

    // Convert wire form of message to computational form
    an_inmsg = theWides.that_pi_client->theMsgFactoryObj->wire2comp_MSGFACTORY(
        theWides.that_pi_client->theMsgFactoryObj,
        (unsigned char *)buffer4wire,
        nrbytesreceivedorsent);
    if (an_inmsg == 0)
      return 0;
    const char *inmsg = MESSAGES_LOOKUP[theWides.that_mess_fact->theInMsg.thetype].msg_name;
    printf("Client <<<<<%*s%*s<<< Server\n",
           10 + (int)strlen(inmsg) / 2,
           inmsg, 10 - (int)strlen(inmsg) / 2, "");
    --maxSteps;
  }
  #if defined(_WIN32)
  closesocket(theWides.sockfd);
  #else
  close(theWides.sockfd);
  #endif
  delete_objects(&theWides);
  return 1;
}