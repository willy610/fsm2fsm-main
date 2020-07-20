use crate::applic_folder::fsm_folder::fsm::Fsm;
impl Fsm {
    pub fn as_tty(&mut self) -> String {
        let mut col_widths: Vec<usize> = self.headers.iter().map(|x| x.len()).collect();
        //        let col_range: Vec<usize> = vec![0, 1, 2, 5, 3, 4];
        let col_range: Vec<usize> = vec![0, 1, 2, 3, 4];
        for a_trans in self.the_normalized_rows.iter() {
            for (col_index, col_value) in a_trans.iter().enumerate() {
                if col_widths[col_index] < col_value.len() {
                    col_widths[col_index] = col_value.len();
                }
            }
        }
        let top_bottom_row: String = [
            "+",
            &col_range
                .iter()
                .map(|col_nr| String::from("-".repeat(2 + col_widths[*col_nr])))
                .collect::<Vec<_>>()
                .join("+"),
            "+",
            "\n",
        ]
        .concat();
        let hdr_row: String = [
            "+",
            &col_range
                .iter()
                .map(|col_nr| {
                    format!(
                        "{:-^1$}",
                        self.headers[*col_nr].clone(),
                        2 + col_widths[*col_nr]
                    )
                })
                .collect::<Vec<_>>()
                .join("+"),
            "+",
            "\n",
        ]
        .concat();
        let mut ret_rows: Vec<String> = Vec::new();
        ret_rows.push("\n".to_string());
        ret_rows.push(hdr_row);
        for (from_state, events) in self.grouped_fsm.iter() {
            let mut event_rows: Vec<String> = Vec::new();
            for (in_event, guard_result) in events {
                let mut guard_rows: Vec<String> = Vec::new();
                // for each event therer will be one more guard conditions
                for (guard_value, out_event_and_next_state) in guard_result {
                    guard_rows.push(
                        vec![
                            //UserGuardValue
                            format!("{: <1$}", guard_value, 2 + col_widths[2]),
                            "|".to_string(),
                            /*
                            //SynteticGuardValue
                            format!(
                                "{: <1$}",
                                out_event_and_next_state.user_guard_result,
                                2 + col_widths[5]
                            ),
                            "|".to_string(),
                            */
                            //OutMessage
                            format!(
                                "{: <1$}",
                                out_event_and_next_state.out_msg,
                                2 + col_widths[3]
                            ),
                            "|".to_string(),
                            //NextState
                            format!(
                                "{: <1$}",
                                out_event_and_next_state.to_state,
                                2 + col_widths[4]
                            ),
                        ]
                        .concat(),
                    );
                }
                // we have one in_event and possible several guardresults
                let event_col_val = format!("{: <1$}", in_event, 2 + col_widths[1]);
                let empty_event = format!("{: <1$}", " ", 2 + col_widths[1]);
                for a_row_nr in 0..guard_rows.len() {
                    if a_row_nr == 0 {
                        event_rows.push(
                            vec![
                                event_col_val.clone(),
                                "|".to_string(),
                                guard_rows[a_row_nr].clone(),
                            ]
                            .concat(),
                        );
                    } else {
                        event_rows.push(
                            vec![
                                empty_event.clone(),
                                "|".to_string(),
                                guard_rows[a_row_nr].clone(),
                            ]
                            .concat(),
                        );
                    }
                }
            }
            // have one from_state
            let state_col_val = format!("{: <1$}", from_state, 2 + col_widths[0]);
            let empty_state = format!("{: <1$}", " ", 2 + col_widths[0]);

            for a_row_nr in 0..event_rows.len() {
                if a_row_nr == 0 {
                    ret_rows.push(
                        vec![
                            "|".to_string(),
                            state_col_val.clone(),
                            "|".to_string(),
                            event_rows[a_row_nr].clone(),
                            "|".to_string(),
                            "\n".to_string(),
                        ]
                        .concat(),
                    );
                } else {
                    ret_rows.push(
                        vec![
                            "|".to_string(),
                            empty_state.clone(),
                            "|".to_string(),
                            event_rows[a_row_nr].clone(),
                            "|".to_string(),
                            "\n".to_string(),
                        ]
                        .concat(),
                    );
                }
            }
        }
        ret_rows.push(top_bottom_row);
        ret_rows.concat()
    }
}
