use super::business_class::BusinessObject;
use super::messagesets::*;
use super::msgfactoryclass::*;
use super::prodconditions::*;

#[derive(Debug)]
pub struct Prodmsgclass {
    //    pub business_object: BusinessObject,
    pub the_msgfactory: Msgfactoryclass,
}
impl Prodmsgclass {
    pub fn new(
        //business_object:BusinessObject,
        the_msgfactory: Msgfactoryclass,
    ) -> Prodmsgclass {
        Prodmsgclass {
            the_msgfactory: the_msgfactory,
            //            business_object : business_object
        }
    }
    pub fn get_whatever(&mut self) -> String {
        return "".to_string();
    }
    /*--------------  prod_24  -------------------------------*/
    pub fn prod_24(
        &mut self,
        _acond: &OutMsg_24,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("24");
        if let EnumRealOutMessagesType::Msg_24(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_24::Cnd_keyexchage_24_questionwas_1,
                    EnumRealInMessagesType::Msg_questionwas(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_24(the_real_outmsg));
                }
                (
                    OutMsg_24::Cnd_startup_how_are_you_rich_1,
                    EnumRealInMessagesType::Msg_rich(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_24(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg '24' in prod_24".to_string());
        }
    }

    /*--------------  prod_42  -------------------------------*/
    pub fn prod_42(
        &mut self,
        _acond: &OutMsg_42,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("42");
        if let EnumRealOutMessagesType::Msg_42(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_42::Cnd_keyexchage_24_questionwas_2,
                    EnumRealInMessagesType::Msg_questionwas(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_42(the_real_outmsg));
                }
                (
                    OutMsg_42::Cnd_startup_how_are_you_rich_2,
                    EnumRealInMessagesType::Msg_rich(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_42(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg '42' in prod_42".to_string());
        }
    }

    /*--------------  prod_hello_alice  -------------------------------*/
    pub fn prod_hello_alice(
        &mut self,
        _acond: &OutMsg_hello_alice,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("hello_alice");
        if let EnumRealOutMessagesType::Msg_hello_alice(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_hello_alice::Cnd_init_callin_2,
                    EnumRealInMessagesType::Msg_callin(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_hello_alice(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'hello_alice' in prod_hello_alice".to_string());
        }
    }

    /*--------------  prod_hm  -------------------------------*/
    pub fn prod_hm(
        &mut self,
        _acond: &OutMsg_hm,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("hm");
        if let EnumRealOutMessagesType::Msg_hm(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_hm::Cnd_awaitconfirm_yes_blablabla_1,
                    EnumRealInMessagesType::Msg_blablabla(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_hm(the_real_outmsg));
                }
                (
                    OutMsg_hm::Cnd_keyexchage_42_43_1,
                    EnumRealInMessagesType::Msg_43(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_hm(the_real_outmsg));
                }
                (
                    OutMsg_hm::Cnd_normal_hm_blablabla_2,
                    EnumRealInMessagesType::Msg_blablabla(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_hm(the_real_outmsg));
                }
                (
                    OutMsg_hm::Cnd_normal_well_blablabla_2,
                    EnumRealInMessagesType::Msg_blablabla(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_hm(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'hm' in prod_hm".to_string());
        }
    }

    /*--------------  prod_how_are_you  -------------------------------*/
    pub fn prod_how_are_you(
        &mut self,
        _acond: &OutMsg_how_are_you,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("how_are_you");
        if let EnumRealOutMessagesType::Msg_how_are_you(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_how_are_you::Cnd_init_hello_alice_hello_bob_1,
                    EnumRealInMessagesType::Msg_hello_bob(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_how_are_you(the_real_outmsg));
                }
                (
                    OutMsg_how_are_you::Cnd_init_ping_pong_1,
                    EnumRealInMessagesType::Msg_pong(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_how_are_you(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'how_are_you' in prod_how_are_you".to_string());
        }
    }

    /*--------------  prod_no  -------------------------------*/
    pub fn prod_no(
        &mut self,
        _acond: &OutMsg_no,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("no");
        if let EnumRealOutMessagesType::Msg_no(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_no::Cnd_awaitconfirm_no_what_1,
                    EnumRealInMessagesType::Msg_what(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_no(the_real_outmsg));
                }
                (OutMsg_no::Cnd_normal_hm_or_3, EnumRealInMessagesType::Msg_or(the_real_inmsg)) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_no(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'no' in prod_no".to_string());
        }
    }

    /*--------------  prod_nooutput  -------------------------------*/
    pub fn prod_nooutput(
        &mut self,
        _acond: &OutMsg_nooutput,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("nooutput");
        if let EnumRealOutMessagesType::Msg_nooutput(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_nooutput::Cnd_awaitconfirm_other_bye_1,
                    EnumRealInMessagesType::Msg_bye(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_nooutput(the_real_outmsg));
                }
                (
                    OutMsg_nooutput::Cnd_init_hello_alice_dontdisturb_1,
                    EnumRealInMessagesType::Msg_dontdisturb(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_nooutput(the_real_outmsg));
                }
                (
                    OutMsg_nooutput::Cnd_init_ping_sorry_1,
                    EnumRealInMessagesType::Msg_sorry(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_nooutput(the_real_outmsg));
                }
                (
                    OutMsg_nooutput::Cnd_normal_well_bye_bob_1,
                    EnumRealInMessagesType::Msg_bye_bob(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_nooutput(the_real_outmsg));
                }
                (
                    OutMsg_nooutput::Cnd_startup_how_are_you_tired_1,
                    EnumRealInMessagesType::Msg_tired(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_nooutput(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'nooutput' in prod_nooutput".to_string());
        }
    }

    /*--------------  prod_other  -------------------------------*/
    pub fn prod_other(
        &mut self,
        _acond: &OutMsg_other,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("other");
        if let EnumRealOutMessagesType::Msg_other(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_other::Cnd_awaitconfirm_no_what_2,
                    EnumRealInMessagesType::Msg_what(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_other(the_real_outmsg));
                }
                (
                    OutMsg_other::Cnd_normal_hm_or_2,
                    EnumRealInMessagesType::Msg_or(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_other(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'other' in prod_other".to_string());
        }
    }

    /*--------------  prod_ping  -------------------------------*/
    pub fn prod_ping(
        &mut self,
        _acond: &OutMsg_ping,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("ping");
        if let EnumRealOutMessagesType::Msg_ping(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_ping::Cnd_init_callin_1,
                    EnumRealInMessagesType::Msg_callin(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_ping(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'ping' in prod_ping".to_string());
        }
    }

    /*--------------  prod_well  -------------------------------*/
    pub fn prod_well(
        &mut self,
        _acond: &OutMsg_well,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("well");
        if let EnumRealOutMessagesType::Msg_well(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_well::Cnd_awaitconfirm_yes_blablabla_2,
                    EnumRealInMessagesType::Msg_blablabla(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_well(the_real_outmsg));
                }
                (
                    OutMsg_well::Cnd_keyexchage_42_43_2,
                    EnumRealInMessagesType::Msg_43(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_well(the_real_outmsg));
                }
                (
                    OutMsg_well::Cnd_normal_hm_blablabla_1,
                    EnumRealInMessagesType::Msg_blablabla(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_well(the_real_outmsg));
                }
                (
                    OutMsg_well::Cnd_normal_well_blablabla_1,
                    EnumRealInMessagesType::Msg_blablabla(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_well(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'well' in prod_well".to_string());
        }
    }

    /*--------------  prod_yes  -------------------------------*/
    pub fn prod_yes(
        &mut self,
        _acond: &OutMsg_yes,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("yes");
        if let EnumRealOutMessagesType::Msg_yes(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_yes::Cnd_awaitconfirm_no_what_3,
                    EnumRealInMessagesType::Msg_what(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_yes(the_real_outmsg));
                }
                (
                    OutMsg_yes::Cnd_normal_hm_or_1,
                    EnumRealInMessagesType::Msg_or(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_yes(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'yes' in prod_yes".to_string());
        }
    }
}
