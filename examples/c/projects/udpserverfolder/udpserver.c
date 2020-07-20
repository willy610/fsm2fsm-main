// Server side. Alice
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

#include "S_FSM.h"
#include "SessionManager.h"

#define PORT 8081
#define MAXLINE 1024
/******************************/

struct wide_variabels
{
  int init_state;
  // the server object FSM
  S_FSM server_fsm;
  S_FSM *that_fsm_server;

  // the server object GI
  S_GI gi_server;
  S_GI *that_gi_server;

  // the server object PI
  S_PI pi_server;
  S_PI *that_pi_server;

  // messagefactory
  MSGFACTORY message_factory;
  MSGFACTORY *that_mess_fact;

  // udp
  int sockfd;
  struct sockaddr_in servaddr;
  struct sockaddr_in cliaddr;
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
    return 0;
  }
#endif
  if ((theWides->sockfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP)) == SOCKET_ERROR)
  {
#if defined(_WIN32)
    printf("socked() failed. Error Code : %d", WSAGetLastError());
#else
    perror("socket creation failed");
#endif
    return 0;
  }
  memset(&theWides->servaddr, 0, sizeof(theWides->servaddr));
  memset(&theWides->cliaddr, 0, sizeof(theWides->cliaddr));
  // Filling server information
  theWides->servaddr.sin_family = AF_INET;
  theWides->servaddr.sin_port = htons(PORT);
  theWides->servaddr.sin_addr.s_addr = htonl(INADDR_ANY);

  if (bind(theWides->sockfd, (const struct sockaddr *)&theWides->servaddr,
           sizeof(theWides->servaddr)) == SOCKET_ERROR)
  {
#if defined(_WIN32)
      fprintf(stderr, "Server bind failed (error %d)\n", WSAGetLastError());
#else
      fprintf(stderr, "Server bind failed (error %d)\n", errno);
#endif
    perror("bind failed");
    return 0;
  }
  return 1;
}

