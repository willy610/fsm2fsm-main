use super::super::shared_folder::realmessages::*;
#[derive(Debug, Clone)]
pub enum Inmessagetypes {
    Msg_43,
    Msg_blablabla,
    Msg_bye,
    Msg_bye_bob,
    Msg_callin,
    Msg_dontdisturb,
    Msg_hello_bob,
    Msg_or,
    Msg_pong,
    Msg_questionwas,
    Msg_rich,
    Msg_sorry,
    Msg_tired,
    Msg_what,
}
#[derive(Debug, Clone)]
pub enum Outmessagetypes {
    Msg_24,
    Msg_42,
    Msg_hello_alice,
    Msg_hm,
    Msg_how_are_you,
    Msg_no,
    Msg_nooutput,
    Msg_other,
    Msg_ping,
    Msg_well,
    Msg_yes,
}
#[derive(Debug, Clone)]
pub enum EnumRealInMessages {
    Msg_43(RealMsg_43Type),
    Msg_blablabla(RealMsg_blablablaType),
    Msg_bye(RealMsg_byeType),
    Msg_bye_bob(RealMsg_bye_bobType),
    Msg_callin(RealMsg_callinType),
    Msg_dontdisturb(RealMsg_dontdisturbType),
    Msg_hello_bob(RealMsg_hello_bobType),
    Msg_or(RealMsg_orType),
    Msg_pong(RealMsg_pongType),
    Msg_questionwas(RealMsg_questionwasType),
    Msg_rich(RealMsg_richType),
    Msg_sorry(RealMsg_sorryType),
    Msg_tired(RealMsg_tiredType),
    Msg_what(RealMsg_whatType),
}
pub type EnumRealInMessagesType = EnumRealInMessages;
#[derive(Debug, Clone)]
pub enum EnumRealOutMessages {
    Msg_24(RealMsg_24Type),
    Msg_42(RealMsg_42Type),
    Msg_hello_alice(RealMsg_hello_aliceType),
    Msg_hm(RealMsg_hmType),
    Msg_how_are_you(RealMsg_how_are_youType),
    Msg_no(RealMsg_noType),
    Msg_nooutput(RealMsg_nooutputType),
    Msg_other(RealMsg_otherType),
    Msg_ping(RealMsg_pingType),
    Msg_well(RealMsg_wellType),
    Msg_yes(RealMsg_yesType),
}
pub type EnumRealOutMessagesType = EnumRealOutMessages;
#[derive(Debug, Clone)]
pub enum ParamTo_24 {
    IN_Cnd_keyexchage_24_questionwas_1(RealMsg_questionwasType),
    IN_Cnd_startup_how_are_you_rich_1(RealMsg_richType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_42 {
    IN_Cnd_keyexchage_24_questionwas_2(RealMsg_questionwasType),
    IN_Cnd_startup_how_are_you_rich_2(RealMsg_richType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_hello_alice {
    IN_Cnd_init_callin_2(RealMsg_callinType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_hm {
    IN_Cnd_awaitconfirm_yes_blablabla_1(RealMsg_blablablaType),
    IN_Cnd_keyexchage_42_43_1(RealMsg_43Type),
    IN_Cnd_normal_hm_blablabla_2(RealMsg_blablablaType),
    IN_Cnd_normal_well_blablabla_2(RealMsg_blablablaType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_how_are_you {
    IN_Cnd_init_hello_alice_hello_bob_1(RealMsg_hello_bobType),
    IN_Cnd_init_ping_pong_1(RealMsg_pongType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_no {
    IN_Cnd_awaitconfirm_no_what_1(RealMsg_whatType),
    IN_Cnd_normal_hm_or_3(RealMsg_orType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_nooutput {
    IN_Cnd_awaitconfirm_other_bye_1(RealMsg_byeType),
    IN_Cnd_init_hello_alice_dontdisturb_1(RealMsg_dontdisturbType),
    IN_Cnd_init_ping_sorry_1(RealMsg_sorryType),
    IN_Cnd_normal_well_bye_bob_1(RealMsg_bye_bobType),
    IN_Cnd_startup_how_are_you_tired_1(RealMsg_tiredType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_other {
    IN_Cnd_awaitconfirm_no_what_2(RealMsg_whatType),
    IN_Cnd_normal_hm_or_2(RealMsg_orType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_ping {
    IN_Cnd_init_callin_1(RealMsg_callinType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_well {
    IN_Cnd_awaitconfirm_yes_blablabla_2(RealMsg_blablablaType),
    IN_Cnd_keyexchage_42_43_2(RealMsg_43Type),
    IN_Cnd_normal_hm_blablabla_1(RealMsg_blablablaType),
    IN_Cnd_normal_well_blablabla_1(RealMsg_blablablaType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_yes {
    IN_Cnd_awaitconfirm_no_what_3(RealMsg_whatType),
    IN_Cnd_normal_hm_or_1(RealMsg_orType),
}
