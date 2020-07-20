use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_msg_id;
use std::collections::BTreeSet;

pub fn c_gen_customrealmsg_h(all_messages: &BTreeSet<String>) -> String {
    let realmsgs = all_messages
        .iter()
        .map(|msg| {
            format!(
                "
typedef struct {msgid}_real {{
  char what[{size}];
}}{msgid}_real;",
                msgid = build_msg_id(msg),
                size = 16 + ((16.0 * rand::random::<f32>()).round() as usize)
            )
        })
        .collect::<Vec<String>>()
        .join("");

    return format!(
        "#ifndef CUSTOMREALMSG_H
#define CUSTOMREALMSG_H
{realmsgs}
#endif /* CUSTOMREALMSG_H */
",
        realmsgs = realmsgs
    );
}
