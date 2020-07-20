#ifndef SESSIONMANAGER_H
#define SESSIONMANAGER_H

#ifdef __cplusplus
extern "C"
{
#endif

#include "S_BusinessObject.h"

#define MAX_NR_SESSIONS 10

  struct sessions
  {
    int in_use;
    int client_port;
    int state;
    S_BusinessObject *business_obj;
  };
  typedef struct SessionManager SessionManager;

  struct SessionManager
  {
    int malloceted;
    // User attributes
    struct sessions sessions[MAX_NR_SESSIONS];
    void (*SessionManager_deleteself)(SessionManager **self);
    // User methods
    int (*SessionManager_find_id_by_port)(SessionManager *self, int port);
    int (*SessionManager_add_port)(SessionManager *self, int port, void *business_obj, int state);
    S_BusinessObject *(*SessionManager_assign_from_session_id)(
        SessionManager *self, int session_id,
        int *state,
        int *the_client_port);
    void *(*SessionManager_save_session)(SessionManager *self, int session_id, int state, void *business_obj);
    void *(*SessionManager_free_id)(SessionManager *self, int session_id);
  };
  // Class Constructor methods
  // For Instances as static or stack
  SessionManager *in_SessionManager(SessionManager *self);
  // For Instances in dynamic memory
  SessionManager *nw_SessionManager();

  // Object or Instance methods

  void SessionManager_deleteself(SessionManager **self);
/*
  // User methods
  static int SessionManager_find_id_by_port(SessionManager *self, int port);
  static int SessionManager_add_port(SessionManager *self, int port, void *business_obj, int state);
  static S_BusinessObject *SessionManager_assign_from_session_id(
      SessionManager *self, int session_id,
      int *state, int *the_client_port);
  static void *SessionManager_save_session(SessionManager *self, int session_id, int state, void *business_obj);
  static void *SessionManager_free_id(SessionManager *self, int session_id);
*/
#ifdef __cplusplus
}
#endif

#endif /* SESSIONMANAGER_H */
