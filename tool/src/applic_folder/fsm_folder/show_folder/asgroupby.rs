use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::BTreeMap;

#[derive(Debug)]
struct GroupByOutMost {
    pub key_names: Vec<&'static str>,
    pub col_widths: Vec<usize>,
}

#[derive(Debug)]
struct GroupByHier {
    pub key_col_index: Vec<usize>,
    pub vals_col_index: Vec<usize>,
    pub groupby: BTreeMap<Vec<String>, Vec<Vec<String>>>,
}
impl GroupByHier {
    pub fn new() -> GroupByHier {
        GroupByHier {
            key_col_index: Vec::new(),
            vals_col_index: Vec::new(),
            groupby: BTreeMap::new(),
        }
    }
    /************************/
    /*
    pub fn set_key_cols(&mut self, out_most: &GroupByOutMost, key_names: &Vec<String>) {
        for a_typed_colname in key_names {
            let is_numeric = a_typed_colname.parse::<usize>();
            match is_numeric {
                Ok(col_index) => self.key_col_index.push(col_index),
                Err(_was_not_numeric_but_col_name) => {
                    match out_most
                        .key_names
                        .iter()
                        .position(|&a_know_colname| a_know_colname == a_typed_colname)
                    {
                        Some(col_index) => self.key_col_index.push(col_index),
                        None => {
                            panic!("Colname '{}' not found", a_typed_colname);
                        }
                    };
                }
            }
        }
    }
    */
    /************************/
    /*
    pub fn set_vals_cols(&mut self, out_most: &GroupByOutMost, val_names: &Vec<String>) {
        for a_typed_colname in val_names {
            match out_most
                .key_names
                .iter()
                .position(|&a_know_colname| a_know_colname == a_typed_colname)
            {
                Some(col_index) => self.vals_col_index.push(col_index),
                None => {
                    panic!("Colname '{}' not found", a_typed_colname);
                }
            };
        }
    }
    */
    /************************/
    pub fn pick_key_or_col_index(
        &mut self,
        all_defined_col_names: &Vec<&'static str>,
        all_lookup_names: &Vec<String>,
    ) -> Vec<usize> {
        let mut found_col_index: Vec<usize> = Vec::new();
        for a_typed_colname in all_lookup_names {
            let is_numeric = a_typed_colname.parse::<usize>();
            match is_numeric {
                Ok(col_index) => found_col_index.push(col_index),
                Err(_was_not_numeric_but_col_name) => {
                    match //out_most
                  all_defined_col_names
                      .iter()
                      .position(|a_know_colname| a_know_colname == a_typed_colname)
                  {
                      Some(col_index) => found_col_index.push(col_index),
                      None => {
                          panic!("Colname '{}' not found", a_typed_colname);
                      }
                  };
                }
            }
        }
        return found_col_index;
    }
    /************************/
    pub fn set_rows(&mut self, rows: &Vec<Vec<String>>, out_most: &mut GroupByOutMost) {
        for arow in rows {
            let mut key: Vec<String> = Vec::new();
            for i in self.key_col_index.iter() {
                let a_col_value = arow[*i].clone();
                if a_col_value.len() > out_most.col_widths[*i] {
                    out_most.col_widths[*i] = a_col_value.len();
                }
                key.push(a_col_value);
            }
            let mut vals: Vec<String> = Vec::new();
            for i in self.vals_col_index.iter() {
                let a_col_value = arow[*i].clone();
                if a_col_value.len() > out_most.col_widths[*i] {
                    out_most.col_widths[*i] = a_col_value.len();
                }
                vals.push(a_col_value);
            }

            let val = match self.groupby.entry(key) {
                Vacant(entry) => entry.insert(Vec::new()),
                Occupied(entry) => entry.into_mut(),
            };
            val.push(vals);
        }
    }
    /************************/
    pub fn as_tty(&mut self, out_most: &GroupByOutMost) {
        let mut header_cols: Vec<String> = Vec::new();
        let mut footer_cols: Vec<String> = Vec::new();
        for i in self.key_col_index.iter() {
            header_cols.push(format!(
                "{:-<1$}",
                out_most.key_names[*i],
                0 + out_most.col_widths[*i]
            ));
            footer_cols.push(format!("{:-<1$}", "-", 0 + out_most.col_widths[*i]));
        }
        header_cols.push("".to_string());
        footer_cols.push("".to_string());
        for i in self.vals_col_index.iter() {
            header_cols.push(format!(
                "{:-<1$}",
                out_most.key_names[*i],
                0 + out_most.col_widths[*i]
            ));
            footer_cols.push(format!("{:-<1$}", "-", 0 + out_most.col_widths[*i]));
        }
        eprintln!(
            "{}",
            vec!["+".to_string(), header_cols.join("+"), "+".to_string()].concat()
        );
        for (the_grpkey, rows_in_group) in &self.groupby {
            let empty_key_cols: Vec<String> = self
                .key_col_index
                .iter()
                .map(|indx| format!("{: <1$}", " ", 0 + out_most.col_widths[*indx]))
                .collect::<Vec<_>>();

            let mut key_cols: Vec<String> = Vec::new();
            for (i, a_key_col) in the_grpkey.iter().enumerate() {
                key_cols.push(format!(
                    "{: <1$}",
                    a_key_col,
                    0 + out_most.col_widths[self.key_col_index[i]]
                ))
            }
            // we have key_cols       ["aaaaa   ","bb   "];
            // we have empty_key_cols ["        ","     "];

            for (i, aval) in rows_in_group.iter().enumerate() {
                let mut val_cols: Vec<String> = Vec::new();
                for (icol, a_val_col) in aval.iter().enumerate() {
                    val_cols.push(format!(
                        "{: <1$}",
                        a_val_col,
                        0 + out_most.col_widths[self.vals_col_index[icol]]
                    ));
                }
                if i == 0 {
                    eprintln!(
                        "{}",
                        vec![
                            "|".to_string(),
                            key_cols.join("|"),
                            "||".to_string(),
                            val_cols.join("|"),
                            "|".to_string()
                        ]
                        .concat()
                    );
                } else {
                    eprintln!(
                        "{}",
                        vec![
                            "|".to_string(),
                            empty_key_cols.join("|"),
                            "||".to_string(),
                            val_cols.join("|"),
                            "|".to_string()
                        ]
                        .concat()
                    );
                }
            }
        }
        eprintln!(
            "{}",
            vec!["+".to_string(), footer_cols.join("+"), "+".to_string()].concat()
        );
    }
}
/*---------------------------*/
use crate::applic_folder::fsm_folder::fsm::Fsm;
impl Fsm {
    pub fn as_groupby(&mut self, keys: Vec<String>, cols: Vec<String>) -> String {
        let mut the_first_gr = GroupByHier::new();
        let mut out_most = GroupByOutMost {
            key_names: Vec::new(),
            col_widths: Vec::new(),
        };
        for acolname in self.headers.iter() {
            out_most.key_names.push(acolname);
            out_most.col_widths.push(acolname.len());
        }
        let columns = keys[0].split(",");
        let mut vec: Vec<String> = Vec::new();
        for y in columns {
            vec.push(y.to_string());
        }
        let mut found_col_index = the_first_gr.pick_key_or_col_index(&out_most.key_names, &keys);
        the_first_gr.key_col_index = found_col_index;

        found_col_index = the_first_gr.pick_key_or_col_index(&out_most.key_names, &cols);
        the_first_gr.vals_col_index = found_col_index;

        the_first_gr.set_rows(&self.the_normalized_rows, &mut out_most);
        the_first_gr.as_tty(&out_most);
        return "".to_string();
    }
}
