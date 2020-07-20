use super::super::shared_folder::realmessages::*;
#[derive(Debug, Clone)]
pub enum Inmessagetypes {
    Msg_24,
    Msg_42,
    Msg_hello_alice,
    Msg_hm,
    Msg_how_are_you,
    Msg_no,
    Msg_other,
    Msg_ping,
    Msg_well,
    Msg_yes,
}
#[derive(Debug, Clone)]
pub enum Outmessagetypes {
    Msg_43,
    Msg_blablabla,
    Msg_bye,
    Msg_bye_bob,
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
pub enum EnumRealInMessages {
    Msg_24(RealMsg_24Type),
    Msg_42(RealMsg_42Type),
    Msg_hello_alice(RealMsg_hello_aliceType),
    Msg_hm(RealMsg_hmType),
    Msg_how_are_you(RealMsg_how_are_youType),
    Msg_no(RealMsg_noType),
    Msg_other(RealMsg_otherType),
    Msg_ping(RealMsg_pingType),
    Msg_well(RealMsg_wellType),
    Msg_yes(RealMsg_yesType),
}
pub type EnumRealInMessagesType = EnumRealInMessages;
#[derive(Debug, Clone)]
pub enum EnumRealOutMessages {
    Msg_43(RealMsg_43Type),
    Msg_blablabla(RealMsg_blablablaType),
    Msg_bye(RealMsg_byeType),
    Msg_bye_bob(RealMsg_bye_bobType),
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
pub type EnumRealOutMessagesType = EnumRealOutMessages;
#[derive(Debug, Clone)]
pub enum ParamTo_43 {
    IN_Cnd_keyexchage_42_1(RealMsg_42Type),
}
#[derive(Debug, Clone)]
pub enum ParamTo_blablabla {
    IN_Cnd_awaitconfirm_yes_1(RealMsg_yesType),
    IN_Cnd_normal_hm_2(RealMsg_hmType),
    IN_Cnd_normal_well_2(RealMsg_wellType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_bye {
    IN_Cnd_awaitconfirm_other_1(RealMsg_otherType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_bye_bob {
    IN_Cnd_normal_well_1(RealMsg_wellType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_dontdisturb {
    IN_Cnd_init_hello_alice_2(RealMsg_hello_aliceType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_hello_bob {
    IN_Cnd_init_hello_alice_1(RealMsg_hello_aliceType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_or {
    IN_Cnd_normal_hm_1(RealMsg_hmType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_pong {
    IN_Cnd_init_ping_1(RealMsg_pingType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_questionwas {
    IN_Cnd_keyexchage_24_1(RealMsg_24Type),
}
#[derive(Debug, Clone)]
pub enum ParamTo_rich {
    IN_Cnd_startup_how_are_you_1(RealMsg_how_are_youType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_sorry {
    IN_Cnd_init_ping_2(RealMsg_pingType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_tired {
    IN_Cnd_startup_how_are_you_2(RealMsg_how_are_youType),
}
#[derive(Debug, Clone)]
pub enum ParamTo_what {
    IN_Cnd_awaitconfirm_no_1(RealMsg_noType),
}