/******************************/
int init_objects(WIDES *theWides)
{
  // INITIALIZE THE INSTACES

  theWides->that_mess_fact = in_MSGFACTORY(&theWides->message_factory);

  theWides->that_fsm_server = in_S_FSM(&theWides->server_fsm);
  theWides->that_gi_server = in_S_GI(&theWides->gi_server);
  theWides->that_pi_server = in_S_PI(&theWides->pi_server);

  // chain the server objects
  theWides->that_fsm_server->the_gi = &theWides->gi_server;
  theWides->that_gi_server->theProdObj = &theWides->pi_server;

  theWides->that_fsm_server->theMsgFactoryObj = theWides->that_mess_fact;
  theWides->that_pi_server->theMsgFactoryObj = theWides->that_mess_fact;
  theWides->that_gi_server->theMsgFactoryObj = theWides->that_mess_fact;

  theWides->init_state = getinitialstate_S_FSM(theWides->that_fsm_server);
  return 1;
}
/******************************/
int delete_objects(WIDES *theWides)
{
  deleteself_MSGFACTORY(theWides->that_mess_fact);
  deleteself_S_FSM(theWides->that_fsm_server);
  deleteself_S_GI(theWides->that_gi_server);
  deleteself_S_PI(theWides->that_pi_server);

  deleteself_MSGFACTORY(theWides->that_mess_fact);
  return 1;
}
/******************************/
int main(int argc, char **argv)
{
  WIDES theWides;

  SessionManager sessionmanager;
  SessionManager *that_session_manager;
  that_session_manager = in_SessionManager(&sessionmanager);
  int session_id;
  S_BusinessObject *a_bo_server = (void *)0;
  int use_state;

  int seed = 0;
  int how;

  unsigned char wire_in[MAXLINE];
  size_t wire_in_len;
  unsigned char *wire_out;
  size_t wire_out_len;
  INMSG *an_inmsg;

  size_t how_sendto;
  socklen_t sizeof_cliaddr;
  sizeof_cliaddr = sizeof(theWides.cliaddr);
  int the_client_port;

  if (argc > 2)
  { //main --seed 1236 or other
    sscanf(argv[2], "%i", &seed);
  }
  srand(seed);

  if (init_udp(&theWides)==0)
    return 0;
  init_objects(&theWides);
 
  while (1)
  {
    if (
        (wire_in_len =
            recvfrom(theWides.sockfd,
                     (char *)wire_in, MAXLINE,
                     0,
                     (struct sockaddr *)&theWides.cliaddr,
                     &sizeof_cliaddr)) == SOCKET_ERROR)
    {
#if defined(_WIN32)
      fprintf(stderr, "Server failed to recvfrom (error %d)\n", WSAGetLastError());
#else
      fprintf(stderr, "Server failed to recvfrom (error %d)\n", errno);
#endif
      return 0;
    }

    the_client_port = theWides.cliaddr.sin_port; // kind of session id

    //////////////////////////
    // Session managment STARTS
    session_id =
        that_session_manager->SessionManager_find_id_by_port(
            that_session_manager,
            the_client_port);
    if (session_id == -1)
    { // NEW SESSION
      a_bo_server = nw_S_BusinessObject();
      use_state = theWides.init_state;
      session_id = that_session_manager->SessionManager_add_port(
          that_session_manager,
          the_client_port,
          a_bo_server,
          use_state);
      fprintf(stderr, "New session on (Port:%i)\n", the_client_port);
    }
    else
    { // OLD SESSION.
      // pick up STATE and BUSINESSOBJ, the_client_port
      a_bo_server = that_session_manager->SessionManager_assign_from_session_id(
          that_session_manager,
          session_id,
          &use_state,
          &the_client_port);
    }
    // Session managment ENDS
    //////////////////////////
    an_inmsg = theWides.that_mess_fact->wire2comp_MSGFACTORY(theWides.that_mess_fact,
                                                             wire_in, wire_in_len);
    if (an_inmsg == 0)
      return 0;
    how = theWides.that_fsm_server->take_event_S_FSM(
        theWides.that_fsm_server, &use_state, a_bo_server);
    if (how == 0)
    {
      fprintf(stderr, "ERROR: take_event_S_FSM()::2:how=%s\n",
              theWides.that_mess_fact->theOutMessage.errortext);
      free(theWides.that_mess_fact->theOutMessage.errortext);
      break;
    }
    // servicetime!!
#if defined(_WIN32)
    Sleep(1000);
#else
    sleep(1);
#endif
    wire_out = theWides.that_mess_fact->comp2wire_MSGFACTORY(theWides.that_mess_fact,
                                                             &wire_out_len);

    const char *inmsg =
        MESSAGES_LOOKUP[theWides.that_mess_fact->theInMsg.thetype].msg_name;
    const char *outmsg =
        MESSAGES_LOOKUP[theWides.that_mess_fact->theOutMessage.msgout.thetype].msg_name;
    printf("Client (Port:%i) >>>>>%*s%*s>>> Server\n",
           the_client_port, 10 + (int)strlen(inmsg) / 2, inmsg, 10 - (int)strlen(inmsg) / 2, "");
    printf("Client (Port:%i) <<<<<%*s%*s<<< Server\n",
           the_client_port, 10 + (int)strlen(outmsg) / 2, outmsg, 10 - (int)strlen(outmsg) / 2, "");

    free_real_outmsg_MSGFACTORY(theWides.that_mess_fact);
//    socklen_t len = sizeof(theWides.cliaddr);
    if (
        (how_sendto = sendto(theWides.sockfd,
                            (const char *)wire_out,
                            (size_t)wire_out_len,
                            0,
                            (const struct sockaddr *)&theWides.cliaddr,
                            sizeof_cliaddr)) == SOCKET_ERROR)
    {
#if defined(_WIN32)
      fprintf(stderr, "Server failed to sendto to client (%i). (error %d)\n", the_client_port, WSAGetLastError());
#else
      fprintf(stderr, "Server failed in sendto to client (%i). Errno=%i\n", the_client_port, errno);
#endif
      return 0;
    }
    //////////////////////////////////////
    // SESSION ONE TRANSITION DONE
    // for sure
    if (isinfinalstate_S_FSM(theWides.that_fsm_server, use_state))
    {
      fprintf(stderr, "Session (Port:%i) went into finalstate\n", the_client_port);
      that_session_manager->SessionManager_free_id(
          that_session_manager, session_id);
      deleteself_S_BusinessObject(a_bo_server);
    }
    else
    { // NOT FINAL. SAVE
      that_session_manager->SessionManager_save_session(
          that_session_manager, session_id, use_state, a_bo_server);
    }
    // forget
    a_bo_server = (void *)0;
    use_state = -1;
    // no smart cache here. always find 'SessionManager_find_id_by_port'
    session_id = 0;
  }
  #if defined(_WIN32)
  closesocket(theWides.sockfd);
  #else
  close(theWides.sockfd);
  #endif
  delete_objects(&theWides);
  return 1;
}