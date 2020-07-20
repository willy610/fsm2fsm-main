use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_cnd_id;

impl Fsm {
    pub fn genprodconditions(
        &mut self,
        _use_lower: &'static str,
        _use_upper: &'static str,
        _no_mess: &String,
    ) -> String {
        let grp_outmsg = self.gen_outmsg_group();
        let mut all_msg_out: Vec<String> = Vec::new();

        for (key, vals) in grp_outmsg.iter() {
            all_msg_out.push(format!(
                "#[derive(Debug, Clone)]\npub enum OutMsg_{msgname}{{",
                msgname = key
            ));
            for arow in vals.iter() {
                all_msg_out.push(format!(
                    "\t{cond},",
                    cond = build_cnd_id(&arow.synt_guard_result)
                ));
            }
            all_msg_out.push("}\n".to_string());
        }
        return format!("{allprodcond}", allprodcond = all_msg_out.join("\n"));
    }
}
