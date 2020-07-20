use crate::applic_folder::fsm_folder::fsm::Fsm;
use crate::applic_folder::fsm_folder::gen_folder::genrust_folder::genall::build_cnd_id;

impl Fsm {
    pub fn gengguardconditions(
        &mut self,
        _use_lower: &'static str,
        _use_upper: &'static str,
        _no_mess: &String,
    ) -> String {
        let grp_state_event = self.gen_state_event_group();
        let mut all_guardconds: Vec<String> = Vec::new();

        for ((state, event), rows) in grp_state_event.iter() {
            all_guardconds.push(format!(
                "#[derive(Debug, Clone)]\npub enum Conds_{state}_{event} {{",
                state = state.to_lowercase(),
                event = event.to_lowercase()
            ));
            for arow in rows.iter() {
                all_guardconds.push(format!(
                    "\t{cond},",
                    cond = build_cnd_id(&arow.synt_guard_result)
                ));
            }
            all_guardconds.push("}\n".to_string());
        }
        return format!("{allguardconds}", allguardconds = all_guardconds.join("\n"));
    }
}
