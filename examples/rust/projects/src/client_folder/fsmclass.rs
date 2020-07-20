use super::business_class::BusinessObject;
use super::guardclass::Guardclass;
use super::guardconditions::*;
use super::messagesets::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum States {
    St_startup_how_are_you,
    St_normal_well,
    St_awaitconfirm_no,
    St_init_ping,
    St_keyexchage_24,
    St_init,
    St_awaitconfirm_other,
    St_normal_hm,
    St_init_hello_alice,
    St_final,
    St_awaitconfirm_yes,
    St_keyexchage_42,
}

#[derive(Debug)]
pub struct Fsmclass {
    pub the_guardobj: Guardclass,
}
impl Fsmclass {
    pub fn new(the_guardobj: Guardclass) -> Fsmclass {
        Fsmclass {
            the_guardobj: the_guardobj,
        }
    }

    pub fn get_possible_init_messages(&mut self) -> Vec<&'static str> {
        return vec!["callin"];
    }

    pub fn get_initial_state(&mut self) -> States {
        return States::St_init;
    }
}
impl Fsmclass {
    pub fn is_state_final(&mut self, st: &mut States) -> bool {
        return *st == States::St_final;
    }
    pub fn take_event(
        &mut self,
        an_event: &EnumRealInMessagesType,
        st: &mut States,
        business_object: &mut BusinessObject,
    ) -> Result<EnumRealOutMessagesType, String> {
        match (*st, an_event) {
            (States::St_awaitconfirm_no, EnumRealInMessagesType::Msg_what(_)) => match self
                .the_guardobj
                .awaitconfirm_no_what(an_event, business_object)
            {
                /* User Guard is: AwaitConfirm_No_What_25 */
                Ok((Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_1, outmsg)) => {
                    *st = States::St_awaitconfirm_no;
                    match outmsg {
    EnumRealOutMessagesType::Msg_no{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_awaitconfirm_no, InMsg=Msg_what the OutMsg must be of kind 'Msg_no', but was='{was:?}'",was=outmsg))}
  }
                }
                /* User Guard is: AwaitConfirm_No_What_27 */
                Ok((Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_2, outmsg)) => {
                    *st = States::St_awaitconfirm_other;
                    match outmsg {
    EnumRealOutMessagesType::Msg_other{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_awaitconfirm_no, InMsg=Msg_what the OutMsg must be of kind 'Msg_other', but was='{was:?}'",was=outmsg))}
  }
                }
                /* User Guard is: AwaitConfirm_No_What_7 */
                Ok((Conds_awaitconfirm_no_what::Cnd_awaitconfirm_no_what_3, outmsg)) => {
                    *st = States::St_awaitconfirm_yes;
                    match outmsg {
    EnumRealOutMessagesType::Msg_yes{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_awaitconfirm_no, InMsg=Msg_what the OutMsg must be of kind 'Msg_yes', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_awaitconfirm_no,InMsg=Msg_what,GuardFunction=awaitconfirm_no_what) error from function was ={}",err));
                }
            },

            (States::St_awaitconfirm_other, EnumRealInMessagesType::Msg_bye(_)) => match self
                .the_guardobj
                .awaitconfirm_other_bye(an_event, business_object)
            {
                /* User Guard is: AwaitConfirm_Other_Bye_16 */
                Ok((Conds_awaitconfirm_other_bye::Cnd_awaitconfirm_other_bye_1, outmsg)) => {
                    *st = States::St_final;
                    match outmsg {
    EnumRealOutMessagesType::Msg_nooutput{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_awaitconfirm_other, InMsg=Msg_bye the OutMsg must be of kind 'Msg_nooutput', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_awaitconfirm_other,InMsg=Msg_bye,GuardFunction=awaitconfirm_other_bye) error from function was ={}",err));
                }
            },

            (States::St_awaitconfirm_yes, EnumRealInMessagesType::Msg_blablabla(_)) => match self
                .the_guardobj
                .awaitconfirm_yes_blablabla(an_event, business_object)
            {
                /* User Guard is: AwaitConfirm_Yes_Blablabla_10 */
                Ok((
                    Conds_awaitconfirm_yes_blablabla::Cnd_awaitconfirm_yes_blablabla_1,
                    outmsg,
                )) => {
                    *st = States::St_normal_hm;
                    match outmsg {
    EnumRealOutMessagesType::Msg_hm{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_awaitconfirm_yes, InMsg=Msg_blablabla the OutMsg must be of kind 'Msg_hm', but was='{was:?}'",was=outmsg))}
  }
                }
                /* User Guard is: AwaitConfirm_Yes_Blablabla_5 */
                Ok((
                    Conds_awaitconfirm_yes_blablabla::Cnd_awaitconfirm_yes_blablabla_2,
                    outmsg,
                )) => {
                    *st = States::St_normal_well;
                    match outmsg {
    EnumRealOutMessagesType::Msg_well{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_awaitconfirm_yes, InMsg=Msg_blablabla the OutMsg must be of kind 'Msg_well', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_awaitconfirm_yes,InMsg=Msg_blablabla,GuardFunction=awaitconfirm_yes_blablabla) error from function was ={}",err));
                }
            },

            (States::St_init, EnumRealInMessagesType::Msg_callin(_)) => {
                match self.the_guardobj.init_callin(an_event, business_object) {
                    /* User Guard is: INIT_Callin_14 */
                    Ok((Conds_init_callin::Cnd_init_callin_1, outmsg)) => {
                        *st = States::St_init_ping;
                        match outmsg {
    EnumRealOutMessagesType::Msg_ping{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_init, InMsg=Msg_callin the OutMsg must be of kind 'Msg_ping', but was='{was:?}'",was=outmsg))}
  }
                    }
                    /* User Guard is: INIT_Callin_6 */
                    Ok((Conds_init_callin::Cnd_init_callin_2, outmsg)) => {
                        *st = States::St_init_hello_alice;
                        match outmsg {
    EnumRealOutMessagesType::Msg_hello_alice{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_init, InMsg=Msg_callin the OutMsg must be of kind 'Msg_hello_alice', but was='{was:?}'",was=outmsg))}
  }
                    }
                    Err(err) => {
                        return Err(format!("In (State=St_init,InMsg=Msg_callin,GuardFunction=init_callin) error from function was ={}",err));
                    }
                }
            }

            (States::St_init_hello_alice, EnumRealInMessagesType::Msg_dontdisturb(_)) => match self
                .the_guardobj
                .init_hello_alice_dontdisturb(an_event, business_object)
            {
                /* User Guard is: INIT_Hello_Alice_DontDisturb_4 */
                Ok((
                    Conds_init_hello_alice_dontdisturb::Cnd_init_hello_alice_dontdisturb_1,
                    outmsg,
                )) => {
                    *st = States::St_final;
                    match outmsg {
    EnumRealOutMessagesType::Msg_nooutput{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_init_hello_alice, InMsg=Msg_dontdisturb the OutMsg must be of kind 'Msg_nooutput', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_init_hello_alice,InMsg=Msg_dontdisturb,GuardFunction=init_hello_alice_dontdisturb) error from function was ={}",err));
                }
            },

            (States::St_init_hello_alice, EnumRealInMessagesType::Msg_hello_bob(_)) => match self
                .the_guardobj
                .init_hello_alice_hello_bob(an_event, business_object)
            {
                /* User Guard is: INIT_Hello_Alice_Hello_Bob_9 */
                Ok((
                    Conds_init_hello_alice_hello_bob::Cnd_init_hello_alice_hello_bob_1,
                    outmsg,
                )) => {
                    *st = States::St_startup_how_are_you;
                    match outmsg {
    EnumRealOutMessagesType::Msg_how_are_you{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_init_hello_alice, InMsg=Msg_hello_bob the OutMsg must be of kind 'Msg_how_are_you', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_init_hello_alice,InMsg=Msg_hello_bob,GuardFunction=init_hello_alice_hello_bob) error from function was ={}",err));
                }
            },

            (States::St_init_ping, EnumRealInMessagesType::Msg_pong(_)) => {
                match self.the_guardobj.init_ping_pong(an_event, business_object) {
                    /* User Guard is: INIT_Ping_Pong_20 */
                    Ok((Conds_init_ping_pong::Cnd_init_ping_pong_1, outmsg)) => {
                        *st = States::St_startup_how_are_you;
                        match outmsg {
    EnumRealOutMessagesType::Msg_how_are_you{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_init_ping, InMsg=Msg_pong the OutMsg must be of kind 'Msg_how_are_you', but was='{was:?}'",was=outmsg))}
  }
                    }
                    Err(err) => {
                        return Err(format!("In (State=St_init_ping,InMsg=Msg_pong,GuardFunction=init_ping_pong) error from function was ={}",err));
                    }
                }
            }

            (States::St_init_ping, EnumRealInMessagesType::Msg_sorry(_)) => {
                match self.the_guardobj.init_ping_sorry(an_event, business_object) {
                    /* User Guard is: INIT_Ping_Sorry_13 */
                    Ok((Conds_init_ping_sorry::Cnd_init_ping_sorry_1, outmsg)) => {
                        *st = States::St_final;
                        match outmsg {
    EnumRealOutMessagesType::Msg_nooutput{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_init_ping, InMsg=Msg_sorry the OutMsg must be of kind 'Msg_nooutput', but was='{was:?}'",was=outmsg))}
  }
                    }
                    Err(err) => {
                        return Err(format!("In (State=St_init_ping,InMsg=Msg_sorry,GuardFunction=init_ping_sorry) error from function was ={}",err));
                    }
                }
            }

            (States::St_keyexchage_24, EnumRealInMessagesType::Msg_questionwas(_)) => match self
                .the_guardobj
                .keyexchage_24_questionwas(an_event, business_object)
            {
                /* User Guard is: KeyExchage_24_QuestionWas_19 */
                Ok((Conds_keyexchage_24_questionwas::Cnd_keyexchage_24_questionwas_1, outmsg)) => {
                    *st = States::St_keyexchage_24;
                    match outmsg {
    EnumRealOutMessagesType::Msg_24{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_keyexchage_24, InMsg=Msg_questionwas the OutMsg must be of kind 'Msg_24', but was='{was:?}'",was=outmsg))}
  }
                }
                /* User Guard is: KeyExchage_24_QuestionWas_2 */
                Ok((Conds_keyexchage_24_questionwas::Cnd_keyexchage_24_questionwas_2, outmsg)) => {
                    *st = States::St_keyexchage_42;
                    match outmsg {
    EnumRealOutMessagesType::Msg_42{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_keyexchage_24, InMsg=Msg_questionwas the OutMsg must be of kind 'Msg_42', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_keyexchage_24,InMsg=Msg_questionwas,GuardFunction=keyexchage_24_questionwas) error from function was ={}",err));
                }
            },

            (States::St_keyexchage_42, EnumRealInMessagesType::Msg_43(_)) => match self
                .the_guardobj
                .keyexchage_42_43(an_event, business_object)
            {
                /* User Guard is: KeyExchage_42_43_23 */
                Ok((Conds_keyexchage_42_43::Cnd_keyexchage_42_43_1, outmsg)) => {
                    *st = States::St_normal_hm;
                    match outmsg {
    EnumRealOutMessagesType::Msg_hm{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_keyexchage_42, InMsg=Msg_43 the OutMsg must be of kind 'Msg_hm', but was='{was:?}'",was=outmsg))}
  }
                }
                /* User Guard is: KeyExchage_42_43_26 */
                Ok((Conds_keyexchage_42_43::Cnd_keyexchage_42_43_2, outmsg)) => {
                    *st = States::St_normal_well;
                    match outmsg {
    EnumRealOutMessagesType::Msg_well{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_keyexchage_42, InMsg=Msg_43 the OutMsg must be of kind 'Msg_well', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_keyexchage_42,InMsg=Msg_43,GuardFunction=keyexchage_42_43) error from function was ={}",err));
                }
            },

            (States::St_normal_hm, EnumRealInMessagesType::Msg_blablabla(_)) => match self
                .the_guardobj
                .normal_hm_blablabla(an_event, business_object)
            {
                /* User Guard is: Normal_Hm_Blablabla_11 */
                Ok((Conds_normal_hm_blablabla::Cnd_normal_hm_blablabla_1, outmsg)) => {
                    *st = States::St_normal_well;
                    match outmsg {
    EnumRealOutMessagesType::Msg_well{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal_hm, InMsg=Msg_blablabla the OutMsg must be of kind 'Msg_well', but was='{was:?}'",was=outmsg))}
  }
                }
                /* User Guard is: Normal_Hm_Blablabla_12 */
                Ok((Conds_normal_hm_blablabla::Cnd_normal_hm_blablabla_2, outmsg)) => {
                    *st = States::St_normal_hm;
                    match outmsg {
    EnumRealOutMessagesType::Msg_hm{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal_hm, InMsg=Msg_blablabla the OutMsg must be of kind 'Msg_hm', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_normal_hm,InMsg=Msg_blablabla,GuardFunction=normal_hm_blablabla) error from function was ={}",err));
                }
            },

            (States::St_normal_hm, EnumRealInMessagesType::Msg_or(_)) => {
                match self.the_guardobj.normal_hm_or(an_event, business_object) {
                    /* User Guard is: Normal_Hm_Or_22 */
                    Ok((Conds_normal_hm_or::Cnd_normal_hm_or_1, outmsg)) => {
                        *st = States::St_awaitconfirm_yes;
                        match outmsg {
    EnumRealOutMessagesType::Msg_yes{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal_hm, InMsg=Msg_or the OutMsg must be of kind 'Msg_yes', but was='{was:?}'",was=outmsg))}
  }
                    }
                    /* User Guard is: Normal_Hm_Or_24 */
                    Ok((Conds_normal_hm_or::Cnd_normal_hm_or_2, outmsg)) => {
                        *st = States::St_awaitconfirm_other;
                        match outmsg {
    EnumRealOutMessagesType::Msg_other{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal_hm, InMsg=Msg_or the OutMsg must be of kind 'Msg_other', but was='{was:?}'",was=outmsg))}
  }
                    }
                    /* User Guard is: Normal_Hm_Or_3 */
                    Ok((Conds_normal_hm_or::Cnd_normal_hm_or_3, outmsg)) => {
                        *st = States::St_awaitconfirm_no;
                        match outmsg {
    EnumRealOutMessagesType::Msg_no{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal_hm, InMsg=Msg_or the OutMsg must be of kind 'Msg_no', but was='{was:?}'",was=outmsg))}
  }
                    }
                    Err(err) => {
                        return Err(format!("In (State=St_normal_hm,InMsg=Msg_or,GuardFunction=normal_hm_or) error from function was ={}",err));
                    }
                }
            }

            (States::St_normal_well, EnumRealInMessagesType::Msg_blablabla(_)) => match self
                .the_guardobj
                .normal_well_blablabla(an_event, business_object)
            {
                /* User Guard is: Normal_Well_Blablabla_1 */
                Ok((Conds_normal_well_blablabla::Cnd_normal_well_blablabla_1, outmsg)) => {
                    *st = States::St_normal_well;
                    match outmsg {
    EnumRealOutMessagesType::Msg_well{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal_well, InMsg=Msg_blablabla the OutMsg must be of kind 'Msg_well', but was='{was:?}'",was=outmsg))}
  }
                }
                /* User Guard is: Normal_Well_Blablabla_21 */
                Ok((Conds_normal_well_blablabla::Cnd_normal_well_blablabla_2, outmsg)) => {
                    *st = States::St_normal_hm;
                    match outmsg {
    EnumRealOutMessagesType::Msg_hm{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal_well, InMsg=Msg_blablabla the OutMsg must be of kind 'Msg_hm', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_normal_well,InMsg=Msg_blablabla,GuardFunction=normal_well_blablabla) error from function was ={}",err));
                }
            },

            (States::St_normal_well, EnumRealInMessagesType::Msg_bye_bob(_)) => match self
                .the_guardobj
                .normal_well_bye_bob(an_event, business_object)
            {
                /* User Guard is: Normal_Well_Bye_Bob_15 */
                Ok((Conds_normal_well_bye_bob::Cnd_normal_well_bye_bob_1, outmsg)) => {
                    *st = States::St_final;
                    match outmsg {
    EnumRealOutMessagesType::Msg_nooutput{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal_well, InMsg=Msg_bye_bob the OutMsg must be of kind 'Msg_nooutput', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_normal_well,InMsg=Msg_bye_bob,GuardFunction=normal_well_bye_bob) error from function was ={}",err));
                }
            },

            (States::St_startup_how_are_you, EnumRealInMessagesType::Msg_rich(_)) => match self
                .the_guardobj
                .startup_how_are_you_rich(an_event, business_object)
            {
                /* User Guard is: StartUp_How_Are_You_Rich_17 */
                Ok((Conds_startup_how_are_you_rich::Cnd_startup_how_are_you_rich_1, outmsg)) => {
                    *st = States::St_keyexchage_24;
                    match outmsg {
    EnumRealOutMessagesType::Msg_24{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_startup_how_are_you, InMsg=Msg_rich the OutMsg must be of kind 'Msg_24', but was='{was:?}'",was=outmsg))}
  }
                }
                /* User Guard is: StartUp_How_Are_You_Rich_18 */
                Ok((Conds_startup_how_are_you_rich::Cnd_startup_how_are_you_rich_2, outmsg)) => {
                    *st = States::St_keyexchage_42;
                    match outmsg {
    EnumRealOutMessagesType::Msg_42{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_startup_how_are_you, InMsg=Msg_rich the OutMsg must be of kind 'Msg_42', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_startup_how_are_you,InMsg=Msg_rich,GuardFunction=startup_how_are_you_rich) error from function was ={}",err));
                }
            },

            (States::St_startup_how_are_you, EnumRealInMessagesType::Msg_tired(_)) => match self
                .the_guardobj
                .startup_how_are_you_tired(an_event, business_object)
            {
                /* User Guard is: StartUp_How_Are_You_Tired_8 */
                Ok((Conds_startup_how_are_you_tired::Cnd_startup_how_are_you_tired_1, outmsg)) => {
                    *st = States::St_final;
                    match outmsg {
    EnumRealOutMessagesType::Msg_nooutput{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_startup_how_are_you, InMsg=Msg_tired the OutMsg must be of kind 'Msg_nooutput', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_startup_how_are_you,InMsg=Msg_tired,GuardFunction=startup_how_are_you_tired) error from function was ={}",err));
                }
            },
            _ => {
                let x = format!(
                    "ERROR 3: Inmessage= {:?} not accepted in state= {:?}",
                    an_event, st
                );
                return Err(x);
            }
        }
    }
}
