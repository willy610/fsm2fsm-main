use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::BTreeMap;
use std::collections::HashSet;

extern crate rand;

#[derive(Debug)]
pub struct OutEventAndNextStateAndUserCondValue {
    pub out_msg: String,
    pub to_state: String,
    pub user_guard_result: String,
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct TransRow {
    from_state: String,
    in_msg: String,
    guard_result: String,
    out_msg: String,
    to_state: String,
}
type StatesEventsAndCondAndOutAndNext =
    BTreeMap<String, BTreeMap<String, BTreeMap<String, OutEventAndNextStateAndUserCondValue>>>;

#[derive(Debug)]
pub struct Fsm {
    pub name: String,
    pub initial_state: String,
    pub final_state: String,
    state_now: String,
    pub grouped_fsm: StatesEventsAndCondAndOutAndNext,
    pub the_rows_5_cols: Vec<Vec<String>>,
    pub the_normalized_rows: Vec<Vec<String>>,
    pub headers: Vec<&'static str>,
}
impl Fsm {
    /*----------------------------------------------------*/
    pub fn new(name: String) -> Fsm {
        Fsm {
            name: name,
            initial_state: String::from("novalue"),
            final_state: String::from("novalue"),
            state_now: String::from("novalue"),
            grouped_fsm: BTreeMap::new(),
            the_rows_5_cols: Vec::new(),
            the_normalized_rows: Vec::new(),
            headers: vec![
                "State",
                "InMessage",
                "UserGuardValue",
                "OutMessage",
                "NextState",
                "SynteticGuardValue",
            ],
        }
    }
    /*----------------------------------------------------*/
    pub fn set_grouped_fsm(&mut self, par: StatesEventsAndCondAndOutAndNext) {
        self.grouped_fsm = par;
    }
    /*----------------------------------------------------*/
    pub fn set_rows(&mut self, the_rows: Vec<Vec<String>>) {
        self.the_rows_5_cols = Vec::new();
        self.the_normalized_rows = Vec::new();
        for arow in &the_rows {
            self.the_rows_5_cols.push(arow.to_vec());
        }
        for mut arow in the_rows {
            arow.push("synt_gurad_result".to_string());
            self.the_normalized_rows.push(arow);
        }
        self.normalize_cond();
        self.set_grouped_fsm(self.group_it());
        //        self.the_rows_5_cols = the_rows;
    }
    /*----------------------------------------------------*/
    fn normalize_cond(&mut self) {
        //        let mut as_normalized_rows: Vec<Vec<String>> = Vec::new();
        let temp = self.group_it();
        self.the_normalized_rows = Vec::new();

        for (state, inmsgs) in temp.iter() {
            for (inmsg, gardvalues) in inmsgs.iter() {
                for (i, (user_guard_result, an_outeve_and_next_state)) in
                    gardvalues.iter().enumerate()
                {
                    let synt_guard_result = format!("{state}_{inmsg}_{enum}", 
            state= state.to_lowercase(),
            inmsg= inmsg.to_lowercase(),
            enum= i+1);
                    let xxx: Vec<String> = vec![
                        state.clone(),
                        inmsg.clone(),
                        user_guard_result.clone(),
                        an_outeve_and_next_state.out_msg.clone(),
                        an_outeve_and_next_state.to_state.clone(),
                        synt_guard_result,
                    ];
                    self.the_normalized_rows.push(xxx.clone());
                }
            }
        }
        //        self.set_rows(as_normalized_rows);
    }
    /*----------------------------------------------------*/
    pub fn check_details(&mut self, the_outer_rows: &Vec<Vec<String>>) -> bool {
        // 1. ensure that state+event produces distinct ouputs
        // 2. find one initial state
        // 3. find one final state

        for (a_state_key, a_state_key_value) in self.grouped_fsm.iter() {
            for (in_event, result_rows) in a_state_key_value.iter() {
                // collect  'Output' from result_rows
                //  and ensure no duplicates. (Otherwise unbiguity at invers at least)
                let nr_res = result_rows.len();
                if nr_res > 1 {
                    let mut all_output: HashSet<String> = HashSet::new();
                    for (_, outevent_and_nextstate) in result_rows {
                        // insert the Output
                        all_output.insert(outevent_and_nextstate.out_msg.clone());
                    }
                    if nr_res != all_output.len() {
                        eprintln!(
                        "In state {:?} and event {:?} some of the Output are the same.\n The FSM not invertable.",
                        a_state_key, in_event);
                    }
                }
            }
        }
        // 2.
        let mut all_from_states: HashSet<String> = HashSet::new();
        let mut all_to_states: HashSet<String> = HashSet::new();
        for atrans in the_outer_rows {
            all_from_states.insert(atrans[0].clone());
            all_to_states.insert(atrans[4].clone());
        }
        let _init_states = &all_from_states - &all_to_states; // set diff
        let _final_states = &all_to_states - &all_from_states; // set diff
        if _init_states.len() != 1 {
            println!(
                "Exact one state should be an initial state.\n Content init_states={:#?}",
                _init_states
            );
            return false;
        }
        if _final_states.len() != 1 {
            println!(
                "Exact one state should be a final state.\n Content final_states={:#?}",
                _final_states
            );
            return false;
        }
        for the_member in _init_states {
            self.initial_state = the_member; // remember
            self.state_now = self.initial_state.clone();
        }
        for the_member in _final_states {
            self.final_state = the_member; // remember
        }
        //        eprintln!("self.final_state={:?}",self.final_state);
        true
    }
    /*----------------------------------------------------*/
    pub fn get_state_now(&mut self) -> String {
        return self.state_now.clone();
    }
    /*----------------------------------------------------*/
    pub fn get_final_state(&mut self) -> String {
        return self.final_state.clone();
    }
    /*----------------------------------------------------*/
    pub fn mirror_direct(&self, the_outer_rows: &Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut mirror_res: HashSet<TransRow> = HashSet::new();
        for trans in the_outer_rows {
            let (from_state, in_msg, out_msg, next_state) =
                (&trans[0], &trans[1], &trans[3], &trans[4]);
            if *from_state == self.initial_state {
                mirror_res.insert(TransRow {
                    // Generate the transition for INITIAL state
                    from_state: self.initial_state.clone(),
                    in_msg: String::from("Callin"),
                    guard_result: String::from("nooutput"),
                    out_msg: in_msg.to_string(),
                    to_state: format!("{}_{}", self.initial_state, in_msg),
                });
            }
            if *next_state == self.final_state {
                // Generate the transition for FINAL state
                mirror_res.insert(TransRow {
                    from_state: format!("{}_{}", from_state, in_msg),
                    in_msg: out_msg.to_string(),
                    guard_result: String::from("COND_OK"),
                    out_msg: String::from("nooutput"),
                    to_state: self.final_state.clone(),
                });
            }
        }
        for first_trans in the_outer_rows {
            let (first_state, first_event, first_out_event, first_next_state) = (
                &first_trans[0],
                &first_trans[1],
                &first_trans[3],
                &first_trans[4],
            );
            for second_trans in the_outer_rows {
                let (second_state, second_event) = (&second_trans[0], &second_trans[1]);
                if second_state == first_next_state {
                    mirror_res.insert(TransRow {
                        from_state: format!("{}_{}", first_state, first_event),
                        in_msg: first_out_event.clone(),
                        guard_result: String::from("nooutput"),
                        out_msg: second_event.to_string(),
                        to_state: format!("{}_{}", second_state, second_event),
                    });
                }
            }
        }
        let mut as_rows: Vec<Vec<String>> = Vec::new();

        let mut guard_enum = 1;
        for a_trans in &mirror_res {
            let guard_result = format!("{state}_{event}_{enum}",
                  state=a_trans.from_state.clone(),
                  event=a_trans.in_msg.clone(),
                  enum=guard_enum);
            let enumed_trans: Vec<String> = vec![
                a_trans.from_state.clone(),
                a_trans.in_msg.clone(),
                guard_result,
                a_trans.out_msg.clone(),
                a_trans.to_state.clone(),
            ];
            as_rows.push(enumed_trans.clone());
            guard_enum += 1;
        }
        as_rows
    }
    /*----------------------------------------------------*/
    fn group_it(&self) -> StatesEventsAndCondAndOutAndNext {
        let mut the_group: StatesEventsAndCondAndOutAndNext = BTreeMap::new();
        // Build State->Event * ->Cond * ->(Out,Nextstate)
        //        eprintln!("{:#?}",self.the_normalized_rows);
        for a_trans in &self.the_normalized_rows {
            let from_state = &a_trans[0];
            let in_event = &a_trans[1];
            let guard_result = &a_trans[2];
            let out_msg = &a_trans[3];
            let to_state = &a_trans[4];
            let user_guard_result = &a_trans[5];
            // https://blog.inliniac.net/2017/05/19/learning-rust-hash-map-lookupinsert-pattern/
            // https://stackoverflow.com/questions/30851464/want-to-add-to-hashmap-using-pattern-match-get-borrow-mutable-more-than-once-at

            // seen this state before ?
            the_group
                .entry(from_state.to_string())
                //      seen this event in this state before ?
                .or_insert(BTreeMap::new()); // Will contain all events
            the_group
                .get_mut(from_state)
                .unwrap()
                .entry(in_event.to_string())
                // have not seen this event earlier. Insert a collection for descisions
                // seen this guard result for this event in this state before ? THEN ITS WRONG
                //                .or_insert(HashMap::new()); // Will contain all guard_results
                .or_insert(BTreeMap::new()); // Will contain all guard_results

            match the_group
                .get_mut(from_state)
                .unwrap()
                .get_mut(in_event)
                .unwrap()
                .entry(guard_result.to_string())
            {
                Vacant(entry) => {
                    entry.insert(OutEventAndNextStateAndUserCondValue {
                        out_msg: out_msg.to_string(),
                        to_state: to_state.to_string(),
                        user_guard_result: user_guard_result.to_string(),
                    });
                }
                Occupied(_entry) => panic!(format!(
                    "Duplicate cond '{}' in trans={},{}",
                    guard_result, from_state, in_event
                )),
            }
        }
        the_group
    }
}
