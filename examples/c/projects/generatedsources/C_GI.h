#ifndef C_GI_H
#define C_GI_H
   
#include "C_PI.h"
#include "C_BusinessObject.h"

typedef struct C_GI C_GI;

void deleteself_C_GI(C_GI * self);

typedef struct { int (*C_awaitconfirm_no_what)(C_GI* self, Cnds_awaitconfirm_no_what *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_awaitconfirm_other_bye)(C_GI* self, Cnds_awaitconfirm_other_bye *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_awaitconfirm_yes_blablabla)(C_GI* self, Cnds_awaitconfirm_yes_blablabla *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_init_callin)(C_GI* self, Cnds_init_callin *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_init_hello_alice_dontdisturb)(C_GI* self, Cnds_init_hello_alice_dontdisturb *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_init_hello_alice_hello_bob)(C_GI* self, Cnds_init_hello_alice_hello_bob *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_init_ping_pong)(C_GI* self, Cnds_init_ping_pong *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_init_ping_sorry)(C_GI* self, Cnds_init_ping_sorry *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_keyexchage_24_questionwas)(C_GI* self, Cnds_keyexchage_24_questionwas *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_keyexchage_42_43)(C_GI* self, Cnds_keyexchage_42_43 *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_normal_hm_blablabla)(C_GI* self, Cnds_normal_hm_blablabla *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_normal_hm_or)(C_GI* self, Cnds_normal_hm_or *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_normal_well_blablabla)(C_GI* self, Cnds_normal_well_blablabla *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_normal_well_bye_bob)(C_GI* self, Cnds_normal_well_bye_bob *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_startup_how_are_you_rich)(C_GI* self, Cnds_startup_how_are_you_rich *ret_choosen_cond, C_BusinessObject *business_object);
    int (*C_startup_how_are_you_tired)(C_GI* self, Cnds_startup_how_are_you_tired *ret_choosen_cond, C_BusinessObject *business_object);
     }C_GI_VTABLE;

struct C_GI {
  int mallocated;
  int something_in_GI;
  void (*deleteself_C_GI)(C_GI * self);

  MSGFACTORY * theMsgFactoryObj;

  C_GI_VTABLE *vtable;
  C_PI *theProdObj;
};
C_GI * in_C_GI(C_GI * self);
C_GI * nw_C_GI(C_GI * self);

// MOVED TO BODY

#endif /* C_GI_H */
