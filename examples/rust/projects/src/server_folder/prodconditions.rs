#[derive(Debug, Clone)]
pub enum OutMsg_43 {
    Cnd_keyexchage_42_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_blablabla {
    Cnd_awaitconfirm_yes_1,
    Cnd_normal_hm_2,
    Cnd_normal_well_2,
}

#[derive(Debug, Clone)]
pub enum OutMsg_bye {
    Cnd_awaitconfirm_other_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_bye_bob {
    Cnd_normal_well_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_dontdisturb {
    Cnd_init_hello_alice_2,
}

#[derive(Debug, Clone)]
pub enum OutMsg_hello_bob {
    Cnd_init_hello_alice_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_or {
    Cnd_normal_hm_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_pong {
    Cnd_init_ping_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_questionwas {
    Cnd_keyexchage_24_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_rich {
    Cnd_startup_how_are_you_1,
}

#[derive(Debug, Clone)]
pub enum OutMsg_sorry {
    Cnd_init_ping_2,
}

#[derive(Debug, Clone)]
pub enum OutMsg_tired {
    Cnd_startup_how_are_you_2,
}

#[derive(Debug, Clone)]
pub enum OutMsg_what {
    Cnd_awaitconfirm_no_1,
}
