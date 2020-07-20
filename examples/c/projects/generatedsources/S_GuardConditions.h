#ifndef S_GUARDCONDITIONS_H
#define S_GUARDCONDITIONS_H

typedef enum  {
  Cnd_awaitconfirm_no_1 
}
Cnds_awaitconfirm_no;

typedef enum  {
  Cnd_awaitconfirm_other_1 
}
Cnds_awaitconfirm_other;

typedef enum  {
  Cnd_awaitconfirm_yes_1 
}
Cnds_awaitconfirm_yes;

typedef enum  {
  Cnd_init_hello_alice_1,
	Cnd_init_hello_alice_2 
}
Cnds_init_hello_alice;

typedef enum  {
  Cnd_init_ping_1,
	Cnd_init_ping_2 
}
Cnds_init_ping;

typedef enum  {
  Cnd_keyexchage_24_1 
}
Cnds_keyexchage_24;

typedef enum  {
  Cnd_keyexchage_42_1 
}
Cnds_keyexchage_42;

typedef enum  {
  Cnd_normal_hm_1,
	Cnd_normal_hm_2 
}
Cnds_normal_hm;

typedef enum  {
  Cnd_normal_well_1,
	Cnd_normal_well_2 
}
Cnds_normal_well;

typedef enum  {
  Cnd_startup_how_are_you_1,
	Cnd_startup_how_are_you_2 
}
Cnds_startup_how_are_you;


#endif /* S_GUARDCONDITIONS_H */
