#ifndef S_GI_H
#define S_GI_H
   
#include "S_PI.h"
#include "S_BusinessObject.h"

typedef struct S_GI S_GI;

void deleteself_S_GI(S_GI * self);

typedef struct { int (*S_awaitconfirm_no)(S_GI* self, Cnds_awaitconfirm_no *ret_choosen_cond, S_BusinessObject *business_object);
    int (*S_awaitconfirm_other)(S_GI* self, Cnds_awaitconfirm_other *ret_choosen_cond, S_BusinessObject *business_object);
    int (*S_awaitconfirm_yes)(S_GI* self, Cnds_awaitconfirm_yes *ret_choosen_cond, S_BusinessObject *business_object);
    int (*S_init_hello_alice)(S_GI* self, Cnds_init_hello_alice *ret_choosen_cond, S_BusinessObject *business_object);
    int (*S_init_ping)(S_GI* self, Cnds_init_ping *ret_choosen_cond, S_BusinessObject *business_object);
    int (*S_keyexchage_24)(S_GI* self, Cnds_keyexchage_24 *ret_choosen_cond, S_BusinessObject *business_object);
    int (*S_keyexchage_42)(S_GI* self, Cnds_keyexchage_42 *ret_choosen_cond, S_BusinessObject *business_object);
    int (*S_normal_hm)(S_GI* self, Cnds_normal_hm *ret_choosen_cond, S_BusinessObject *business_object);
    int (*S_normal_well)(S_GI* self, Cnds_normal_well *ret_choosen_cond, S_BusinessObject *business_object);
    int (*S_startup_how_are_you)(S_GI* self, Cnds_startup_how_are_you *ret_choosen_cond, S_BusinessObject *business_object);
     }S_GI_VTABLE;

struct S_GI {
  int mallocated;
  int something_in_GI;
  void (*deleteself_S_GI)(S_GI * self);

  MSGFACTORY * theMsgFactoryObj;

  S_GI_VTABLE *vtable;
  S_PI *theProdObj;
};
S_GI * in_S_GI(S_GI * self);
S_GI * nw_S_GI(S_GI * self);

// MOVED TO BODY

#endif /* S_GI_H */
