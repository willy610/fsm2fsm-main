#include "MsgFactory.h"
#include "C_GuardConditions.h"
#include "C_BusinessObject.h"

#ifndef C_PI_H
#define C_PI_H

typedef enum { FROM_Cnd_keyexchage_24_questionwas_2,
FROM_Cnd_startup_how_are_you_rich_2 
}TOMSG_24 ;
typedef enum { FROM_Cnd_keyexchage_24_questionwas_1,
FROM_Cnd_startup_how_are_you_rich_1 
}TOMSG_42 ;
typedef enum { FROM_Cnd_init_callin_1 
}TOMSG_hello_alice ;
typedef enum { FROM_Cnd_awaitconfirm_yes_blablabla_2,
FROM_Cnd_keyexchage_42_43_1,
FROM_Cnd_normal_hm_blablabla_1,
FROM_Cnd_normal_well_blablabla_1 
}TOMSG_hm ;
typedef enum { FROM_Cnd_init_hello_alice_hello_bob_1,
FROM_Cnd_init_ping_pong_1 
}TOMSG_how_are_you ;
typedef enum { FROM_Cnd_awaitconfirm_no_what_2,
FROM_Cnd_normal_hm_or_3 
}TOMSG_no ;
typedef enum { FROM_Cnd_awaitconfirm_other_bye_1,
FROM_Cnd_init_hello_alice_dontdisturb_1,
FROM_Cnd_init_ping_sorry_1,
FROM_Cnd_normal_well_bye_bob_1,
FROM_Cnd_startup_how_are_you_tired_1 
}TOMSG_nooutput ;
typedef enum { FROM_Cnd_awaitconfirm_no_what_3,
FROM_Cnd_normal_hm_or_2 
}TOMSG_other ;
typedef enum { FROM_Cnd_init_callin_2 
}TOMSG_ping ;
typedef enum { FROM_Cnd_awaitconfirm_yes_blablabla_1,
FROM_Cnd_keyexchage_42_43_2,
FROM_Cnd_normal_hm_blablabla_2,
FROM_Cnd_normal_well_blablabla_2 
}TOMSG_well ;
typedef enum { FROM_Cnd_awaitconfirm_no_what_1,
FROM_Cnd_normal_hm_or_1 
}TOMSG_yes ;

typedef struct C_PI C_PI;
void deleteself_C_PI(C_PI * self);
typedef struct { 
int (*C_prod_24)(C_PI * self,
TOMSG_24 given_cond, C_BusinessObject *business_object);

int (*C_prod_42)(C_PI * self,
TOMSG_42 given_cond, C_BusinessObject *business_object);

int (*C_prod_hello_alice)(C_PI * self,
TOMSG_hello_alice given_cond, C_BusinessObject *business_object);

int (*C_prod_hm)(C_PI * self,
TOMSG_hm given_cond, C_BusinessObject *business_object);

int (*C_prod_how_are_you)(C_PI * self,
TOMSG_how_are_you given_cond, C_BusinessObject *business_object);

int (*C_prod_no)(C_PI * self,
TOMSG_no given_cond, C_BusinessObject *business_object);

int (*C_prod_nooutput)(C_PI * self,
TOMSG_nooutput given_cond, C_BusinessObject *business_object);

int (*C_prod_other)(C_PI * self,
TOMSG_other given_cond, C_BusinessObject *business_object);

int (*C_prod_ping)(C_PI * self,
TOMSG_ping given_cond, C_BusinessObject *business_object);

int (*C_prod_well)(C_PI * self,
TOMSG_well given_cond, C_BusinessObject *business_object);

int (*C_prod_yes)(C_PI * self,
TOMSG_yes given_cond, C_BusinessObject *business_object); }C_PI_VTABLE;
struct C_PI {
int mallocated;
int something_in_PI;
void (*deleteself_C_PI)(C_PI * self);

MSGFACTORY * theMsgFactoryObj;
C_PI_VTABLE *vtable;
};
C_PI * in_C_PI(C_PI * self);
//C_PI * nw_C_PI(C_PI * self);
C_PI * nw_C_PI(void);
// MOVED TO BODY 
#endif /* C_PI_H */
