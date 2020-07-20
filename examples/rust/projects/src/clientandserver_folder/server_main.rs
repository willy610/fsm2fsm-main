use super::super::server_folder::business_class::BusinessObject;
use super::super::server_folder::fsmclass::Fsmclass;
use super::super::server_folder::fsmclass::States;
use super::super::server_folder::guardclass::Guardclass;
use super::super::server_folder::msgfactoryclass::*;
use super::super::server_folder::prodmsgclass::Prodmsgclass;

#[derive(Debug)]
pub struct Server_main_Object {
    pub something: i8,
    pub last_time: u32,
    pub the_msgfactory_obj: Msgfactoryclass,
    pub server_fsm: Fsmclass,
    pub business_object: BusinessObject,
    pub fsm_state: States,
}
impl Server_main_Object {
    pub fn new(_seed: u16) -> Server_main_Object {
        let _server_msgfactory = Msgfactoryclass::new();
        let _server_productmsg: Prodmsgclass = Prodmsgclass::new(_server_msgfactory);
        let _server_guard: Guardclass = Guardclass::new(_server_productmsg, _seed);

        let mut _the_obj = Server_main_Object {
            something: -1,
            last_time: 0,
            the_msgfactory_obj: Msgfactoryclass::new(),
            server_fsm: Fsmclass::new(_server_guard),
            business_object: BusinessObject::new(),
            fsm_state: States::St_init,
        };
        _the_obj.fsm_state = _the_obj.server_fsm.get_initial_state();
        _the_obj
    }
    pub fn init(&mut self) {}
    pub fn sendto_and_recvfrom(&mut self, receive_wire_data: &Vec<u8>) -> Vec<u8> {
        let in_msg = self
            .the_msgfactory_obj
            .wire2comp(receive_wire_data, receive_wire_data.len());
        let out_msg =
            self.server_fsm
                .take_event(&in_msg, &mut self.fsm_state, &mut self.business_object);
        match out_msg {
            Ok(msg) => {
                let send_wire_data = self.the_msgfactory_obj.comp2wire(&msg);
                eprintln!(
                    "Client >>>>>> {msg_in:^25} >>>>>>> Server\nClient <<<<<< {msg_out:^25} <<<<<<< Server",
                    msg_in = String::from_utf8(receive_wire_data.to_vec()).unwrap(),
                    msg_out = String::from_utf8(send_wire_data.to_vec()).unwrap()
                );
                if self.server_fsm.is_state_final(&mut self.fsm_state) {
                    eprintln!("Server_main_Object:: fsm went into final state");
                }
                return send_wire_data;
            }
            Err(txt) => {
                eprintln!("Server_main_Object::, err={}", txt);
                panic!()
            }
        }
    }
}
