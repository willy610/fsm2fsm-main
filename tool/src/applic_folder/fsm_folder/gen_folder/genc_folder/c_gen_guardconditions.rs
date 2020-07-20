use crate::applic_folder::fsm_folder::fsm::Fsm;

impl Fsm {
    pub fn c_gen_guardconditions(&mut self, prepost: &'static str) -> String {
        let mut _grp_state_event = self.gen_state_event_group();
        let all_conds: Vec<String> = _grp_state_event
            .iter()
            .map(|((_state, _inmsg), ref _rows)| {
                let state_as_lower = _state.clone().to_lowercase();
                let inmsg_as_lower = _inmsg.to_lowercase();
                let condset = _rows
                    .iter()
                    .enumerate()
                    .map(|(index, _arow)| {
                        format!("Cnd_{}_{}_{}", state_as_lower, inmsg_as_lower, 1 + index)
                            .to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(",\n\t");
                format!(
                    "typedef enum  {{
  {condset} 
}}
Cnds_{state}_{inmsg};\n\n",
                    state = state_as_lower,
                    inmsg = inmsg_as_lower,
                    condset = condset
                )
                .to_string()
            })
            .collect::<Vec<String>>();
        //////////////////////
        return format!(
            "#ifndef {prepost}GUARDCONDITIONS_H
#define {prepost}GUARDCONDITIONS_H

{allconds}
#endif /* {prepost}GUARDCONDITIONS_H */
",
            prepost = prepost,
            allconds = all_conds.join(""),
        );
    }
}
