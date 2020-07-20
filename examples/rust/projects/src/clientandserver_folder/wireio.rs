use super::super::clientandserver_folder::server_main::Server_main_Object;
#[derive(Debug)]
pub struct WireIO {
    it: u8,
    the_listener: Server_main_Object,
    //    the_first_speaker: Option<Client_main_Object>,
}

impl WireIO {
    pub fn new(the_listener: Server_main_Object) -> WireIO {
        WireIO {
            it: 0,
            the_listener: the_listener,
        }
    }
    /*
    pub fn socket_and_bind(&mut self, the_listener: Server_main_Object) {
        self.the_listener = Some(the_listener);
    }
    pub fn socket(&mut self, the_first_speaker: Client_main_Object) {
        self.the_first_speaker = Some(the_first_speaker);
    }
    */
    pub fn sendto_and_recvfrom(
        &mut self,
        //        tag: String,
        send_wire_data: &Vec<u8>,
    ) -> Vec<u8> {
        //      let (gottag, gotwire) = self.the_listener.sendto_and_recvfrom(send_wire_data);
        let gotwire = self.the_listener.sendto_and_recvfrom(send_wire_data);
        //      return (gottag, gotwire);
        return gotwire;
    }
}
