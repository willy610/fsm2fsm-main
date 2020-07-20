//use crate::Fsm;
use crate::applic_folder::fsm_folder::fsm::Fsm;
use std::cmp;
use std::collections::HashMap;
use std::f32;
#[derive(Debug)]
pub struct Permuta {
    size_remainder: usize,
    nr_permuts: usize,
    initial_permut: Vec<usize>, // [0,1,2,...]
}
impl Default for Permuta {
    fn default() -> Permuta {
        Permuta {
            nr_permuts: 0,
            size_remainder: 0,
            initial_permut: Vec::new(),
        }
    }
}
#[derive(Debug)]
pub struct CondResult {
    pub cond: String,
    pub next_state: String,
    pub out_event: String,
    next_state_is_final: bool,
}
#[derive(Debug)]
pub struct Event {
    pub name: String,
    permuta: Permuta,
    pub ordering_conds: Vec<usize>,
    pub all_cond_results: Vec<CondResult>,
}
#[derive(Debug)]
pub struct State {
    pub name: String,
    permuta: Permuta,
    pub ordering_events: Vec<usize>,
    pub all_events: Vec<Event>,
}
#[derive(Debug)]
pub struct Branch {
    name: String,
    permuta: Permuta,
    pub ordering_states: Vec<usize>,
    pub all_states: Vec<State>,
}
impl Default for Branch {
    fn default() -> Branch {
        Branch {
            name: "".to_string(),
            ordering_states: Vec::new(),
            all_states: Vec::new(),
            permuta: Permuta {
                ..Default::default()
            },
        }
    }
}
#[derive(Debug)]
pub struct SharedDims {
    pub x_off: usize,
    pub y_off: isize,
    pub scale_x: f32,
    pub scale_y: f32,
    pub text_of_x: f32,
    pub text_of_y: f32,
    pub max_state_width: usize,
    pub max_inevent_width: usize,
    pub max_cond_and_outmsg_width: usize,
    pub max_cond_or_outmsg_width: usize,
    pub max_cond_width: usize,
    pub max_outmsg_width: usize,
}
#[derive(Debug)]
pub struct Pnt {
    pub x: f32,
    pub y: f32,
}
impl Copy for Pnt {}

