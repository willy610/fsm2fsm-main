use super::super::client_folder::business_class::BusinessObject;
use super::super::client_folder::fsmclass::Fsmclass;
use super::super::client_folder::guardclass::Guardclass;
use super::super::client_folder::msgfactoryclass::*;
use super::super::client_folder::prodmsgclass::Prodmsgclass;
use super::super::clientandserver_folder::wireio::WireIO;

#[derive(Debug)]
pub struct Client_main_Object {
    pub something: i8,
    pub last_time: u32,
    pub client_fsm: Fsmclass,
    the_msgfactory_obj: Msgfactoryclass,
    the_wire_io: WireIO,
}
impl Client_main_Object {
    pub fn new(the_wire_io: WireIO, _seed: u16) -> Client_main_Object {
        let _busines_object: BusinessObject = BusinessObject::new();
        let _client_msgfactory = Msgfactoryclass::new();
        let _client_productmsg: Prodmsgclass = Prodmsgclass::new(_client_msgfactory);
        let _client_guard: Guardclass = Guardclass::new(_client_productmsg, _seed);

        Client_main_Object {
            something: -1,
            last_time: 0,
            client_fsm: Fsmclass::new(_client_guard),
            the_msgfactory_obj: Msgfactoryclass::new(),
            the_wire_io: the_wire_io,
        }
    }
    pub fn init(&mut self) {}
    pub fn go(&mut self) {
        let mut st = self.client_fsm.get_initial_state();
        let mut bo = BusinessObject::new();

        let possible_init_messages_to_client = self.client_fsm.get_possible_init_messages();
        let first_of_intitial_possible_init_calls_to_client = self
            .the_msgfactory_obj
            .gen_in_msg_skeleton(possible_init_messages_to_client[0]);

        let mut maxet = 20;
        let mut out_message = self.client_fsm.take_event(
            &first_of_intitial_possible_init_calls_to_client,
            &mut st,
            &mut bo,
        );

        loop {
            match out_message {
                Ok(ref msg_from_client) => {
                    if self.client_fsm.is_state_final(&mut st) {
                        eprintln!("Client_main_Object:: fsm went into final state");
                        maxet = -1;
                    }
                    maxet -= 1;
                    if maxet <= 0 {
                        break;
                    }
                    // let msgfactory convert (comp2wire) output msg from self fsm
                    let out_msg_as_vec_u8 = self.the_msgfactory_obj.comp2wire(msg_from_client);
                    // send it to server (and get a wire form back)
                    let readmsg = self.the_wire_io.sendto_and_recvfrom(&out_msg_as_vec_u8);
                    // convert read wire into computationa form
                    let answer_from_server =
                        self.the_msgfactory_obj.wire2comp(&readmsg, readmsg.len());
                    out_message = self
                        .client_fsm
                        .take_event(&answer_from_server, &mut st, &mut bo);
                }
                Err(txt) => {
                    eprintln!("client_main()::Err:{:?}", txt);
                    break;
                }
            }
        }
    }
}
