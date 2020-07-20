use super::super::shared_folder::myrand::Myrand;
use super::business_class::BusinessObject;
use super::guardconditions::*;
use super::messagesets::EnumRealInMessagesType;
use super::messagesets::EnumRealOutMessagesType;
use super::prodconditions::*;
use super::prodmsgclass::Prodmsgclass;

#[derive(Debug)]
pub struct Guardclass {
    pub the_prodmsgobj: Prodmsgclass,
    pub myrand: Myrand,
}
impl Guardclass {
    pub fn new(the_prodmsgobj: Prodmsgclass, _seed: u16) -> Guardclass {
        Guardclass {
            the_prodmsgobj: the_prodmsgobj,
            myrand: Myrand::new(_seed),
        }
    }
    /*
    Note on verify / test.
    NOTE
    ====
    Do a random decision here in TEST
    Normally to make a decision
    A. Peek into inmessage
    B. Consult business object attribute 'something' via
     business_object.something
    C. assign cond to 'cond_to_set'

    */
    ///////////////////////////  awaitconfirm_no  /////////
    pub fn awaitconfirm_no(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_awaitconfirm_no, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_no(ref _real_msg_no_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: Retry */
                    cond_to_set = Conds_awaitconfirm_no::Cnd_awaitconfirm_no_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_awaitconfirm_no' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_awaitconfirm_no::Cnd_awaitconfirm_no_1 => {
                    match self.the_prodmsgobj.prod_what(
                        &OutMsg_what::Cnd_awaitconfirm_no_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_awaitconfirm_no::Cnd_awaitconfirm_no_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_what) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_awaitconfirm_no' ".to_string());
        }
    }
    ///////////////////////////  awaitconfirm_other  /////////
    pub fn awaitconfirm_other(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_awaitconfirm_other, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_other(ref _real_msg_other_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: Realbad */
                    cond_to_set = Conds_awaitconfirm_other::Cnd_awaitconfirm_other_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_awaitconfirm_other' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_awaitconfirm_other::Cnd_awaitconfirm_other_1 => {
                    match self.the_prodmsgobj.prod_bye(
                        &OutMsg_bye::Cnd_awaitconfirm_other_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_awaitconfirm_other::Cnd_awaitconfirm_other_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_bye) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_awaitconfirm_other' ".to_string());
        }
    }
    ///////////////////////////  awaitconfirm_yes  /////////
    pub fn awaitconfirm_yes(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_awaitconfirm_yes, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_yes(ref _real_msg_yes_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: Good */
                    cond_to_set = Conds_awaitconfirm_yes::Cnd_awaitconfirm_yes_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_awaitconfirm_yes' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_awaitconfirm_yes::Cnd_awaitconfirm_yes_1 => {
                    match self.the_prodmsgobj.prod_blablabla(
                        &OutMsg_blablabla::Cnd_awaitconfirm_yes_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_awaitconfirm_yes::Cnd_awaitconfirm_yes_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_blablabla) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_awaitconfirm_yes' ".to_string());
        }
    }
    ///////////////////////////  init_hello_alice  /////////
    pub fn init_hello_alice(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_init_hello_alice, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_hello_alice(ref _real_msg_hello_alice_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: Alert */
                    cond_to_set = Conds_init_hello_alice::Cnd_init_hello_alice_1;
                }
                1 => {
                    /* User Guard is: Tired */
                    cond_to_set = Conds_init_hello_alice::Cnd_init_hello_alice_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_init_hello_alice' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_init_hello_alice::Cnd_init_hello_alice_1 => {
                    match self.the_prodmsgobj.prod_hello_bob(
                        &OutMsg_hello_bob::Cnd_init_hello_alice_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_init_hello_alice::Cnd_init_hello_alice_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_hello_bob) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_init_hello_alice::Cnd_init_hello_alice_2 => {
                    match self.the_prodmsgobj.prod_dontdisturb(
                        &OutMsg_dontdisturb::Cnd_init_hello_alice_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_init_hello_alice::Cnd_init_hello_alice_2,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_dontdisturb) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_init_hello_alice' ".to_string());
        }
    }
    ///////////////////////////  init_ping  /////////
    pub fn init_ping(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_init_ping, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_ping(ref _real_msg_ping_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: Continue */
                    cond_to_set = Conds_init_ping::Cnd_init_ping_1;
                }
                1 => {
                    /* User Guard is: Done */
                    cond_to_set = Conds_init_ping::Cnd_init_ping_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_init_ping' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_init_ping::Cnd_init_ping_1 => {
                    match self.the_prodmsgobj.prod_pong(
                        &OutMsg_pong::Cnd_init_ping_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_init_ping::Cnd_init_ping_1, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_pong) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_init_ping::Cnd_init_ping_2 => {
                    match self.the_prodmsgobj.prod_sorry(
                        &OutMsg_sorry::Cnd_init_ping_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_init_ping::Cnd_init_ping_2, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_sorry) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_init_ping' ".to_string());
        }
    }
    ///////////////////////////  keyexchage_24  /////////
    pub fn keyexchage_24(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_keyexchage_24, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_24(ref _real_msg_24_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: Well */
                    cond_to_set = Conds_keyexchage_24::Cnd_keyexchage_24_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_keyexchage_24' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_keyexchage_24::Cnd_keyexchage_24_1 => {
                    match self.the_prodmsgobj.prod_questionwas(
                        &OutMsg_questionwas::Cnd_keyexchage_24_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_keyexchage_24::Cnd_keyexchage_24_1, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_questionwas) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_keyexchage_24' ".to_string());
        }
    }
    ///////////////////////////  keyexchage_42  /////////
    pub fn keyexchage_42(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_keyexchage_42, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_42(ref _real_msg_42_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: Goon */
                    cond_to_set = Conds_keyexchage_42::Cnd_keyexchage_42_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_keyexchage_42' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_keyexchage_42::Cnd_keyexchage_42_1 => {
                    match self.the_prodmsgobj.prod_43(
                        &OutMsg_43::Cnd_keyexchage_42_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_keyexchage_42::Cnd_keyexchage_42_1, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_43) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_keyexchage_42' ".to_string());
        }
    }
    ///////////////////////////  normal_hm  /////////
    pub fn normal_hm(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_normal_hm, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_hm(ref _real_msg_hm_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: Hardcheck */
                    cond_to_set = Conds_normal_hm::Cnd_normal_hm_1;
                }
                1 => {
                    /* User Guard is: Lot */
                    cond_to_set = Conds_normal_hm::Cnd_normal_hm_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_normal_hm' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_normal_hm::Cnd_normal_hm_1 => {
                    match self.the_prodmsgobj.prod_or(
                        &OutMsg_or::Cnd_normal_hm_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_normal_hm::Cnd_normal_hm_1, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_or) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_normal_hm::Cnd_normal_hm_2 => {
                    match self.the_prodmsgobj.prod_blablabla(
                        &OutMsg_blablabla::Cnd_normal_hm_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_normal_hm::Cnd_normal_hm_2, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_blablabla) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_normal_hm' ".to_string());
        }
    }
    ///////////////////////////  normal_well  /////////
    pub fn normal_well(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_normal_well, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_well(ref _real_msg_well_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: Bad */
                    cond_to_set = Conds_normal_well::Cnd_normal_well_1;
                }
                1 => {
                    /* User Guard is: Lot */
                    cond_to_set = Conds_normal_well::Cnd_normal_well_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_normal_well' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_normal_well::Cnd_normal_well_1 => {
                    match self.the_prodmsgobj.prod_bye_bob(
                        &OutMsg_bye_bob::Cnd_normal_well_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_normal_well::Cnd_normal_well_1, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_bye_bob) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_normal_well::Cnd_normal_well_2 => {
                    match self.the_prodmsgobj.prod_blablabla(
                        &OutMsg_blablabla::Cnd_normal_well_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_normal_well::Cnd_normal_well_2, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_blablabla) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_normal_well' ".to_string());
        }
    }
    ///////////////////////////  startup_how_are_you  /////////
    pub fn startup_how_are_you(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_startup_how_are_you, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_how_are_you(ref _real_msg_how_are_you_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: Good */
                    cond_to_set = Conds_startup_how_are_you::Cnd_startup_how_are_you_1;
                }
                1 => {
                    /* User Guard is: Tired */
                    cond_to_set = Conds_startup_how_are_you::Cnd_startup_how_are_you_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_startup_how_are_you' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_startup_how_are_you::Cnd_startup_how_are_you_1 => {
                    match self.the_prodmsgobj.prod_rich(
                        &OutMsg_rich::Cnd_startup_how_are_you_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_startup_how_are_you::Cnd_startup_how_are_you_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_rich) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_startup_how_are_you::Cnd_startup_how_are_you_2 => {
                    match self.the_prodmsgobj.prod_tired(
                        &OutMsg_tired::Cnd_startup_how_are_you_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_startup_how_are_you::Cnd_startup_how_are_you_2,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_tired) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_startup_how_are_you' ".to_string());
        }
    }
}
