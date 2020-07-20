#[derive(Debug, Clone)]
pub enum OutMsg_24 {
    Cnd_keyexchage_24_questionwas_1,
    Cnd_startup_how_are_you_rich_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_42 {
    Cnd_keyexchage_24_questionwas_2,
    Cnd_startup_how_are_you_rich_2,
}

#[derive(Debug, Clone)]
pub enum OutMsg_hello_alice {
    Cnd_init_callin_2,
}

#[derive(Debug, Clone)]
pub enum OutMsg_hm {
    Cnd_awaitconfirm_yes_blablabla_1,
    Cnd_keyexchage_42_43_1,
    Cnd_normal_hm_blablabla_2,
    Cnd_normal_well_blablabla_2,
}

#[derive(Debug, Clone)]
pub enum OutMsg_how_are_you {
    Cnd_init_hello_alice_hello_bob_1,
    Cnd_init_ping_pong_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_no {
    Cnd_awaitconfirm_no_what_1,
    Cnd_normal_hm_or_3,
}

#[derive(Debug, Clone)]
pub enum OutMsg_nooutput {
    Cnd_awaitconfirm_other_bye_1,
    Cnd_init_hello_alice_dontdisturb_1,
    Cnd_init_ping_sorry_1,
    Cnd_normal_well_bye_bob_1,
    Cnd_startup_how_are_you_tired_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_other {
    Cnd_awaitconfirm_no_what_2,
    Cnd_normal_hm_or_2,
}

#[derive(Debug, Clone)]
pub enum OutMsg_ping {
    Cnd_init_callin_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_well {
    Cnd_awaitconfirm_yes_blablabla_2,
    Cnd_keyexchage_42_43_2,
    Cnd_normal_hm_blablabla_1,
    Cnd_normal_well_blablabla_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_yes {
    Cnd_awaitconfirm_no_what_3,
    Cnd_normal_hm_or_1,
}
