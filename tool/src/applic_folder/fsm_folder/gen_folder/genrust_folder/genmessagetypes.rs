use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_msg_id;
use std::collections::BTreeSet;
impl Fsm {
    pub fn genmessagetypes(&mut self) -> String {
        let _all_in_messages: BTreeSet<String> = BTreeSet::new();
        let _all_out_messages: BTreeSet<String> = BTreeSet::new();

        let grp_outmsg = self.gen_outmsg_group();
        let grp_inmsg = self.gen_inmsg_group();
        // In Real{data}
        /*---------------------------------------*/
        let _in_message_real = grp_inmsg
            .iter()
            .map(|(msg, _)| {
                format!(
                    "#[derive(Debug, Clone)]
pub struct Real{msgid} {{
    pub most: [char:8],
    pub last: char,
}}
type Real{msgid}Type = Real{msgid};",
                    msgid = build_msg_id(msg)
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        // In Real{data}
        /*---------------------------------------*/
        let _out_message_real = grp_outmsg
            .iter()
            .map(|(msg, _)| {
                format!(
                    "#[derive(Debug, Clone)]
pub struct Real{msgid} {{
    pub time: u64,
    pub data: [u8; 8],
}}
type Real{msgid}Type = Real{msgid};",
                    msgid = build_msg_id(msg)
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        // enum Inmessagetypes
        /*---------------------------------------*/
        let in_messages_content = format!(
            "#[derive(Debug, Clone)]\npub enum Inmessagetypes {{\n\t{allmessages}\n}}",
            allmessages = grp_inmsg
                .iter()
                .map(|(msg, _)| build_msg_id(msg))
                .collect::<Vec<String>>()
                .join(",\n\t"),
        );
        // enum Outmessagetypes
        /*---------------------------------------*/
        let out_messages_content = format!(
            "#[derive(Debug, Clone)]\npub enum Outmessagetypes {{\n\t{allmessages}\n}}",
            allmessages = grp_outmsg
                .iter()
                .map(|(msg, _)| build_msg_id(msg))
                .collect::<Vec<String>>()
                .join(",\n\t"),
        );
        // enum EnumRealInMessagesType
        /*---------------------------------------*/
        let in_enum_message_types_content = format!(
            "#[derive(Debug, Clone)]
          pub enum EnumRealInMessages {{\n\t{allmessages}\n}}
          pub type EnumRealInMessagesType = EnumRealInMessages;",
            allmessages = grp_inmsg
                .iter()
                .map(|(msg, _)| format!("{msgid}(Real{msgid}Type)", msgid = build_msg_id(msg)))
                .collect::<Vec<String>>()
                .join(",\n\t"),
        );
        // enum EnumRealOutMessagesType
        /*---------------------------------------*/
        let out_enum_message_types_content = format!(
            "#[derive(Debug, Clone)]
          pub enum EnumRealOutMessages {{\n\t{allmessages}\n}}
          pub type EnumRealOutMessagesType = EnumRealOutMessages;",
            allmessages = grp_outmsg
                .iter()
                .map(|(msg, _)| format!("{msgid}(Real{msgid}Type)", msgid = build_msg_id(msg)))
                .collect::<Vec<String>>()
                .join(",\n\t"),
        );
        ///////
        // types and messages out
        /*---------------------------------------*/
        /*
                let out_messages_content_types = format!(
                    "#[derive(Debug, Clone)]
        pub enum RealOutMessages {{\n\t{allmessages}\n}}",
                    allmessages = grp_outmsg
                        .iter()
                        .map(|(msg, _)| format!("{msgid}(Real{msgid}Type)", msgid = build_msg_id(msg)))
                        .collect::<Vec<String>>()
                        .join(",\n\t"),
                );
                */
        // types and messages in
        /*---------------------------------------*/
        /*
                let in_messages_content_types = format!(
                    "#[derive(Debug, Clone)]
        pub enum RealInMessages {{\n\t{allmessages}\n}}",
                    allmessages = grp_inmsg
                        .iter()
                        .map(|(msg, _)| format!("{msgid}(Real{msgid}Type)", msgid = build_msg_id(msg)))
                        .collect::<Vec<String>>()
                        .join(",\n\t"),
                );
                */
        ///////
        /*---------------------------------------*/
        /*---------------------------------------*/
        let param_to_prod = grp_outmsg
            .iter()
            .map(|(outmsg, rows)| {
                let conds = rows
                    .iter()
                    .map(|arow| {
                        format!(
                            " IN_Cnd_{thecond}(RealMsg_{inmsg}Type)",
                            //                            inmsg = &arow[1].to_lowercase(),
                            inmsg = &arow.in_msg.to_lowercase(),
                            //                            thecond = &arow[2].to_lowercase()
                            thecond = &arow.synt_guard_result.to_lowercase()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(",\n");

                format!(
                    "#[derive(Debug, Clone)]
pub enum ParamTo_{outmsg} {{
{conds}
}}",
                    outmsg = outmsg,
                    conds = conds
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        /*---------------------------------------*/
        // START HERE
        return vec![
            //            in_message_real,
            //            out_message_real,
            "use super::super::shared_folder::realmessages::*;".to_string(),
            in_messages_content,
            out_messages_content,
            in_enum_message_types_content,
            out_enum_message_types_content,
            //            out_messages_content_types,
            //            in_messages_content_types,
            param_to_prod,
        ]
        .join("\n");
    }
}
