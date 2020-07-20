#include <stdlib.h>
#include <stdio.h>
#include "SessionManager.h"

//static int other_SessionManager_method1(SessionManager *self, int p1, int p2);
/*++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
SessionManager *in_SessionManager(SessionManager *self)
{
  self->malloceted = 0;
  // init user attributes
  for (int i = 0; i < MAX_NR_SESSIONS; ++i)
    self->sessions[i].in_use = 0;

  self->SessionManager_deleteself = &SessionManager_deleteself;
  self->SessionManager_find_id_by_port = &SessionManager_find_id_by_port;
  self->SessionManager_add_port = &SessionManager_add_port;
  self->SessionManager_assign_from_session_id = &SessionManager_assign_from_session_id;
  self->SessionManager_save_session=&SessionManager_save_session;
  self->SessionManager_free_id = &SessionManager_free_id;

  return self;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
SessionManager *nw_SessionManager()
{
  SessionManager *self = malloc(sizeof(SessionManager));
  self = in_SessionManager(self);
  self->malloceted = 1; // Is in dynamic memory
  return self;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
void SessionManager_deleteself(SessionManager **self)
{
  if ((*self)->malloceted)
  {
    free(*self);
  }
  *self = 0;
  return;
};
/*++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
static int SessionManager_find_id_by_port(SessionManager *self, int port)
{
  for (int session_id = 0; session_id < MAX_NR_SESSIONS; ++session_id)
  {
    if (self->sessions[session_id].in_use == 1 && self->sessions[session_id].client_port == port)
      return session_id;
  }
  return -1; // not found
};
/*++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
static int SessionManager_add_port(SessionManager *self,
                                   int port, void *business_obj, int state)
{
  for (int session_id = 0; session_id < MAX_NR_SESSIONS; ++session_id)
  {
    if (self->sessions[session_id].in_use == 0)
    {
      self->sessions[session_id].in_use = 1;
      self->sessions[session_id].client_port = port;
      self->sessions[session_id].business_obj = business_obj;
      self->sessions[session_id].state = state;
      return session_id;
    }
  }
  return -1; // full
};
/*++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
static S_BusinessObject *SessionManager_assign_from_session_id(
    SessionManager *self, int session_id,
    int *state, int *client_port)
{
  *state = self->sessions[session_id].state;
  *client_port = self->sessions[session_id].client_port;
  return self->sessions[session_id].business_obj;
  ;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
static void *SessionManager_save_session(
    SessionManager *self, int session_id,
    int state, void *business_obj)
{
  self->sessions[session_id].state = state;;
  self->sessions[session_id].business_obj = business_obj;
  return (void *)0;
}
/*++++++++++++++++++++++++++++++++++++++++++++++++++++++*/
static void *SessionManager_free_id(SessionManager *self, int session_id)
{
  self->sessions[session_id].in_use = 0;
  return (void *)0;
}
