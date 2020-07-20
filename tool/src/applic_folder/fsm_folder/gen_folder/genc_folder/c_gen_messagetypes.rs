use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_msg_id;
use std::collections::BTreeSet;

pub fn c_gen_messagetypes(all_messages: &BTreeSet<String>) -> String {
    let msgtypes = all_messages
        .iter()
        .map(|msg| build_msg_id(msg))
        .collect::<Vec<String>>()
        .join(",\n\t");

    return format!(
        "#ifndef MESSAGETYPES_H
#define MESSAGETYPES_H
typedef enum {{
{msgtypes}
}} MESSAGETYPES;

#endif /* MESSAGETYPES_H */
",
        msgtypes = msgtypes
    );
}
