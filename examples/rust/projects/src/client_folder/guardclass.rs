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
    ///////////////////////////  awaitconfirm_no_what  /////////
    pub fn awaitconfirm_no_what(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_awaitconfirm_no_what, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_what(ref _real_msg_what_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(3);
            match i {
                0 => {
                    /* User Guard is: AwaitConfirm_No_What_25 */
                    cond_to_set = Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_1;
                }
                1 => {
                    /* User Guard is: AwaitConfirm_No_What_27 */
                    cond_to_set = Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_2;
                }
                2 => {
                    /* User Guard is: AwaitConfirm_No_What_7 */
                    cond_to_set = Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_3;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_awaitconfirm_no_what' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_1 => {
                    match self.the_prodmsgobj.prod_no(
                        &OutMsg_no::Cnd_awaitconfirm_no_what_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_no) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_2 => {
                    match self.the_prodmsgobj.prod_other(
                        &OutMsg_other::Cnd_awaitconfirm_no_what_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_2,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_other) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_3 => {
                    match self.the_prodmsgobj.prod_yes(
                        &OutMsg_yes::Cnd_awaitconfirm_no_what_3,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_3,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_yes) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_awaitconfirm_no_what' ".to_string());
        }
    }
    ///////////////////////////  awaitconfirm_other_bye  /////////
    pub fn awaitconfirm_other_bye(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_awaitconfirm_other_bye, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_bye(ref _real_msg_bye_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: AwaitConfirm_Other_Bye_16 */
                    cond_to_set = Conds_awaitconfirm_other_bye::Cnd_awaitconfirm_other_bye_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_awaitconfirm_other_bye' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_awaitconfirm_other_bye::Cnd_awaitconfirm_other_bye_1 => {
                    match self.the_prodmsgobj.prod_nooutput(
                        &OutMsg_nooutput::Cnd_awaitconfirm_other_bye_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_awaitconfirm_other_bye::Cnd_awaitconfirm_other_bye_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_nooutput) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err(
                "Wrong message type into guard_awaitconfirm_other_bye' ".to_string(),
            );
        }
    }
    ///////////////////////////  awaitconfirm_yes_blablabla  /////////
    pub fn awaitconfirm_yes_blablabla(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_awaitconfirm_yes_blablabla, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_blablabla(ref _real_msg_blablabla_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: AwaitConfirm_Yes_Blablabla_10 */
                    cond_to_set =
                        Conds_awaitconfirm_yes_blablabla::Cnd_awaitconfirm_yes_blablabla_1;
                }
                1 => {
                    /* User Guard is: AwaitConfirm_Yes_Blablabla_5 */
                    cond_to_set =
                        Conds_awaitconfirm_yes_blablabla::Cnd_awaitconfirm_yes_blablabla_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_awaitconfirm_yes_blablabla' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_awaitconfirm_yes_blablabla::Cnd_awaitconfirm_yes_blablabla_1 => {
                    match self.the_prodmsgobj.prod_hm(
                        &OutMsg_hm::Cnd_awaitconfirm_yes_blablabla_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_awaitconfirm_yes_blablabla::Cnd_awaitconfirm_yes_blablabla_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_hm) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_awaitconfirm_yes_blablabla::Cnd_awaitconfirm_yes_blablabla_2 => {
                    match self.the_prodmsgobj.prod_well(
                        &OutMsg_well::Cnd_awaitconfirm_yes_blablabla_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_awaitconfirm_yes_blablabla::Cnd_awaitconfirm_yes_blablabla_2,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_well) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err(
                "Wrong message type into guard_awaitconfirm_yes_blablabla' ".to_string(),
            );
        }
    }
    ///////////////////////////  init_callin  /////////
    pub fn init_callin(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_init_callin, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_callin(ref _real_msg_callin_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: INIT_Callin_14 */
                    cond_to_set = Conds_init_callin::Cnd_init_callin_1;
                }
                1 => {
                    /* User Guard is: INIT_Callin_6 */
                    cond_to_set = Conds_init_callin::Cnd_init_callin_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_init_callin' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_init_callin::Cnd_init_callin_1 => {
                    match self.the_prodmsgobj.prod_ping(
                        &OutMsg_ping::Cnd_init_callin_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_init_callin::Cnd_init_callin_1, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_ping) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_init_callin::Cnd_init_callin_2 => {
                    match self.the_prodmsgobj.prod_hello_alice(
                        &OutMsg_hello_alice::Cnd_init_callin_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_init_callin::Cnd_init_callin_2, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_hello_alice) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_init_callin' ".to_string());
        }
    }
    ///////////////////////////  init_hello_alice_dontdisturb  /////////
    pub fn init_hello_alice_dontdisturb(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_init_hello_alice_dontdisturb, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_dontdisturb(ref _real_msg_dontdisturb_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: INIT_Hello_Alice_DontDisturb_4 */
                    cond_to_set =
                        Conds_init_hello_alice_dontdisturb::Cnd_init_hello_alice_dontdisturb_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_init_hello_alice_dontdisturb' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_init_hello_alice_dontdisturb::Cnd_init_hello_alice_dontdisturb_1 => {
                    match self.the_prodmsgobj.prod_nooutput(
                        &OutMsg_nooutput::Cnd_init_hello_alice_dontdisturb_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_init_hello_alice_dontdisturb::Cnd_init_hello_alice_dontdisturb_1, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_nooutput) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err(
                "Wrong message type into guard_init_hello_alice_dontdisturb' ".to_string(),
            );
        }
    }
    ///////////////////////////  init_hello_alice_hello_bob  /////////
    pub fn init_hello_alice_hello_bob(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_init_hello_alice_hello_bob, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_hello_bob(ref _real_msg_hello_bob_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: INIT_Hello_Alice_Hello_Bob_9 */
                    cond_to_set =
                        Conds_init_hello_alice_hello_bob::Cnd_init_hello_alice_hello_bob_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_init_hello_alice_hello_bob' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_init_hello_alice_hello_bob::Cnd_init_hello_alice_hello_bob_1 => {
                    match self.the_prodmsgobj.prod_how_are_you(
                        &OutMsg_how_are_you::Cnd_init_hello_alice_hello_bob_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_init_hello_alice_hello_bob::Cnd_init_hello_alice_hello_bob_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_how_are_you) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err(
                "Wrong message type into guard_init_hello_alice_hello_bob' ".to_string(),
            );
        }
    }
    ///////////////////////////  init_ping_pong  /////////
    pub fn init_ping_pong(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_init_ping_pong, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_pong(ref _real_msg_pong_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: INIT_Ping_Pong_20 */
                    cond_to_set = Conds_init_ping_pong::Cnd_init_ping_pong_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_init_ping_pong' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_init_ping_pong::Cnd_init_ping_pong_1 => {
                    match self.the_prodmsgobj.prod_how_are_you(
                        &OutMsg_how_are_you::Cnd_init_ping_pong_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_init_ping_pong::Cnd_init_ping_pong_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_how_are_you) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_init_ping_pong' ".to_string());
        }
    }
    ///////////////////////////  init_ping_sorry  /////////
    pub fn init_ping_sorry(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_init_ping_sorry, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_sorry(ref _real_msg_sorry_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: INIT_Ping_Sorry_13 */
                    cond_to_set = Conds_init_ping_sorry::Cnd_init_ping_sorry_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_init_ping_sorry' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_init_ping_sorry::Cnd_init_ping_sorry_1 => {
                    match self.the_prodmsgobj.prod_nooutput(
                        &OutMsg_nooutput::Cnd_init_ping_sorry_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_init_ping_sorry::Cnd_init_ping_sorry_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_nooutput) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_init_ping_sorry' ".to_string());
        }
    }
    ///////////////////////////  keyexchage_24_questionwas  /////////
    pub fn keyexchage_24_questionwas(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_keyexchage_24_questionwas, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_questionwas(ref _real_msg_questionwas_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: KeyExchage_24_QuestionWas_19 */
                    cond_to_set = Conds_keyexchage_24_questionwas::Cnd_keyexchage_24_questionwas_1;
                }
                1 => {
                    /* User Guard is: KeyExchage_24_QuestionWas_2 */
                    cond_to_set = Conds_keyexchage_24_questionwas::Cnd_keyexchage_24_questionwas_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_keyexchage_24_questionwas' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_keyexchage_24_questionwas::Cnd_keyexchage_24_questionwas_1 => {
                    match self.the_prodmsgobj.prod_24(
                        &OutMsg_24::Cnd_keyexchage_24_questionwas_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_keyexchage_24_questionwas::Cnd_keyexchage_24_questionwas_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_24) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_keyexchage_24_questionwas::Cnd_keyexchage_24_questionwas_2 => {
                    match self.the_prodmsgobj.prod_42(
                        &OutMsg_42::Cnd_keyexchage_24_questionwas_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_keyexchage_24_questionwas::Cnd_keyexchage_24_questionwas_2,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_42) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err(
                "Wrong message type into guard_keyexchage_24_questionwas' ".to_string(),
            );
        }
    }
    ///////////////////////////  keyexchage_42_43  /////////
    pub fn keyexchage_42_43(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_keyexchage_42_43, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_43(ref _real_msg_43_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: KeyExchage_42_43_23 */
                    cond_to_set = Conds_keyexchage_42_43::Cnd_keyexchage_42_43_1;
                }
                1 => {
                    /* User Guard is: KeyExchage_42_43_26 */
                    cond_to_set = Conds_keyexchage_42_43::Cnd_keyexchage_42_43_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_keyexchage_42_43' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_keyexchage_42_43::Cnd_keyexchage_42_43_1 => {
                    match self.the_prodmsgobj.prod_hm(
                        &OutMsg_hm::Cnd_keyexchage_42_43_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_keyexchage_42_43::Cnd_keyexchage_42_43_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_hm) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_keyexchage_42_43::Cnd_keyexchage_42_43_2 => {
                    match self.the_prodmsgobj.prod_well(
                        &OutMsg_well::Cnd_keyexchage_42_43_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_keyexchage_42_43::Cnd_keyexchage_42_43_2,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_well) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_keyexchage_42_43' ".to_string());
        }
    }
    ///////////////////////////  normal_hm_blablabla  /////////
    pub fn normal_hm_blablabla(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_normal_hm_blablabla, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_blablabla(ref _real_msg_blablabla_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: Normal_Hm_Blablabla_11 */
                    cond_to_set = Conds_normal_hm_blablabla::Cnd_normal_hm_blablabla_1;
                }
                1 => {
                    /* User Guard is: Normal_Hm_Blablabla_12 */
                    cond_to_set = Conds_normal_hm_blablabla::Cnd_normal_hm_blablabla_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_normal_hm_blablabla' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_normal_hm_blablabla::Cnd_normal_hm_blablabla_1 => {
                    match self.the_prodmsgobj.prod_well(
                        &OutMsg_well::Cnd_normal_hm_blablabla_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_normal_hm_blablabla::Cnd_normal_hm_blablabla_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_well) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_normal_hm_blablabla::Cnd_normal_hm_blablabla_2 => {
                    match self.the_prodmsgobj.prod_hm(
                        &OutMsg_hm::Cnd_normal_hm_blablabla_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_normal_hm_blablabla::Cnd_normal_hm_blablabla_2,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_hm) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_normal_hm_blablabla' ".to_string());
        }
    }
    ///////////////////////////  normal_hm_or  /////////
    pub fn normal_hm_or(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_normal_hm_or, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_or(ref _real_msg_or_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(3);
            match i {
                0 => {
                    /* User Guard is: Normal_Hm_Or_22 */
                    cond_to_set = Conds_normal_hm_or::Cnd_normal_hm_or_1;
                }
                1 => {
                    /* User Guard is: Normal_Hm_Or_24 */
                    cond_to_set = Conds_normal_hm_or::Cnd_normal_hm_or_2;
                }
                2 => {
                    /* User Guard is: Normal_Hm_Or_3 */
                    cond_to_set = Conds_normal_hm_or::Cnd_normal_hm_or_3;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_normal_hm_or' ".to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_normal_hm_or::Cnd_normal_hm_or_1 => {
                    match self.the_prodmsgobj.prod_yes(
                        &OutMsg_yes::Cnd_normal_hm_or_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_normal_hm_or::Cnd_normal_hm_or_1, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_yes) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_normal_hm_or::Cnd_normal_hm_or_2 => {
                    match self.the_prodmsgobj.prod_other(
                        &OutMsg_other::Cnd_normal_hm_or_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_normal_hm_or::Cnd_normal_hm_or_2, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_other) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_normal_hm_or::Cnd_normal_hm_or_3 => {
                    match self.the_prodmsgobj.prod_no(
                        &OutMsg_no::Cnd_normal_hm_or_3,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((Conds_normal_hm_or::Cnd_normal_hm_or_3, msg_out));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_no) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_normal_hm_or' ".to_string());
        }
    }
    ///////////////////////////  normal_well_blablabla  /////////
    pub fn normal_well_blablabla(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_normal_well_blablabla, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_blablabla(ref _real_msg_blablabla_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: Normal_Well_Blablabla_1 */
                    cond_to_set = Conds_normal_well_blablabla::Cnd_normal_well_blablabla_1;
                }
                1 => {
                    /* User Guard is: Normal_Well_Blablabla_21 */
                    cond_to_set = Conds_normal_well_blablabla::Cnd_normal_well_blablabla_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_normal_well_blablabla' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_normal_well_blablabla::Cnd_normal_well_blablabla_1 => {
                    match self.the_prodmsgobj.prod_well(
                        &OutMsg_well::Cnd_normal_well_blablabla_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_normal_well_blablabla::Cnd_normal_well_blablabla_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_well) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_normal_well_blablabla::Cnd_normal_well_blablabla_2 => {
                    match self.the_prodmsgobj.prod_hm(
                        &OutMsg_hm::Cnd_normal_well_blablabla_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_normal_well_blablabla::Cnd_normal_well_blablabla_2,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_hm) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err(
                "Wrong message type into guard_normal_well_blablabla' ".to_string(),
            );
        }
    }
    ///////////////////////////  normal_well_bye_bob  /////////
    pub fn normal_well_bye_bob(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_normal_well_bye_bob, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_bye_bob(ref _real_msg_bye_bob_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: Normal_Well_Bye_Bob_15 */
                    cond_to_set = Conds_normal_well_bye_bob::Cnd_normal_well_bye_bob_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_normal_well_bye_bob' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_normal_well_bye_bob::Cnd_normal_well_bye_bob_1 => {
                    match self.the_prodmsgobj.prod_nooutput(
                        &OutMsg_nooutput::Cnd_normal_well_bye_bob_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_normal_well_bye_bob::Cnd_normal_well_bye_bob_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_nooutput) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err("Wrong message type into guard_normal_well_bye_bob' ".to_string());
        }
    }
    ///////////////////////////  startup_how_are_you_rich  /////////
    pub fn startup_how_are_you_rich(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_startup_how_are_you_rich, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_rich(ref _real_msg_rich_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(2);
            match i {
                0 => {
                    /* User Guard is: StartUp_How_Are_You_Rich_17 */
                    cond_to_set = Conds_startup_how_are_you_rich::Cnd_startup_how_are_you_rich_1;
                }
                1 => {
                    /* User Guard is: StartUp_How_Are_You_Rich_18 */
                    cond_to_set = Conds_startup_how_are_you_rich::Cnd_startup_how_are_you_rich_2;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_startup_how_are_you_rich' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_startup_how_are_you_rich::Cnd_startup_how_are_you_rich_1 => {
                    match self.the_prodmsgobj.prod_24(
                        &OutMsg_24::Cnd_startup_how_are_you_rich_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_startup_how_are_you_rich::Cnd_startup_how_are_you_rich_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_24) ",
                                txt
                            ));
                        }
                    }
                }
                Conds_startup_how_are_you_rich::Cnd_startup_how_are_you_rich_2 => {
                    match self.the_prodmsgobj.prod_42(
                        &OutMsg_42::Cnd_startup_how_are_you_rich_2,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_startup_how_are_you_rich::Cnd_startup_how_are_you_rich_2,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_42) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err(
                "Wrong message type into guard_startup_how_are_you_rich' ".to_string(),
            );
        }
    }
    ///////////////////////////  startup_how_are_you_tired  /////////
    pub fn startup_how_are_you_tired(
        &mut self,
        _inmsg: &EnumRealInMessagesType,
        business_object: &mut BusinessObject,
    ) -> Result<(Conds_startup_how_are_you_tired, EnumRealOutMessagesType), String> {
        if let EnumRealInMessagesType::Msg_tired(ref _real_msg_tired_type) = _inmsg {
            let cond_to_set;
            // TEST/VERIFY STARTS
            // Read NOTE on test / verify above
            let i = self.myrand.in_lim(1);
            match i {
                0 => {
                    /* User Guard is: StartUp_How_Are_You_Tired_8 */
                    cond_to_set = Conds_startup_how_are_you_tired::Cnd_startup_how_are_you_tired_1;
                }
                _ => {
                    return Result::Err(
                        "Could not make a proper decision in 'guard_startup_how_are_you_tired' "
                            .to_string(),
                    );
                }
            }
            // TEST/VERIFY ENDS
            match cond_to_set {
                Conds_startup_how_are_you_tired::Cnd_startup_how_are_you_tired_1 => {
                    match self.the_prodmsgobj.prod_nooutput(
                        &OutMsg_nooutput::Cnd_startup_how_are_you_tired_1,
                        business_object,
                        _inmsg,
                    ) {
                        Ok(msg_out) => {
                            return Result::Ok((
                                Conds_startup_how_are_you_tired::Cnd_startup_how_are_you_tired_1,
                                msg_out,
                            ));
                        }
                        Err(txt) => {
                            return Result::Err(format!(
                                "Error ({}) from message producer(prod_nooutput) ",
                                txt
                            ));
                        }
                    }
                }
            }
        } else {
            return Result::Err(
                "Wrong message type into guard_startup_how_are_you_tired' ".to_string(),
            );
        }
    }
}
