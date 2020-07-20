use super::super::super::super::fsm_folder::fsm::Fsm;

use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
///////
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RowToStateEventGroup {
    pub user_guard_result: String,
    pub out_msg: String,
    pub to_state: String,
    pub synt_guard_result: String,
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RowToOutmsgGroup {
    pub in_msg: String,
    pub user_guard_result: String,
    pub synt_guard_result: String,
}

pub fn build_msg_id(id: &String) -> String {
    return format!("Msg_{}", id.to_lowercase());
}
pub fn build_state_id(id: &String) -> String {
    return format!("St_{}", id.to_lowercase());
}
pub fn build_cnd_id(id: &String) -> String {
    return format!("Cnd_{}", id.to_lowercase());
}
impl Fsm {
    /*****************************************/
    pub fn collect_messages(&mut self, all_messages: &mut BTreeSet<String>, no_mess: &String) {
        for a_trans in &self.the_normalized_rows {
            all_messages.insert(a_trans[1].to_lowercase()); // inevent
            all_messages.insert(a_trans[3].to_lowercase()); // outevent
            all_messages.insert(no_mess.to_lowercase()); // outevent
        }
    }
    /*****************************************/
    pub fn gen_state_event_group(
        &mut self,
    ) -> BTreeMap<(String, String), Vec<RowToStateEventGroup>> {
        // ordered group on state,event
        let mut grp: BTreeMap<(String, String), Vec<RowToStateEventGroup>> = BTreeMap::new();
        for a_trans in &self.the_normalized_rows {
            let val = match grp.entry((a_trans[0].clone(), a_trans[1].clone())) {
                Vacant(entry) => entry.insert(Vec::new()),
                Occupied(entry) => entry.into_mut(),
            };
            //            val.push(a_trans.to_vec());
            val.push(RowToStateEventGroup {
                user_guard_result: a_trans[2].clone(),
                out_msg: a_trans[3].clone(),
                to_state: a_trans[4].clone(),
                synt_guard_result: a_trans[5].clone(),
            })
        }
        return grp;
    }
    /*****************************************/
    pub fn gen_outmsg_group(&mut self) -> BTreeMap<String, Vec<RowToOutmsgGroup>> {
        // ordered group on outmsg
        let mut grp: BTreeMap<String, Vec<RowToOutmsgGroup>> = BTreeMap::new();
        for a_trans in &self.the_normalized_rows {
            let val = match grp.entry(a_trans[3].clone().to_lowercase()) {
                Vacant(entry) => entry.insert(Vec::new()),
                Occupied(entry) => entry.into_mut(),
            };
            //            val.push(a_trans.to_vec());
            val.push(RowToOutmsgGroup {
                in_msg: a_trans[1].clone(),
                user_guard_result: a_trans[2].clone(),
                synt_guard_result: a_trans[5].clone(),
            })
        }
        //        eprintln!("grp={:#?}",grp);
        return grp;
    }
    /*****************************************/
    pub fn gen_inmsg_group(&mut self) -> BTreeMap<String, Vec<Vec<String>>> {
        // ordered group on outmsg
        let mut grp: BTreeMap<String, Vec<Vec<String>>> = BTreeMap::new();
        for a_trans in &self.the_normalized_rows {
            let val = match grp.entry(a_trans[1].clone().to_lowercase()) {
                Vacant(entry) => entry.insert(Vec::new()),
                Occupied(entry) => entry.into_mut(),
            };
            val.push(a_trans.to_vec());
        }
        return grp;
    }
    /*****************************************/
    pub fn gen_sources(
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

        obj_client.collect_messages(
            &mut all_messages,
            /*&mut all_mess_into_client,*/ &no_mess,
        );
        obj_server.collect_messages(
            &mut all_messages,
            /*&mut all_mess_into_server,*/ &no_mess,
        );
        all_messages.remove(&"-".to_string());
        /////////////
        let gen_all_real_messages = || {
            all_messages
                .iter()
                .map(|amsg| {
                    format!(
                        "
#[derive(Debug, Clone)]
  pub struct RealMsg_{amsg} {{pub all_chars: [char; 8]}}
pub type RealMsg_{amsg}Type = RealMsg_{amsg};",
                        amsg = amsg
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        };
        /////////////
        for (content, filename, gen_at_update_too) in vec![
            (
                gen_all_real_messages(),
                format!("{}/src/shared_folder/realmessages.rs", dest_proj),
                false, // keep old
            ),
            (
                obj_client.genmessagetypes(),
                format!("{}/src/client_folder/messagesets.rs", dest_proj),
                true,
            ),
            (
                obj_server.genmessagetypes(),
                format!("{}/src/server_folder/messagesets.rs", dest_proj),
                true,
            ),
            (
                obj_client.genmsgfactoryclass(),
                format!("{}/src/client_folder/msgfactoryclass.rs", dest_proj),
                false, // keep old
            ),
            (
                obj_server.genmsgfactoryclass(),
                format!("{}/src/server_folder/msgfactoryclass.rs", dest_proj),
                false,
            ),
            (
                obj_client.genfsmclass("client", "Client", &no_mess),
                format!("{}/src/client_folder/fsmclass.rs", dest_proj),
                true,
            ),
            (
                obj_server.genfsmclass("client", "Client", &no_mess),
                format!("{}/src/server_folder/fsmclass.rs", dest_proj),
                true,
            ),
            (
                obj_client.genguardclass("client", "Client", &no_mess),
                format!("{}/src/client_folder/guardclass.rs", dest_proj),
                false, // keep old
            ),
            (
                obj_server.genguardclass("client", "Client", &no_mess),
                format!("{}/src/server_folder/guardclass.rs", dest_proj),
                false, // keep old
            ),
            (
                obj_client.genprodmsgclass("client", "Client", &no_mess),
                format!("{}/src/client_folder/prodmsgclass.rs", dest_proj),
                false, // keep old
            ),
            (
                obj_server.genprodmsgclass("client", "Client", &no_mess),
                format!("{}/src/server_folder/prodmsgclass.rs", dest_proj),
                false, // keep old
            ),
            /* CONDITIONS */
            (
                obj_client.gengguardconditions("client", "Client", &no_mess),
                format!("{}/src/client_folder/guardconditions.rs", dest_proj),
                true,
            ),
            (
                obj_server.gengguardconditions("client", "Client", &no_mess),
                format!("{}/src/server_folder/guardconditions.rs", dest_proj),
                true,
            ),
            (
                obj_client.genprodconditions("client", "Client", &no_mess),
                format!("{}/src/client_folder/prodconditions.rs", dest_proj),
                true,
            ),
            (
                obj_server.genprodconditions("client", "Client", &no_mess),
                format!("{}/src/server_folder/prodconditions.rs", dest_proj),
                true,
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
