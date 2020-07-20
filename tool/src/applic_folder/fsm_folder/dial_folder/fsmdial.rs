use crate::applic_folder::fsm_folder::fsm::Fsm;
//use fsm::Fsm;
use crate::applic_folder::fsm_folder::fsm::OutEventAndNextStateAndUserCondValue;
use std::io;

impl Fsm {
    pub fn get_cond_choice(
        &self,
        state_now: String,
        in_event: String,
        name: &str,
    ) -> &OutEventAndNextStateAndUserCondValue {
        println!(
            "FSM Name={}, State Now={}, In Event={}\n Choose a Condition number",
            name, state_now, in_event
        );
        match self.grouped_fsm.get(&state_now) {
            Some(event_collection) => {
                let result_collection = event_collection.get(&in_event).unwrap();
                let result_keys: Vec<_> = result_collection.keys().cloned().collect();
                let mut index_choice = 0;
                let mut result_keys_choice_list: Vec<String> = Vec::new();

                for (cond_value, out_event_and_next_state) in result_collection {
                    result_keys_choice_list.push(format!(
                        "{}->{}, Outevent={}, Nextstate={}",
                        index_choice,
                        cond_value,
                        out_event_and_next_state.out_msg,
                        out_event_and_next_state.to_state
                    ));
                    index_choice += 1;
                }
                println!("{:#?}", result_keys_choice_list);
                let guard_choice_number = Fsm::read_number_from_console();
                // ensure guard_choice_number is within limits
                let retval = result_collection
                    .get(&result_keys[guard_choice_number])
                    .unwrap();
                retval
            }
            None => panic!("client_state {} not found", state_now),
        }
    }

    /*........................................*/
    fn read_number_from_console() -> usize {
        let mut _to_return: usize = 0;
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_length) => {
                // length is length !
                let trimmed_input = input.trim();
                match trimmed_input.parse::<usize>() {
                    Ok(n) => {
                        _to_return = n;
                    }
                    Err(e) => panic!("Error {} when reading number", e),
                }
            }
            Err(e) => panic!("Error {} when reading console", e),
        }
        _to_return
    }
    /*........................................*/

    pub fn dial(server: &mut Fsm, client: &mut Fsm) {
        // first choose some event in the initial state
        let mut client_state = client.get_state_now();
        let mut server_state = server.get_state_now();
        let server_finale_state = server.get_final_state();

        let res_from_client =
            client.get_cond_choice(client_state.to_string(), "Callin".to_string(), "Client");
        // println!("res_from_client={:#?}", res_from_client);

        client_state = res_from_client.to_state.clone();
        let mut event_to_server = res_from_client.out_msg.clone();
        loop {
            let res_from_server =
                server.get_cond_choice(server_state.to_string(), event_to_server, "Server");

            server_state = res_from_server.to_state.clone();
            if server_finale_state == server_state {
                println!("Server went into final state");
                break;
            }
            let event_from_server = res_from_server.out_msg.clone();
            let event_to_client = event_from_server.clone();
            let res_from_client = client.get_cond_choice(
                client_state.to_string(),
                event_to_client.to_string(),
                "Client",
            );
            client_state = res_from_client.to_state.clone();
            event_to_server = res_from_client.out_msg.clone();
        }
    }
}
