use crate::applic_folder::fsm_folder::fsm::Fsm;
impl Fsm {
    /*---------------- print as_csv ------------------------------*/
    pub fn as_csv(&mut self) {
        println!("{}", self.headers.join(","));
        for a_row in self.the_normalized_rows.iter() {
            println!(
                "{}",
                format!(
                    "{},{},{},{},{},{}",
                    a_row[0], a_row[1], a_row[2], a_row[5], a_row[3], a_row[4]
                )
            );
        }
    }
}
