use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genc_folder::c_gen_customrealmsg_h::c_gen_customrealmsg_h;
use crate::applic_folder::fsm_folder::gen_folder::genc_folder::c_gen_message_h::c_gen_message_h;
use crate::applic_folder::fsm_folder::gen_folder::genc_folder::c_gen_messagetypes::c_gen_messagetypes;
use crate::applic_folder::fsm_folder::gen_folder::genc_folder::c_gen_msgfactory_c::c_gen_msgfactory_c;
use crate::applic_folder::fsm_folder::gen_folder::genc_folder::c_gen_msgfactory_h::c_gen_msgfactory_h;

use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;

impl Fsm {
    pub fn genc(
        mut obj_server: Fsm,
        mut obj_client: Fsm,
        dest_proj: String,
        new_or_update: String,
        what_only_wanted: bool,
    ) {
        fn write_string_to_file(content: String, file_name: &String) {
            match File::create(file_name.clone()) {
                Err(msg) => panic!("couldn't create {}: {}", file_name.clone(), msg.to_string()),
                Ok(mut file) => {
                    let res = file.write_all(content.as_bytes());
                    match res {
                        Ok(_) => {}
                        Err(txt) => println!("Error writing file {:?}", txt),
                    }
                }
            }
        }
        let mut all_messages: BTreeSet<String> = BTreeSet::new();
        let no_mess = "no_message_to_send".to_string();

        obj_client.collect_messages(&mut all_messages, &no_mess);
        obj_server.collect_messages(&mut all_messages, &no_mess);
        all_messages.remove(&"-".to_string());

        for (content, filename, gen_at_update_too) in vec![
            (
                c_gen_message_h(&all_messages),
                format!("{}/generatedsources/Messages.h", dest_proj),
                true,
            ),
            (
                c_gen_customrealmsg_h(&all_messages),
                format!("{}/generatedsources/CustomRealMsg.h", dest_proj),
                false,
            ),
            (
                c_gen_messagetypes(&all_messages),
                format!("{}/generatedsources/MessageTypes.h", dest_proj),
                true,
            ),
            (
                obj_client.c_gen_guardconditions("C_"),
                format!("{}/generatedsources/C_GuardConditions.h", dest_proj),
                true,
            ),
            (
                obj_server.c_gen_guardconditions("S_"),
                format!("{}/generatedsources/S_GuardConditions.h", dest_proj),
                true,
            ),
            (
                obj_client.c_gen_gi_h("C_"),
                format!("{}/generatedsources/C_GI.h", dest_proj),
                false,
            ),
            (
                obj_server.c_gen_gi_h("S_"),
                format!("{}/generatedsources/S_GI.h", dest_proj),
                false,
            ),
            (
                obj_client.c_gen_gi_c("C_"),
                format!("{}/generatedsources/C_GI.c", dest_proj),
                false,
            ),
            (
                obj_server.c_gen_gi_c("S_"),
                format!("{}/generatedsources/S_GI.c", dest_proj),
                false,
            ),
            (
                obj_client.c_gen_pi_c("C_"),
                format!("{}/generatedsources/C_PI.c", dest_proj),
                false,
            ),
            (
                obj_server.c_gen_pi_c("S_"),
                format!("{}/generatedsources/S_PI.c", dest_proj),
                false,
            ),
            (
                obj_client.c_gen_pi_h("C_"),
                format!("{}/generatedsources/C_PI.h", dest_proj),
                false,
            ),
            (
                obj_server.c_gen_pi_h("S_"),
                format!("{}/generatedsources/S_PI.h", dest_proj),
                false,
            ),
            (
                obj_client.c_gen_fsm_h("C_"),
                format!("{}/generatedsources/C_FSM.h", dest_proj),
                true, // always at both --nwe and --update
            ),
            (
                obj_server.c_gen_fsm_h("S_"),
                format!("{}/generatedsources/S_FSM.h", dest_proj),
                true, // always at both --nwe and --update
            ),
            (
                obj_client.c_gen_fsm_c("C_"),
                format!("{}/generatedsources/C_FSM.c", dest_proj),
                true, // always at both --nwe and --update
            ),
            (
                obj_server.c_gen_fsm_c("S_"),
                format!("{}/generatedsources/S_FSM.c", dest_proj),
                true, // always at both --nwe and --update
            ),
            (
                c_gen_msgfactory_c(&all_messages),
                format!("{}/generatedsources/MsgFactory.c", dest_proj),
                false,
            ),
            (
                c_gen_msgfactory_h(),
                format!("{}/generatedsources/MsgFactory.h", dest_proj),
                false,
            ),
        ] {
            if new_or_update == "new" || (new_or_update == "update" && gen_at_update_too) {
                if what_only_wanted {
                    eprintln! {"File to be written= {}",&filename};
                } else {
                    eprintln! {"File written={}",&filename};
                    write_string_to_file(content, &filename);
                }
            }
        }
    }
}
