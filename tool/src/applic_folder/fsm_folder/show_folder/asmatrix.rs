//use fsm::Fsm;
//use std::collections::BTreeMap;
use crate::applic_folder::fsm_folder::fsm::Fsm;

impl Fsm {
    /*----------------------------------------------------*/
    //    pub fn as_matrix(&self, _headers: &Vec<String>) -> String {
    pub fn as_matrix(&self) -> String {
        /*
        +-------------------+-------------------+----------
        | tostate   ->      |                   |
        |-------------------| Awaitconfirmation |(Init is not here) | Final is here
        | fromstate    -v   |                   |
        +-------------------|-------------------|
        | Awaitconfirmation | No                |
        |                   |  Descision1       |
        |                   |   OutMsg34        |
        |                   | No                |
        |                   |  Descision2       |
        |                   |   OutMsg56        |
        |                   | Yes               |
        |                   |  Descision1       |
        |                   |   OutMsg007       |
        +-------------------+-------------------+------
        | OtherState        |
        | Init is here
        | Final is not here
        */
        let mut from_state_rows: Vec<String> = Vec::new();
        let mut to_state_cols: Vec<String> = Vec::new();
        let mut matrix: Vec<Vec<Vec<String>>> = Vec::new();
        // the first row os for the initial (from) state
        // add a row for that from state
        let mut _cols = (0..(self.grouped_fsm.len() + 1))
            .map(|_| Vec::new())
            .collect();
        matrix.push(_cols);

        // look into all known fromstates

        for (state, _) in self.grouped_fsm.iter() {
            // build from_state_rows
            // build to_state_rows
            if *state == self.initial_state {
                continue;
            }
            if *state == self.final_state {
                continue;
            }
            from_state_rows.push(state.clone());
            to_state_cols.push(state.clone());
            // and add a set of cells (include a final) to the row in matrix
            let mut _cols = (0..(self.grouped_fsm.len() + 1))
                .map(|_| Vec::new())
                .collect();
            matrix.push(_cols)
        }
        // now sort and add init and final
        from_state_rows.sort();
        from_state_rows.insert(0, self.initial_state.clone()); // first
        to_state_cols.sort();
        to_state_cols.push(self.final_state.clone()); // last
                                                      // calc colwidths
        let mut _col_widths: Vec<usize> = to_state_cols.iter().map(|k| k.len()).collect();
        let mut _row_heights: Vec<usize> = vec![1; from_state_rows.len()];
        let mut col_nr: usize;
        let mut row_nr: usize;
        for a_fromstate in from_state_rows.iter() {
            let a_state_key_value = self.grouped_fsm.get(a_fromstate).unwrap();
            for (in_event, result_rows) in a_state_key_value.iter() {
                for (guard_cond, outevent_and_nextstate) in result_rows {
                    row_nr = from_state_rows
                        .iter()
                        .position(|x| x == a_fromstate)
                        .unwrap();
                    col_nr = to_state_cols
                        .iter()
                        .position(|x| *x == outevent_and_nextstate.to_state)
                        .unwrap();
                    // now fill the cell with EVENT\n Cond\n  OutMessage
                    //    println!("row_nr,col_nr={:?},{:?}", row_nr,col_nr);
                    matrix[row_nr][col_nr].push(in_event.to_string());
                    matrix[row_nr][col_nr].push(format!(" {}", guard_cond.to_string()));
                    matrix[row_nr][col_nr]
                        .push(format!("  {}", outevent_and_nextstate.out_msg.to_string()));
                    // check if this cell is the widest sofar
                    _col_widths[col_nr] = matrix[row_nr][col_nr]
                        .iter()
                        .fold(_col_widths[col_nr], |m, ref i| std::cmp::max(m, i.len()));
                    //          println!("max _col_widths={}", _col_widths[col_nr]);
                    // check if this cell is the highets sofar
                    _row_heights[row_nr] = matrix[row_nr]
                        .iter()
                        .fold(_row_heights[row_nr], |m, ref i| std::cmp::max(m, i.len()));
                }
            }
        }
        let mut width_first_col = from_state_rows
            .iter()
            .fold(0, |m, k| std::cmp::max(m, k.len()))
            + 2;
        let fix_part_second_row = "tostate ->";
        let fix_part_fourth_row = "fromstate -v";
        if fix_part_second_row.len() > width_first_col {
            width_first_col = fix_part_second_row.len()
        }
        if fix_part_fourth_row.len() > width_first_col {
            width_first_col = fix_part_fourth_row.len()
        }
        // top and bottom row
        let top_bottom_row: String = [
            String::from("+"),
            String::from("-".repeat(0 + width_first_col)), // first column
            String::from("+"),
            _col_widths
                .iter()
                .map(|w| String::from("-".repeat(*w)))
                .collect::<Vec<_>>()
                .join("+"),
            String::from("+"),
            String::from("\n"),
        ]
        .concat();

        // second row
        let second_row: String = [
            format!("|{: <1$}|", fix_part_second_row, width_first_col),
            //      "|".to_string(),
            _col_widths
                .iter()
                .map(|w| String::from(" ".repeat(*w)))
                .collect::<Vec<_>>()
                .join("|"),
            "|".to_string(),
            "\n".to_string(),
        ]
        .concat();

        // third row
        let third_row = [
            "|".to_string(),
            String::from(" ".repeat(0 + width_first_col)), // first column
            "|".to_string(),
            //      from_state_rows
            (0..to_state_cols.len())
                //        .iter()
                .map(|col_nr| format!("{:-^1$}", to_state_cols[col_nr], 0 + _col_widths[col_nr]))
                .collect::<Vec<_>>()
                .join("|"),
            "|".to_string(),
            "\n".to_string(),
        ]
        .concat();
        // fourth row
        let fourth_row: String = [
            format!("|{: <1$}|", fix_part_fourth_row, width_first_col),
            //      "|".to_string(),
            _col_widths
                .iter()
                .map(|w| String::from(" ".repeat(*w)))
                .collect::<Vec<_>>()
                .join("|"),
            "|".to_string(),
            "\n".to_string(),
        ]
        .concat();
        let mut ret_rows: Vec<String> = Vec::new();
        ret_rows.push("\n".to_string());

        ret_rows.push(top_bottom_row.clone());
        ret_rows.push(second_row);
        ret_rows.push(third_row);
        ret_rows.push(fourth_row);
        ret_rows.push(top_bottom_row.clone());
        for row_nr in 0..to_state_cols.len() {
            // down
            // for each state. it will hold n * rows in each cell
            let mut a_res_row: Vec<String> = Vec::new();
            // walk through each column
            for row_in_cell in 0.._row_heights[row_nr] {
                // down in cell
                // first row in the cell ?
                if row_in_cell == 0 {
                    a_res_row.push(format!(
                        "|{: <1$}|",
                        from_state_rows[row_nr], width_first_col
                    ));
                } else {
                    a_res_row.push(format!("|{: <1$}|", " ", width_first_col)); // spaces
                }
                for col_nr in 0..to_state_cols.len() {
                    if row_in_cell < matrix[row_nr][col_nr].len() {
                        a_res_row.push(format!(
                            "{: <1$}|",
                            matrix[row_nr][col_nr][row_in_cell], _col_widths[col_nr]
                        ));
                    } else {
                        a_res_row.push(format!("{: <1$}|", " ", _col_widths[col_nr]));
                    }
                }
                /*
                for col_nr in 0..col_index {
                  println!(
                    "matrix[row_nr][col_nr].len()={:?}",
                    matrix[row_nr][col_nr].len()
                  );
                  if row_in_cell < matrix[row_nr][col_nr].len() {
                    a_res_row.push(format!(
                      "{: <1$}|",
                      matrix[row_nr][col_nr][row_in_cell], _col_widths[col_nr]
                    ));
                  } else {
                    a_res_row.push(format!("{: <1$}|", " ", _col_widths[col_nr]));
                  }
                }
                */
                a_res_row.push("\n".to_string());
            }
            ret_rows.push(a_res_row.concat());
            ret_rows.push(top_bottom_row.clone());
        }

        ret_rows.concat()
    }
}
