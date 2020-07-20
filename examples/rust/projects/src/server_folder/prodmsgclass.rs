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
    /*--------------  prod_43  -------------------------------*/
    pub fn prod_43(
        &mut self,
        _acond: &OutMsg_43,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("43");
        if let EnumRealOutMessagesType::Msg_43(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_43::Cnd_keyexchage_42_1,
                    EnumRealInMessagesType::Msg_42(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_43(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg '43' in prod_43".to_string());
        }
    }

    /*--------------  prod_blablabla  -------------------------------*/
    pub fn prod_blablabla(
        &mut self,
        _acond: &OutMsg_blablabla,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("blablabla");
        if let EnumRealOutMessagesType::Msg_blablabla(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_blablabla::Cnd_awaitconfirm_yes_1,
                    EnumRealInMessagesType::Msg_yes(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_blablabla(the_real_outmsg));
                }
                (
                    OutMsg_blablabla::Cnd_normal_hm_2,
                    EnumRealInMessagesType::Msg_hm(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_blablabla(the_real_outmsg));
                }
                (
                    OutMsg_blablabla::Cnd_normal_well_2,
                    EnumRealInMessagesType::Msg_well(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_blablabla(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'blablabla' in prod_blablabla".to_string());
        }
    }

    /*--------------  prod_bye  -------------------------------*/
    pub fn prod_bye(
        &mut self,
        _acond: &OutMsg_bye,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("bye");
        if let EnumRealOutMessagesType::Msg_bye(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_bye::Cnd_awaitconfirm_other_1,
                    EnumRealInMessagesType::Msg_other(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_bye(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'bye' in prod_bye".to_string());
        }
    }

    /*--------------  prod_bye_bob  -------------------------------*/
    pub fn prod_bye_bob(
        &mut self,
        _acond: &OutMsg_bye_bob,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("bye_bob");
        if let EnumRealOutMessagesType::Msg_bye_bob(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_bye_bob::Cnd_normal_well_1,
                    EnumRealInMessagesType::Msg_well(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_bye_bob(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'bye_bob' in prod_bye_bob".to_string());
        }
    }

    /*--------------  prod_dontdisturb  -------------------------------*/
    pub fn prod_dontdisturb(
        &mut self,
        _acond: &OutMsg_dontdisturb,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("dontdisturb");
        if let EnumRealOutMessagesType::Msg_dontdisturb(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_dontdisturb::Cnd_init_hello_alice_2,
                    EnumRealInMessagesType::Msg_hello_alice(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_dontdisturb(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'dontdisturb' in prod_dontdisturb".to_string());
        }
    }

    /*--------------  prod_hello_bob  -------------------------------*/
    pub fn prod_hello_bob(
        &mut self,
        _acond: &OutMsg_hello_bob,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("hello_bob");
        if let EnumRealOutMessagesType::Msg_hello_bob(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_hello_bob::Cnd_init_hello_alice_1,
                    EnumRealInMessagesType::Msg_hello_alice(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_hello_bob(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'hello_bob' in prod_hello_bob".to_string());
        }
    }

    /*--------------  prod_or  -------------------------------*/
    pub fn prod_or(
        &mut self,
        _acond: &OutMsg_or,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("or");
        if let EnumRealOutMessagesType::Msg_or(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (OutMsg_or::Cnd_normal_hm_1, EnumRealInMessagesType::Msg_hm(the_real_inmsg)) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_or(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'or' in prod_or".to_string());
        }
    }

    /*--------------  prod_pong  -------------------------------*/
    pub fn prod_pong(
        &mut self,
        _acond: &OutMsg_pong,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("pong");
        if let EnumRealOutMessagesType::Msg_pong(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_pong::Cnd_init_ping_1,
                    EnumRealInMessagesType::Msg_ping(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_pong(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'pong' in prod_pong".to_string());
        }
    }

    /*--------------  prod_questionwas  -------------------------------*/
    pub fn prod_questionwas(
        &mut self,
        _acond: &OutMsg_questionwas,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("questionwas");
        if let EnumRealOutMessagesType::Msg_questionwas(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_questionwas::Cnd_keyexchage_24_1,
                    EnumRealInMessagesType::Msg_24(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_questionwas(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'questionwas' in prod_questionwas".to_string());
        }
    }

    /*--------------  prod_rich  -------------------------------*/
    pub fn prod_rich(
        &mut self,
        _acond: &OutMsg_rich,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("rich");
        if let EnumRealOutMessagesType::Msg_rich(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_rich::Cnd_startup_how_are_you_1,
                    EnumRealInMessagesType::Msg_how_are_you(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_rich(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'rich' in prod_rich".to_string());
        }
    }

    /*--------------  prod_sorry  -------------------------------*/
    pub fn prod_sorry(
        &mut self,
        _acond: &OutMsg_sorry,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("sorry");
        if let EnumRealOutMessagesType::Msg_sorry(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_sorry::Cnd_init_ping_2,
                    EnumRealInMessagesType::Msg_ping(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_sorry(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'sorry' in prod_sorry".to_string());
        }
    }

    /*--------------  prod_tired  -------------------------------*/
    pub fn prod_tired(
        &mut self,
        _acond: &OutMsg_tired,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("tired");
        if let EnumRealOutMessagesType::Msg_tired(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_tired::Cnd_startup_how_are_you_2,
                    EnumRealInMessagesType::Msg_how_are_you(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_tired(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'tired' in prod_tired".to_string());
        }
    }

    /*--------------  prod_what  -------------------------------*/
    pub fn prod_what(
        &mut self,
        _acond: &OutMsg_what,
        business_object: &mut BusinessObject,
        _inmsg: &EnumRealInMessagesType,
    ) -> Result<EnumRealOutMessagesType, String> {
        // A. In case of error return Err("..some reason (DO NO COMMIT BUT DO ROLLBACK)...".to_string());
        // B. Create an outmsg template
        let outmsgskeleton = self.the_msgfactory.gen_out_msg_skeleton("what");
        if let EnumRealOutMessagesType::Msg_what(mut the_real_outmsg) = outmsgskeleton {
            // Got a skeleton the_real_outmsg
            match (_acond, _inmsg) {
                (
                    OutMsg_what::Cnd_awaitconfirm_no_1,
                    EnumRealInMessagesType::Msg_no(the_real_inmsg),
                ) => {
                    // reference around like this
                    the_real_outmsg.all_chars[7] = the_real_inmsg.all_chars[0];
                    the_real_outmsg.all_chars[0] = business_object.something;
                    business_object.lastlast = the_real_inmsg.all_chars[7];
                    return Ok(EnumRealOutMessages::Msg_what(the_real_outmsg));
                }

                _ => {
                    return Err(format!(
                        "Condition '{:?}' and inmsg '{:?}' not allowed here",
                        _acond, _inmsg
                    ))
                }
            }
        } else {
            return Err("Could not create an outmsg 'what' in prod_what".to_string());
        }
    }
}
