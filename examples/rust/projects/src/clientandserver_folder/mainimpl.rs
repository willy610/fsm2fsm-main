use super::super::clientandserver_folder::client_main::Client_main_Object;
use super::super::clientandserver_folder::server_main::Server_main_Object;
use super::super::clientandserver_folder::wireio::WireIO;

pub fn mainimpl(_seed: u16) {
    let mut the_server_main_object: Server_main_Object = Server_main_Object::new(_seed);
    the_server_main_object.init();

    let the_rio = WireIO::new(the_server_main_object);

    let mut the_client_main_object: Client_main_Object = Client_main_Object::new(the_rio, _seed);

    the_client_main_object.init();
    the_client_main_object.go();
}
