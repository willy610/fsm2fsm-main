use super::business_class::BusinessObject;
use super::guardclass::Guardclass;
use super::guardconditions::*;
use super::messagesets::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum States {
    St_awaitconfirm,
    St_keyexchage,
    St_normal,
    St_startup,
    St_init,
    St_final,
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
        return vec!["hello_alice", "ping"];
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
            (States::St_awaitconfirm, EnumRealInMessagesType::Msg_no(_)) => {
                match self.the_guardobj.awaitconfirm_no(an_event, business_object) {
                    /* User Guard is: Retry */
                    Ok((Conds_awaitconfirm_no::Cnd_awaitconfirm_no_1, outmsg)) => {
                        *st = States::St_awaitconfirm;
                        match outmsg {
    EnumRealOutMessagesType::Msg_what{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_awaitconfirm, InMsg=Msg_no the OutMsg must be of kind 'Msg_what', but was='{was:?}'",was=outmsg))}
  }
                    }
                    Err(err) => {
                        return Err(format!("In (State=St_awaitconfirm,InMsg=Msg_no,GuardFunction=awaitconfirm_no) error from function was ={}",err));
                    }
                }
            }

            (States::St_awaitconfirm, EnumRealInMessagesType::Msg_other(_)) => match self
                .the_guardobj
                .awaitconfirm_other(an_event, business_object)
            {
                /* User Guard is: Realbad */
                Ok((Conds_awaitconfirm_other::Cnd_awaitconfirm_other_1, outmsg)) => {
                    *st = States::St_final;
                    match outmsg {
    EnumRealOutMessagesType::Msg_bye{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_awaitconfirm, InMsg=Msg_other the OutMsg must be of kind 'Msg_bye', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_awaitconfirm,InMsg=Msg_other,GuardFunction=awaitconfirm_other) error from function was ={}",err));
                }
            },

            (States::St_awaitconfirm, EnumRealInMessagesType::Msg_yes(_)) => match self
                .the_guardobj
                .awaitconfirm_yes(an_event, business_object)
            {
                /* User Guard is: Good */
                Ok((Conds_awaitconfirm_yes::Cnd_awaitconfirm_yes_1, outmsg)) => {
                    *st = States::St_normal;
                    match outmsg {
    EnumRealOutMessagesType::Msg_blablabla{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_awaitconfirm, InMsg=Msg_yes the OutMsg must be of kind 'Msg_blablabla', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_awaitconfirm,InMsg=Msg_yes,GuardFunction=awaitconfirm_yes) error from function was ={}",err));
                }
            },

            (States::St_init, EnumRealInMessagesType::Msg_hello_alice(_)) => match self
                .the_guardobj
                .init_hello_alice(an_event, business_object)
            {
                /* User Guard is: Alert */
                Ok((Conds_init_hello_alice::Cnd_init_hello_alice_1, outmsg)) => {
                    *st = States::St_startup;
                    match outmsg {
    EnumRealOutMessagesType::Msg_hello_bob{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_init, InMsg=Msg_hello_alice the OutMsg must be of kind 'Msg_hello_bob', but was='{was:?}'",was=outmsg))}
  }
                }
                /* User Guard is: Tired */
                Ok((Conds_init_hello_alice::Cnd_init_hello_alice_2, outmsg)) => {
                    *st = States::St_final;
                    match outmsg {
    EnumRealOutMessagesType::Msg_dontdisturb{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_init, InMsg=Msg_hello_alice the OutMsg must be of kind 'Msg_dontdisturb', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_init,InMsg=Msg_hello_alice,GuardFunction=init_hello_alice) error from function was ={}",err));
                }
            },

            (States::St_init, EnumRealInMessagesType::Msg_ping(_)) => {
                match self.the_guardobj.init_ping(an_event, business_object) {
                    /* User Guard is: Continue */
                    Ok((Conds_init_ping::Cnd_init_ping_1, outmsg)) => {
                        *st = States::St_startup;
                        match outmsg {
    EnumRealOutMessagesType::Msg_pong{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_init, InMsg=Msg_ping the OutMsg must be of kind 'Msg_pong', but was='{was:?}'",was=outmsg))}
  }
                    }
                    /* User Guard is: Done */
                    Ok((Conds_init_ping::Cnd_init_ping_2, outmsg)) => {
                        *st = States::St_final;
                        match outmsg {
    EnumRealOutMessagesType::Msg_sorry{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_init, InMsg=Msg_ping the OutMsg must be of kind 'Msg_sorry', but was='{was:?}'",was=outmsg))}
  }
                    }
                    Err(err) => {
                        return Err(format!("In (State=St_init,InMsg=Msg_ping,GuardFunction=init_ping) error from function was ={}",err));
                    }
                }
            }

            (States::St_keyexchage, EnumRealInMessagesType::Msg_24(_)) => {
                match self.the_guardobj.keyexchage_24(an_event, business_object) {
                    /* User Guard is: Well */
                    Ok((Conds_keyexchage_24::Cnd_keyexchage_24_1, outmsg)) => {
                        *st = States::St_keyexchage;
                        match outmsg {
    EnumRealOutMessagesType::Msg_questionwas{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_keyexchage, InMsg=Msg_24 the OutMsg must be of kind 'Msg_questionwas', but was='{was:?}'",was=outmsg))}
  }
                    }
                    Err(err) => {
                        return Err(format!("In (State=St_keyexchage,InMsg=Msg_24,GuardFunction=keyexchage_24) error from function was ={}",err));
                    }
                }
            }

            (States::St_keyexchage, EnumRealInMessagesType::Msg_42(_)) => {
                match self.the_guardobj.keyexchage_42(an_event, business_object) {
                    /* User Guard is: Goon */
                    Ok((Conds_keyexchage_42::Cnd_keyexchage_42_1, outmsg)) => {
                        *st = States::St_normal;
                        match outmsg {
    EnumRealOutMessagesType::Msg_43{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_keyexchage, InMsg=Msg_42 the OutMsg must be of kind 'Msg_43', but was='{was:?}'",was=outmsg))}
  }
                    }
                    Err(err) => {
                        return Err(format!("In (State=St_keyexchage,InMsg=Msg_42,GuardFunction=keyexchage_42) error from function was ={}",err));
                    }
                }
            }

            (States::St_normal, EnumRealInMessagesType::Msg_hm(_)) => {
                match self.the_guardobj.normal_hm(an_event, business_object) {
                    /* User Guard is: Hardcheck */
                    Ok((Conds_normal_hm::Cnd_normal_hm_1, outmsg)) => {
                        *st = States::St_awaitconfirm;
                        match outmsg {
    EnumRealOutMessagesType::Msg_or{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal, InMsg=Msg_hm the OutMsg must be of kind 'Msg_or', but was='{was:?}'",was=outmsg))}
  }
                    }
                    /* User Guard is: Lot */
                    Ok((Conds_normal_hm::Cnd_normal_hm_2, outmsg)) => {
                        *st = States::St_normal;
                        match outmsg {
    EnumRealOutMessagesType::Msg_blablabla{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal, InMsg=Msg_hm the OutMsg must be of kind 'Msg_blablabla', but was='{was:?}'",was=outmsg))}
  }
                    }
                    Err(err) => {
                        return Err(format!("In (State=St_normal,InMsg=Msg_hm,GuardFunction=normal_hm) error from function was ={}",err));
                    }
                }
            }

            (States::St_normal, EnumRealInMessagesType::Msg_well(_)) => {
                match self.the_guardobj.normal_well(an_event, business_object) {
                    /* User Guard is: Bad */
                    Ok((Conds_normal_well::Cnd_normal_well_1, outmsg)) => {
                        *st = States::St_final;
                        match outmsg {
    EnumRealOutMessagesType::Msg_bye_bob{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal, InMsg=Msg_well the OutMsg must be of kind 'Msg_bye_bob', but was='{was:?}'",was=outmsg))}
  }
                    }
                    /* User Guard is: Lot */
                    Ok((Conds_normal_well::Cnd_normal_well_2, outmsg)) => {
                        *st = States::St_normal;
                        match outmsg {
    EnumRealOutMessagesType::Msg_blablabla{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_normal, InMsg=Msg_well the OutMsg must be of kind 'Msg_blablabla', but was='{was:?}'",was=outmsg))}
  }
                    }
                    Err(err) => {
                        return Err(format!("In (State=St_normal,InMsg=Msg_well,GuardFunction=normal_well) error from function was ={}",err));
                    }
                }
            }

            (States::St_startup, EnumRealInMessagesType::Msg_how_are_you(_)) => match self
                .the_guardobj
                .startup_how_are_you(an_event, business_object)
            {
                /* User Guard is: Good */
                Ok((Conds_startup_how_are_you::Cnd_startup_how_are_you_1, outmsg)) => {
                    *st = States::St_keyexchage;
                    match outmsg {
    EnumRealOutMessagesType::Msg_rich{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_startup, InMsg=Msg_how_are_you the OutMsg must be of kind 'Msg_rich', but was='{was:?}'",was=outmsg))}
  }
                }
                /* User Guard is: Tired */
                Ok((Conds_startup_how_are_you::Cnd_startup_how_are_you_2, outmsg)) => {
                    *st = States::St_final;
                    match outmsg {
    EnumRealOutMessagesType::Msg_tired{..} =>{return Ok(outmsg);},
  _=>{return Err(format!("In State=St_startup, InMsg=Msg_how_are_you the OutMsg must be of kind 'Msg_tired', but was='{was:?}'",was=outmsg))}
  }
                }
                Err(err) => {
                    return Err(format!("In (State=St_startup,InMsg=Msg_how_are_you,GuardFunction=startup_how_are_you) error from function was ={}",err));
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
