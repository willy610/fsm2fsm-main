#include "MsgFactory.h"
#include "S_GuardConditions.h"
#include "S_BusinessObject.h"

#ifndef S_PI_H
#define S_PI_H

typedef enum { FROM_Cnd_keyexchage_42_1 
}TOMSG_43 ;
typedef enum { FROM_Cnd_awaitconfirm_yes_1,
FROM_Cnd_normal_hm_2,
FROM_Cnd_normal_well_2 
}TOMSG_blablabla ;
typedef enum { FROM_Cnd_awaitconfirm_other_1 
}TOMSG_bye ;
typedef enum { FROM_Cnd_normal_well_1 
}TOMSG_bye_bob ;
typedef enum { FROM_Cnd_init_hello_alice_2 
}TOMSG_dontdisturb ;
typedef enum { FROM_Cnd_init_hello_alice_1 
}TOMSG_hello_bob ;
typedef enum { FROM_Cnd_normal_hm_1 
}TOMSG_or ;
typedef enum { FROM_Cnd_init_ping_1 
}TOMSG_pong ;
typedef enum { FROM_Cnd_keyexchage_24_1 
}TOMSG_questionwas ;
typedef enum { FROM_Cnd_startup_how_are_you_1 
}TOMSG_rich ;
typedef enum { FROM_Cnd_init_ping_2 
}TOMSG_sorry ;
typedef enum { FROM_Cnd_startup_how_are_you_2 
}TOMSG_tired ;
typedef enum { FROM_Cnd_awaitconfirm_no_1 
}TOMSG_what ;

typedef struct S_PI S_PI;
void deleteself_S_PI(S_PI * self);
typedef struct { 
int (*S_prod_43)(S_PI * self,
TOMSG_43 given_cond, S_BusinessObject *business_object);

int (*S_prod_blablabla)(S_PI * self,
TOMSG_blablabla given_cond, S_BusinessObject *business_object);

int (*S_prod_bye)(S_PI * self,
TOMSG_bye given_cond, S_BusinessObject *business_object);

int (*S_prod_bye_bob)(S_PI * self,
TOMSG_bye_bob given_cond, S_BusinessObject *business_object);

int (*S_prod_dontdisturb)(S_PI * self,
TOMSG_dontdisturb given_cond, S_BusinessObject *business_object);

int (*S_prod_hello_bob)(S_PI * self,
TOMSG_hello_bob given_cond, S_BusinessObject *business_object);

int (*S_prod_or)(S_PI * self,
TOMSG_or given_cond, S_BusinessObject *business_object);

int (*S_prod_pong)(S_PI * self,
TOMSG_pong given_cond, S_BusinessObject *business_object);

int (*S_prod_questionwas)(S_PI * self,
TOMSG_questionwas given_cond, S_BusinessObject *business_object);

int (*S_prod_rich)(S_PI * self,
TOMSG_rich given_cond, S_BusinessObject *business_object);

int (*S_prod_sorry)(S_PI * self,
TOMSG_sorry given_cond, S_BusinessObject *business_object);

int (*S_prod_tired)(S_PI * self,
TOMSG_tired given_cond, S_BusinessObject *business_object);

int (*S_prod_what)(S_PI * self,
TOMSG_what given_cond, S_BusinessObject *business_object); }S_PI_VTABLE;
struct S_PI {
int mallocated;
int something_in_PI;
void (*deleteself_S_PI)(S_PI * self);

MSGFACTORY * theMsgFactoryObj;
S_PI_VTABLE *vtable;
};
S_PI * in_S_PI(S_PI * self);
//S_PI * nw_S_PI(S_PI * self);
S_PI * nw_S_PI(void);
// MOVED TO BODY 
#endif /* S_PI_H */