impl Clone for Pnt {
    fn clone(&self) -> Pnt {
        *self
    }
}
#[derive(Debug)]
pub struct RowAndNextState {
    row_nr: usize,
    next_state: String,
}
/*============================================*/
struct OptCalc {
    running_row_nr: usize,
    collect_from_to: Vec<RowAndNextState>,
    state_and_row: HashMap<String, usize>,
}
/*============================================*/
#[derive(Debug)]
pub struct GraphRoot<'a> {
    pub the_fsm: &'a Fsm,
    pub the_root: Branch,
    pub shared_dims: SharedDims,
    pub factorial_list: Vec<usize>,
}
/*--------------------------------------------------*/
fn calc_cost(the_opt_calc: OptCalc) -> usize {
    let mut cost = 0;
    for a_row_and_next in the_opt_calc.collect_from_to {
        let to_row_nr = the_opt_calc
            .state_and_row
            .get(&a_row_and_next.next_state)
            .unwrap();
        if to_row_nr > &a_row_and_next.row_nr {
            cost += to_row_nr - &a_row_and_next.row_nr;
        } else {
            cost += &a_row_and_next.row_nr - to_row_nr;
        }
    }
    cost
}
/*============================================*/
fn xxx_event(this: &mut Event, layout_nr: usize, the_opt_calc: &mut OptCalc) {
    let a = this.permuta.nr_permuts;
    let b = this.permuta.size_remainder / a;
    let (high_for_cond, _lowfor_cond_kids) = (layout_nr / b, layout_nr % b);
    let permut_list_cond = nth_permut(high_for_cond, &this.permuta.initial_permut);
    this.ordering_conds = permut_list_cond;
    for i_cond_result in &mut this.ordering_conds {
        let a_cond_result = &mut this.all_cond_results[*i_cond_result];
        // in case of finding optimum
        if !a_cond_result.next_state_is_final {
            the_opt_calc.collect_from_to.push(RowAndNextState {
                row_nr: the_opt_calc.running_row_nr,
                next_state: a_cond_result.next_state.to_string(),
            });
        }
        // one row for each condition
        the_opt_calc.running_row_nr += 1;
    }
}
/*============================================*/
fn xxx_state(this: &mut State, layout_nr: usize, the_opt_calc: &mut OptCalc) {
    let a = this.permuta.nr_permuts;
    let b = this.permuta.size_remainder / a;
    let (high_for_events, mut lowfor_event_kids) = (layout_nr / b, layout_nr % b);
    let permut_list_events = nth_permut(high_for_events, &this.permuta.initial_permut);
    this.ordering_events = permut_list_events;
    for i_event in &mut this.ordering_events {
        let an_event = &mut this.all_events[*i_event];
        let size_of_this_cond = an_event.permuta.size_remainder;
        xxx_event(
            an_event,
            lowfor_event_kids % size_of_this_cond,
            the_opt_calc,
        );
        lowfor_event_kids = lowfor_event_kids / size_of_this_cond;
    }
    the_opt_calc
        .state_and_row
        .insert(this.name.clone(), the_opt_calc.running_row_nr);
    the_opt_calc.running_row_nr += 1;
}
/*============================================*/
fn xxx_root(this: &mut GraphRoot, layout_nr: usize) -> OptCalc {
    let b = this.the_root.permuta.size_remainder;
    let (high_for_states, mut lowfor_state_kids) = (layout_nr / b, layout_nr % b);
    //  let permut_list_states = nth_permut(high_for_states, &this.the_root.permuta.initial_permut);
    let permut_list_states = nth_permut_using_fac_list(
        high_for_states,
        &this.the_root.permuta.initial_permut,
        &this.factorial_list,
    );
    this.the_root.ordering_states = permut_list_states;

    let mut the_opt_calc: OptCalc = OptCalc {
        running_row_nr: 0,
        collect_from_to: Vec::new(),
        state_and_row: HashMap::new(),
    };

    for i_state in &mut this.the_root.ordering_states {
        let a_state = &mut this.the_root.all_states[*i_state];
        let size_of_this_event = a_state.permuta.size_remainder;
        xxx_state(
            a_state,
            lowfor_state_kids % size_of_this_event,
            &mut the_opt_calc,
        );
        lowfor_state_kids = lowfor_state_kids / size_of_this_event;
    }
    the_opt_calc
}
/*----------------------------------------*/
fn create_factorial_once(initial_list: &Vec<usize>) -> Vec<usize> {
    let mut fac_values: Vec<usize> = Vec::new();
    fac_values.push(1); // 0!
    fac_values.push(1); // 1!
    for i in 2..initial_list.len() + 2 {
        let prev = fac_values[i - 1];
        fac_values.push(i * prev);
    }
    fac_values
}
/*----------------------------------------*/
fn nth_permut(n_in: usize, l: &Vec<usize>) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    let mut n = n_in;
    let mut k: usize;
    let mut s: Vec<usize> = l.clone();
    let mut fac_values: Vec<usize> = Vec::new();
    fac_values.push(1); // 0!
    fac_values.push(1); // 1!
    for i in 2..l.len() + 2 {
        let prev = fac_values[i - 1];
        fac_values.push(i * prev);
    }
    if n_in > fac_values[l.len()] {
        eprintln!(
            "nth_permut(n,l); n '{}' out of range when elements are '{:?}'",
            n, l
        );
        l.to_vec()
    } else {
        for i in (0..l.len()).rev() {
            n = n % fac_values[(i + 1)];
            //      k = s[(n / fac_values[i]) + 1 - 1];
            k = s[(n / fac_values[i])];
            // remove k from s
            s.retain(|&x| x != k);
            ret.push(k);
        }
        ret
    }
}
/*----------------------------------------*/
fn nth_permut_using_fac_list(n_in: usize, l: &Vec<usize>, fac_values: &Vec<usize>) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    let mut n = n_in;
    let mut k: usize;
    let mut s: Vec<usize> = l.clone();
    //  let mut fac_values: Vec<usize> = Vec::new();
    //  fac_values.push(1); // 0!
    //  fac_values.push(1); // 1!
    //  for i in 2..l.len() + 2 {
    //    let prev = fac_values[i - 1];
    //    fac_values.push(i * prev);
    //  }
    if n_in > fac_values[l.len()] {
        eprintln!(
            "nth_permut(n,l); n '{}' out of range when elements are '{:?}'",
            n, l
        );
        l.to_vec()
    } else {
        for i in (0..l.len()).rev() {
            n = n % fac_values[(i + 1)];
            //      k = s[(n / fac_values[i]) + 1 - 1];
            k = s[(n / fac_values[i])];
            // remove k from s
            s.retain(|&x| x != k);
            ret.push(k);
        }
        ret
    }
}
impl<'a> GraphRoot<'a> {
    /*----------------------------------------------------*/
    pub fn new(the_fsm: &Fsm) -> GraphRoot {
        GraphRoot {
            the_fsm: the_fsm,
            the_root: Branch {
                ..Default::default()
            },
            shared_dims: SharedDims {
                x_off: 150,
                y_off: 50,
                scale_x: 11.0,
                scale_y: 32.0,
                max_state_width: 0,
                max_inevent_width: 0,
                max_cond_and_outmsg_width: 0,
                max_cond_or_outmsg_width: 0,
                max_cond_width: 0,
                max_outmsg_width: 0,
                text_of_x: 3.0,
                text_of_y: -3.0,
            },
            factorial_list: Vec::new(),
        }
    }
    /*----------------------------------------------------*/
    fn _perm(&self, a: Vec<usize>) -> Vec<Vec<usize>> {
        // in is a vector
        // out is a vector with all permutaions of input
        let mut result: Vec<Vec<usize>> = Vec::new();
        match a.len() {
            1 => result.push(a),
            2 => result.append(&mut vec![a.clone(), vec![a[1], a[0]]]),
            _ => {
                for c in 0..a.len() {
                    // pick one element at pos c (from left to right)
                    let pick_elem = a[c].clone();
                    // collect left side of c
                    // and collect rigth side of c
                    // into one vec 'left_and_right'
                    let left_and_right: Vec<usize> = (0..a.len())
                        .filter(|&i| i != c)
                        .map(|i| a[i].clone())
                        .collect();
                    // permute the left_and_right
                    for mut a_perm_from_left_and_right in self._perm(left_and_right) {
                        // for each permutation of left_and_right
                        // prepend the removed iterm
                        let mut one_final_perm = vec![pick_elem];
                        one_final_perm.append(&mut a_perm_from_left_and_right);
                        // and collect result
                        result.push(one_final_perm); // local collect
                    }
                }
            }
        };
        result
    }
    /*----------------------------------------*/
    pub fn arrow_at_and_dir(&self, at: Pnt, to: Pnt) -> String {
        let mut dirp7p6: Pnt = Pnt {
            x: to.x - at.x,
            y: to.y - at.y,
        };
        let len: f32 = (dirp7p6.x * dirp7p6.x + dirp7p6.y * dirp7p6.y).sqrt();
        dirp7p6.x /= len;
        dirp7p6.y /= len;
        let mut rot_angle: f32 = (dirp7p6.y / dirp7p6.x).atan() * 180.0 / std::f32::consts::PI;
        if dirp7p6.x < 0.0 {
            rot_angle += 180.0;
        }
        //        format!("<path  d='M 0,0 L-17,-9 -12,0 -17,9' transform='translate({},{}) rotate({})' class='ARROW'/>",
        format!("<path  d='M 0,0 L-12,-5 -6,0 -12,5' transform='translate({},{}) rotate({})' class='ARROW'/>", 
    at.x,at.y,rot_angle)
    }
    /*--------------------------------------------------*/
    pub fn find_optimum_better(&mut self) {
        self.setup_basics();
        let _facvalues = create_factorial_once(&self.the_root.permuta.initial_permut);
        //        eprintln!("facvalues={:#?}", facvalues);
        let _iter_to = self.the_root.permuta.size_remainder * self.the_root.permuta.nr_permuts;
        //    eprintln!("_iter_to={}", _iter_to);
        eprintln!("Amount of combinations = {}", _iter_to);
        let mut _min_value_sofar: usize = usize::max_value();
        let mut _min_iter_was = 0;
        for iter in 0.._iter_to {
            if _iter_to > 10 && iter % (_iter_to / 10) == 0 {
                eprintln!("next iter={} of {}", iter, _iter_to);
            }
            let the_opt_calc = xxx_root(self, iter);
            let cost = calc_cost(the_opt_calc);
            if cost < _min_value_sofar {
                _min_value_sofar = cost;
                _min_iter_was = iter;
                eprintln!(
                    "at iter={} new min cost={:?} found",
                    _min_iter_was, _min_value_sofar
                );
            }
        }
        eprintln!("Optimum layout at number '{}'", _min_iter_was);
    }
    /*+++++++++++++++++++++++++++++++++++*/
    pub fn setup_basics(&mut self) {
        self.build_tree();
    }
    /*++++++++++++++++++++++++++++++++++*/
    pub fn use_layout_nr(&mut self, layoutnr: usize) {
        xxx_root(self, layoutnr);
    }
    /*+++++++++++++++++++++++++++++++++++*/
    fn build_tree(&mut self) {
        let mut _the_root_branch: Branch = Branch {
            name: "root".to_string(),
            ordering_states: Vec::new(),
            all_states: Vec::new(),
            permuta: Permuta {
                nr_permuts: 0,
                size_remainder: 0,
                initial_permut: Vec::new(),
            },
        };
        self.the_root = _the_root_branch;
        // State->Events->Cond
        for (state, ref mut events) in self.the_fsm.grouped_fsm.iter() {
            let mut a_state: State = State {
                name: state.to_string(),
                ordering_events: Vec::new(),
                all_events: Vec::new(),
                permuta: Permuta {
                    nr_permuts: 0,
                    size_remainder: 0,
                    initial_permut: Vec::new(),
                    //          use_permut: Vec::new(),
                },
            };
            self.shared_dims.max_state_width =
                cmp::max(self.shared_dims.max_state_width, state.len() * 3 / 5);
            for (event, conditions) in events.iter() {
                let mut an_event: Event = Event {
                    name: event.to_string(),
                    ordering_conds: Vec::new(),
                    all_cond_results: Vec::new(),
                    permuta: Permuta {
                        nr_permuts: 0,
                        size_remainder: 0,
                        initial_permut: Vec::new(),
                        //            use_permut: Vec::new(),
                    },
                };
                self.shared_dims.max_inevent_width =
                    cmp::max(self.shared_dims.max_inevent_width, event.len());
                for (cond, an_outeventandnextstate) in conditions.iter() {
                    let is_final: bool;
                    if an_outeventandnextstate.to_state == self.the_fsm.final_state {
                        is_final = true;
                    } else {
                        is_final = false;
                    }
                    let str_cond: CondResult = CondResult {
                        cond: cond.to_string(),
                        next_state: an_outeventandnextstate.to_state.to_string(),
                        out_event: an_outeventandnextstate.out_msg.to_string(),
                        next_state_is_final: is_final,
                    };
                    self.shared_dims.max_cond_and_outmsg_width = cmp::max(
                        self.shared_dims.max_cond_and_outmsg_width,
                        (cond.len() + 3 + an_outeventandnextstate.out_msg.len()) * 3 / 4,
                    );
                    self.shared_dims.max_cond_width =
                        cmp::max(self.shared_dims.max_cond_width, cond.len());
                    self.shared_dims.max_outmsg_width = cmp::max(
                        self.shared_dims.max_outmsg_width,
                        an_outeventandnextstate.out_msg.len(),
                    );
                    self.shared_dims.max_cond_or_outmsg_width = cmp::max(
                        self.shared_dims.max_cond_or_outmsg_width,
                        cmp::max(
                            self.shared_dims.max_cond_width,
                            self.shared_dims.max_outmsg_width,
                        ),
                    );
                    an_event.all_cond_results.push(str_cond);
                }
                // Events done.
                // default order for cond
                // 1. sort
                an_event
                    .all_cond_results
                    .sort_by(|a, b| a.cond.cmp(&b.cond));
                // 2. initial permut vector
                an_event.permuta.nr_permuts =
                    (1..(1 + an_event.all_cond_results.len())).fold(1, |fac, item| fac * item);
                an_event.permuta.initial_permut = (0..(an_event.all_cond_results.len())).collect();

                // 3. 'size_remainder'
                an_event.permuta.size_remainder = an_event.permuta.nr_permuts; // same
                a_state.all_events.push(an_event);
            }
            // State done
            a_state.all_events.sort_by(|a, b| a.name.cmp(&b.name));
            a_state.permuta.nr_permuts =
                (1..(1 + a_state.all_events.len())).fold(1, |fac, item| fac * item);
            a_state.permuta.initial_permut = (0..(a_state.all_events.len())).collect();

            a_state.permuta.size_remainder = a_state
                .all_events
                .iter()
                .fold(a_state.permuta.nr_permuts, |prod, an_event| {
                    prod * an_event.permuta.size_remainder
                }); // same
            self.the_root.all_states.push(a_state);
        }
        self.the_root.all_states.sort_by(|a, b| a.name.cmp(&b.name));
        self.the_root.permuta.initial_permut = (0..(self.the_root.all_states.len())).collect();
        self.factorial_list = create_factorial_once(&self.the_root.permuta.initial_permut);
        self.the_root.permuta.nr_permuts =
            (1..(1 + self.the_root.all_states.len())).fold(1, |fac, item| fac * item);

        self.the_root.permuta.size_remainder = self
            .the_root
            .all_states
            .iter()
            .fold(1, |now, next| now * next.permuta.size_remainder);
    }
}
