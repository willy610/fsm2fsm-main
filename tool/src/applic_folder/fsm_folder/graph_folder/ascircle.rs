use super::graphroot::CondResult;
use super::graphroot::Event;
use super::graphroot::GraphRoot;
use super::graphroot::Pnt;
use super::graphroot::State;
use std::collections::HashMap;

#[derive(Debug)]
struct Circdims {
    modell_center_x_off_px: f32,
    modell_center_y_off_px: f32,
    scale_x: f32,
    scale_y: f32,
    text_of_x_event: f32,
    text_of_y_event: f32,
    text_of_x_cond: f32,
    text_of_y_cond: f32,
    state_radie_px: f32,
    radie_all_states_px: f32,
    len_event_text_px: f32,
    len_cond_or_outmsg_text_px: f32,
    radie_all_cond_px: f32,
    radie_all_back_next_state_px: f32,
    soft_corner_radie: f32,
    text_dims_state_px: Pnt,
    text_dims_event_px: Pnt,
    text_dims_cond_or_outmsg_px: Pnt,
}
struct FromCond2NextState {
    from_sector_angle: f32,
    to_next_state: String,
}
struct StateEndings {
    //  state_name: String,
    radie_index_off: usize,
    ends_at_outer_sector_angle: f32,
    //  ends_at_inner_sector_angle: f32,
}
struct SetupStyles {
    state_font_size: usize,
    event_font_size: usize,
    cond_font_size: usize,
    width_and_heigth: Pnt,
}
#[derive(Debug)]
struct Nyy {
    dv_cond: f32,
    big_r_start_cond: f32,
    big_r_end_cond: f32,
    small_r_4_states: f32,
}
fn dir_from_2_points(p_start: Pnt, p_end: Pnt) -> Pnt {
    let len: f32 = ((p_end.x - p_start.x) * (p_end.x - p_start.x)
        + (p_end.y - p_start.y) * (p_end.y - p_start.y))
        .sqrt();
    Pnt {
        x: (p_end.x - p_start.x) / len,
        y: (p_end.y - p_start.y) / len,
    }
}
impl<'a> GraphRoot<'a> {
    pub fn ascircle(&mut self, layoutnr: usize, sector_size: usize) -> String {
        //      self.
        self.setup_basics();
        self.use_layout_nr(layoutnr);
        let mut put_gen_content_here: Vec<String> = Vec::new();
        let canvas_dims = self.build_circle(&mut put_gen_content_here, sector_size);
        let mut putinitrowshere: Vec<String> = Vec::new();
        self.gen_intro(&mut putinitrowshere, canvas_dims);
        putinitrowshere.append(&mut put_gen_content_here);
        self.gen_finish_circle(&mut putinitrowshere)
    }
    /*##########################################*/
    fn aaa_state(
        &self,
        the_state: &State,
        nyy: &Nyy,
        start_v: f32,
        circ_dims: &Circdims,
        origo: Pnt,
        put_gen_content_here: &mut Vec<String>,
        cond_to_next: &mut Vec<FromCond2NextState>,
        states_coll: &mut HashMap<String, StateEndings>,
        state_index: usize,
    ) -> f32 {
        let tot_nr_conds: usize = 0 + the_state.all_events.iter().fold(0, |n, an_event| {
            an_event.all_cond_results.iter().fold(0, |m, _| m + 1) + n
        });
        let d_v_half = (tot_nr_conds - 1) as f32 * nyy.dv_cond / 2.0;
        let end_v = start_v + (1 + tot_nr_conds) as f32 * nyy.dv_cond;

        let state_xy_px: Pnt = Pnt {
            x: origo.x + nyy.small_r_4_states * (start_v + d_v_half).cos(),
            y: origo.y + nyy.small_r_4_states * (start_v + d_v_half).sin(),
        };
        // final connect into the state
        if !(self.the_fsm.final_state == the_state.name
            || self.the_fsm.initial_state == the_state.name)
        {
            let ending = StateEndings {
                radie_index_off: state_index + 1,
                //        ends_at_outer_sector_angle: start_v - nyy.dv_cond,
                ends_at_outer_sector_angle: end_v - nyy.dv_cond,
            };
            {
                let big_r = nyy.big_r_end_cond + (ending.radie_index_off * 20) as f32;
                let end_round_off = big_r
                    - ((big_r - circ_dims.soft_corner_radie)
                        * (big_r - circ_dims.soft_corner_radie)
                        + circ_dims.soft_corner_radie * circ_dims.soft_corner_radie)
                        .sqrt();
                let p1: Pnt = Pnt {
                    x: (nyy.big_r_end_cond - end_round_off + (ending.radie_index_off * 20) as f32)
                        * ending.ends_at_outer_sector_angle.cos(),
                    y: (nyy.big_r_end_cond - end_round_off + (ending.radie_index_off * 20) as f32)
                        * ending.ends_at_outer_sector_angle.sin(),
                };
                let p2: Pnt = Pnt {
                    x: circ_dims.radie_all_states_px * ending.ends_at_outer_sector_angle.cos(),
                    y: circ_dims.radie_all_states_px * ending.ends_at_outer_sector_angle.sin(),
                };
                put_gen_content_here.push(format!(
                    "<line class='LINERESULT_CIRC' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}'></line>",
                    p1.x, p1.y, p2.x, p2.y
                ));
                let arr_1: String = self.arrow_at_and_dir(
                    Pnt {
                        x: p1.x + (p2.x - p1.x) / 3.0,
                        y: p1.y + (p2.y - p1.y) / 3.0,
                    },
                    Pnt { x: p2.x, y: p2.y },
                );
                put_gen_content_here.push(arr_1);
                put_gen_content_here.push(self.arc_to_svg(
                    origo.x,
                    origo.y,
                    nyy.small_r_4_states,
                    ending.ends_at_outer_sector_angle,
                    start_v + d_v_half, // center state circle
                    false,
                ));
                states_coll.insert(the_state.name.clone(), ending);
            }
        }
        // the state circle
        put_gen_content_here.push(format!(
            "<circle class='STATE_CIRC' cx='{:?}' cy='{:?}' r='{}'>{}</circle>",
            state_xy_px.x, state_xy_px.y, circ_dims.state_radie_px, the_state.name
        ));
        // the state name
        put_gen_content_here.push(format!(
            "<text class='STATETEXT_CIRC' x='{:?}' y='{:?}'>{}</text>",
            state_xy_px.x,
            state_xy_px.y + circ_dims.text_of_y_event,
            the_state.name
        ));
        //
        // all events
        self.aaa_all_events_in_a_state(
            the_state,
            nyy,
            start_v,
            state_xy_px,
            circ_dims,
            origo,
            put_gen_content_here,
            cond_to_next,
        );
        end_v
    }
    /*##########################################*/
    fn aaa_all_events_in_a_state(
        &self,
        the_state: &State,
        nyy: &Nyy,
        start_v: f32,
        state_model: Pnt,
        circ_dims: &Circdims,
        origo: Pnt,
        put_gen_content_here: &mut Vec<String>,
        cond_to_next: &mut Vec<FromCond2NextState>,
    ) {
        let mut cond_start_end_v: Vec<(f32, f32)> = Vec::new();
        let mut start = start_v;
        for event_nr in 0..the_state.ordering_events.len() {
            let event_index = &the_state.ordering_events[event_nr];
            let event = &the_state.all_events[*event_index];
            let end = self.aaa_all_conds_in_an_event(
                event,
                nyy,
                start,
                circ_dims,
                origo,
                put_gen_content_here,
                cond_to_next,
            );
            cond_start_end_v.push((start, end - nyy.dv_cond));
            start = end;
        }
        // draw each event into middle of conds
        for event_nr in 0..the_state.ordering_events.len() {
            let event_index = &the_state.ordering_events[event_nr];
            let event = &the_state.all_events[*event_index];
            let (fromv, tov) = cond_start_end_v[event_nr];
            // start for line
            // end for line
            // middle for text
            let start_p = Pnt {
                x: state_model.x + circ_dims.state_radie_px * ((fromv + tov) / 2.0).cos(),
                y: state_model.y + circ_dims.state_radie_px * ((fromv + tov) / 2.0).sin(),
            };
            let end_p = Pnt {
                x: nyy.big_r_start_cond * ((fromv + tov) / 2.0).cos(),
                y: nyy.big_r_start_cond * ((fromv + tov) / 2.0).sin(),
            };
            let anchor_p = Pnt {
                x: (end_p.x + start_p.x) / 2.0,
                y: (end_p.y + start_p.y) / 2.0,
            };

            // the event line
            put_gen_content_here.push(format!(
                "<line class='LINEEVENT_CIRC' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}'></line>",
                start_p.x, start_p.y, end_p.x, end_p.y
            ));

            let the_dir = dir_from_2_points(start_p, end_p);
            let mut rot_angle: f32 = (the_dir.y / the_dir.x).atan() * 180.0 / std::f32::consts::PI;
            if the_dir.x < 0.0 {
                rot_angle += 180.0;
            }
            // the event text
            if rot_angle >= 270.0 || rot_angle <= 90.0 {
                put_gen_content_here.push(format!(
          "<text class='EVENTTEXT_CIRC' x='{:?}' y='{:?}' transform='translate({},{}) rotate({})'>{}</text>",
          circ_dims.text_of_x_event,
          circ_dims.text_of_y_event,
          anchor_p.x ,
          anchor_p.y ,
          rot_angle,
          event.name
        ));
            } else {
                put_gen_content_here.push(format!(
          "<text class='EVENTTEXT_CIRC_ROT' x='{:?}' y='{:?}' transform='translate({},{}) rotate({})'>{}</text>",
          -circ_dims.text_of_x_event,
          circ_dims.text_of_y_event,
          anchor_p.x ,
          anchor_p.y ,
          rot_angle + 180.0,
          event.name
        ));
            }
            // arrow  at end of event
            put_gen_content_here.push(self.arrow_at_and_dir(
                end_p,
                Pnt {
                    x: end_p.x + (end_p.x - start_p.x),
                    y: end_p.y + (end_p.y - start_p.y),
                },
            ));
        }
    }
    /*##########################################*/
    fn aaa_all_conds_in_an_event(
        &self,
        the_event: &Event,
        nyy: &Nyy,
        start_v: f32,
        circ_dims: &Circdims,
        origo: Pnt,
        put_gen_content_here: &mut Vec<String>,
        cond_to_next: &mut Vec<FromCond2NextState>,
    ) -> f32 {
        let mut at_v = start_v;
        for cond_nr in 0..the_event.ordering_conds.len() {
            let cond_index = &the_event.ordering_conds[cond_nr];
            let cond = &the_event.all_cond_results[*cond_index];
            let x = self.aaa_draw_one_cond(cond, nyy, at_v, circ_dims, origo, put_gen_content_here);
            cond_to_next.push(x);
            at_v += nyy.dv_cond
        }
        // connect all cond in this event with an arc
        put_gen_content_here.push(self.arc_to_svg(
            origo.x,
            origo.y,
            nyy.big_r_start_cond,
            start_v - 0.05,
            at_v + 0.05 - nyy.dv_cond,
            false,
        ));
        at_v
    }
    /*##########################################*/
    fn aaa_draw_one_cond(
        &self,
        the_cond: &CondResult,
        nyy: &Nyy,
        at_v: f32,
        circ_dims: &Circdims,
        origo: Pnt,
        put_gen_content_here: &mut Vec<String>,
    ) -> FromCond2NextState {
        let cond_start_xy_model =
            self.add_mdl_pnt_and_len_and_dir(&origo, nyy.big_r_start_cond, at_v);
        let cond_end_xy_model = self.add_mdl_pnt_and_len_and_dir(&origo, nyy.big_r_end_cond, at_v);

        put_gen_content_here.push(format!(
            "<line class='LINERESULT_CIRC' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}'></line>",
            cond_start_xy_model.x, cond_start_xy_model.y, cond_end_xy_model.x, cond_end_xy_model.y
        ));
        let the_dir = dir_from_2_points(cond_start_xy_model, cond_end_xy_model);
        let mut rot_angle: f32 = (the_dir.y / the_dir.x).atan() * 180.0 / std::f32::consts::PI;
        //    let the_turner: Turner;
        //    if v_cond >= 3.0 / 2.0 * std::f32::consts::PI || v_cond <= std::f32::consts::PI / 2.0 {
        if the_dir.x < 0.0 {
            rot_angle += 180.0;
        }
        if rot_angle >= 270.0 || rot_angle <= 90.0 {
            put_gen_content_here.push(format!(
          "<text class='RESULTTEXT_CIRC' x='{:?}' y='{:?}' transform='translate({},{}) rotate({})'>{}</text>",
          circ_dims.text_of_x_cond,
          circ_dims.text_of_y_cond,
          cond_start_xy_model.x ,
          cond_start_xy_model.y ,
          rot_angle,
          the_cond.cond
        ));
        } else {
            put_gen_content_here.push(format!(
          "<text class='RESULTTEXT_CIRC_ROT' x='{:?}' y='{:?}' transform='translate({},{}) rotate({})'>{}</text>",
          -circ_dims.text_of_x_cond,
          circ_dims.text_of_y_cond,
          cond_start_xy_model.x ,
          cond_start_xy_model.y ,
          rot_angle + 180.0,
          the_cond.cond
        ));
        }
        FromCond2NextState {
            from_sector_angle: at_v,
            to_next_state: the_cond.next_state.clone(),
        }
    }
    /*##########################################*/
    fn aaa_connect_all_conds_to_next_or_final(
        &self,
        cond_to_next: &Vec<FromCond2NextState>,
        circ_dims: &Circdims,
        nyy: &Nyy,
        put_gen_content_here: &mut Vec<String>,
        states_coll: &HashMap<String, StateEndings>,
        //    origo: Pnt,
        //    origo_4_arc: Pnt,
    ) {
        for back in cond_to_next {
            if self.the_fsm.final_state == back.to_next_state {
                let start_angle = back.from_sector_angle;
                let rot_point = Pnt {
                    x: nyy.big_r_end_cond * start_angle.cos(),
                    y: nyy.big_r_end_cond * start_angle.sin(),
                };
                for i in vec![0, 1, 2].iter() {
                    let dx: f32 = *i as f32 * 5.0;
                    let dy: f32 = (*i - 3) as f32 * 5.0;
                    put_gen_content_here.push(format!(
            "<line class='LINEEARTH_CIRC' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}' transform='translate({},{}) rotate({})'></line>",
            dx,
            - dy,
            dx,
            dy,
rot_point.x,rot_point.y,
            start_angle * 180.0 / std::f32::consts::PI
          ));
                }
            } else {
                let start_angle = back.from_sector_angle;
                let origo: Pnt = Pnt { x: 0.0, y: 0.0 };
                //        let (end_angle, off) = states_coll.get(&back.to_next_state).unwrap();
                let ending = states_coll.get(&back.to_next_state).unwrap();
                let x = self.arc(
                    origo.x,
                    origo.y,
                    nyy.big_r_end_cond + (ending.radie_index_off * 20) as f32,
                    start_angle,
                    ending.ends_at_outer_sector_angle, // - dv * 1.0,
                    true,
                );
                put_gen_content_here.push(x);
                let radie = nyy.big_r_end_cond;
                let p1 = self.add_mdl_pnt_and_len_and_dir(&origo, radie, start_angle);
                let end_round_off = radie
                    - ((radie - circ_dims.soft_corner_radie)
                        * (radie - circ_dims.soft_corner_radie)
                        + circ_dims.soft_corner_radie * circ_dims.soft_corner_radie)
                        .sqrt();
                let p2: Pnt = Pnt {
                    x: (radie - end_round_off + (ending.radie_index_off * 20) as f32)
                        * start_angle.cos(),
                    y: (radie - end_round_off + (ending.radie_index_off * 20) as f32)
                        * start_angle.sin(),
                };
                put_gen_content_here.push(format!(
                    "<line class='LINERESULT_CIRC' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}'></line>",
                    p1.x, p1.y, p2.x, p2.y
                ));
            }
        }
    }

    /*##########################################*/
    /*##########################################*/
    /*##########################################*/
    fn build_circle(
        &self,
        put_gen_content_here: &mut Vec<String>,
        sector_size: usize,
    ) -> SetupStyles {
        let sector_procent: f32 = sector_size as f32;
        let mut style = SetupStyles {
            state_font_size: 16,
            event_font_size: 14,
            cond_font_size: 14,
            width_and_heigth: Pnt { x: 0.0, y: 0.0 },
        };
        let mut circ_dims: Circdims = Circdims {
            modell_center_x_off_px: 0.0,
            modell_center_y_off_px: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            text_of_x_event: 4.0,
            text_of_y_event: -5.0,
            text_of_x_cond: 4.0,
            text_of_y_cond: -6.0,
            text_dims_state_px: Pnt {
                x: style.state_font_size as f32,
                y: style.state_font_size as f32,
            },
            text_dims_event_px: Pnt {
                x: style.event_font_size as f32,
                y: style.event_font_size as f32,
            },
            text_dims_cond_or_outmsg_px: Pnt {
                x: style.cond_font_size as f32,
                y: style.cond_font_size as f32,
            },
            soft_corner_radie: 20.0,
            state_radie_px: 0.0,
            radie_all_states_px: 0.0,
            len_event_text_px: 0.0,
            len_cond_or_outmsg_text_px: 0.0,
            //      len_outmsg_px: 0.0,
            radie_all_cond_px: 0.0,
            radie_all_back_next_state_px: 0.0,
        };
        // class STATETEXT_CIRC holds font-size:16px;
        // class EVENTTEXT_CIRC holds font-size:16px; and _ROT
        // class RESULTTEXT_CIRC holds font-size:14px; and _ROT

        let tot_nr_conds = self.calc_tot_nr_conds();
        //    let tot_nr_events = self.calc_tot_nr_events();
        let tot_nr_states = self.the_root.ordering_states.len();
        let mut nyy: Nyy = Nyy {
            dv_cond: 0.0,
            big_r_start_cond: 0.0,
            big_r_end_cond: 0.0,
            small_r_4_states: 0.0,
        };

        circ_dims.text_dims_state_px.x *= self.shared_dims.max_state_width as f32 * 0.32;
        circ_dims.text_dims_event_px.x *= self.shared_dims.max_inevent_width as f32 * 0.32;
        circ_dims.text_dims_cond_or_outmsg_px.x *=
            self.shared_dims.max_cond_or_outmsg_width as f32 * 0.60;

        let min_state_arc_len: f32 = 100.0;
        let min_outmost_arc_len: f32 = 50.0;
        nyy.dv_cond = 2.0 * std::f32::consts::PI / ((tot_nr_conds + tot_nr_states) as f32);
        //        nyy.dv_cond /= 2.0;
        //nyy.dv_cond /= 1.1;
        nyy.dv_cond *= sector_procent / 100.0;
        {
            let mut small_r =
                tot_nr_states as f32 * min_state_arc_len / (2.0 * std::f32::consts::PI);
            let maxlen_condandoutmsg = circ_dims.text_dims_cond_or_outmsg_px.x;
            let maxlen_event = circ_dims.text_dims_event_px.x;
            let arc_len = (2.0
                * std::f32::consts::PI
                * (maxlen_condandoutmsg + maxlen_event + small_r) as f32)
                / ((tot_nr_conds + tot_nr_states) as f32);
            let big_r: f32;
            if arc_len < min_outmost_arc_len {
                big_r = (min_outmost_arc_len + (tot_nr_conds + tot_nr_states) as f32)
                    / (2.0 * std::f32::consts::PI);
                small_r = big_r - (maxlen_condandoutmsg + maxlen_event) as f32;
            } else {
                big_r = small_r + maxlen_condandoutmsg + maxlen_event;
            }
            nyy.big_r_start_cond = big_r;
            nyy.big_r_end_cond = nyy.big_r_start_cond + maxlen_condandoutmsg;
            nyy.small_r_4_states = small_r;
        }

        //    eprintln!("nyy={:#?}", nyy);
        circ_dims.state_radie_px = 20.0; // fixed

        circ_dims.radie_all_states_px = circ_dims.state_radie_px * 1.0;
        circ_dims.radie_all_states_px = self.the_root.ordering_states.len() as f32
            * (2.5 * circ_dims.radie_all_states_px + 0.0)
            / std::f32::consts::PI;
        circ_dims.radie_all_cond_px = circ_dims.radie_all_states_px + circ_dims.len_event_text_px;
        circ_dims.len_cond_or_outmsg_text_px = self.shared_dims.max_cond_or_outmsg_width as f32;
        circ_dims.radie_all_back_next_state_px =
            circ_dims.radie_all_cond_px + circ_dims.len_cond_or_outmsg_text_px;

        //    eprintln!(
        //      "circ_dims.radie_all_back_next_state_px={:#?}",
        //     circ_dims.radie_all_back_next_state_px
        //    );
        circ_dims.modell_center_x_off_px = 30.0 + nyy.big_r_end_cond;
        circ_dims.modell_center_y_off_px = 30.0 + nyy.big_r_end_cond;

        let mut cond_to_next: Vec<FromCond2NextState> = Vec::new();
        let mut states_coll: HashMap<String, StateEndings> = HashMap::new();
        let origo: Pnt = Pnt { x: 0.0, y: 0.0 };

        let mut _nytt_v_at: f32 = 0.0;
        _nytt_v_at = nyy.dv_cond + 1.5 * std::f32::consts::PI;
        for state_nr in 0..self.the_root.ordering_states.len() {
            let state_index = &self.the_root.ordering_states[state_nr];
            let _state = &self.the_root.all_states[*state_index];
            _nytt_v_at = self.aaa_state(
                _state,
                &nyy,
                _nytt_v_at,
                &circ_dims,
                origo,
                put_gen_content_here,
                &mut cond_to_next,
                &mut states_coll,
                *state_index,
            );
        }
        // connect all conds to next state or final state
        self.aaa_connect_all_conds_to_next_or_final(
            &cond_to_next,
            &circ_dims,
            &nyy,
            put_gen_content_here,
            &states_coll,
            //      origo,
            //      origo,
        );
        style.width_and_heigth = Pnt {
            //      x: 3.0 * circ_dims.modell_center_x_off_px,
            //      y: 3.0 * circ_dims.modell_center_x_off_px,
            x: 2.0 * (nyy.big_r_end_cond + tot_nr_states as f32 * 20.0),
            y: 2.0 * (nyy.big_r_end_cond + tot_nr_states as f32 * 20.0),
        };
        style
    }
    /*---------------------------------------------*/
    fn calc_tot_nr_conds(&self) -> usize {
        let mut tot_nr_conds: usize = 0;
        for state_nr in 0..self.the_root.ordering_states.len() {
            let state_index = &self.the_root.ordering_states[state_nr];
            let _state = &self.the_root.all_states[*state_index];
            let tot_nr_conds_state: usize = _state.all_events.iter().fold(0, |n, an_event| {
                an_event.all_cond_results.iter().fold(0, |m, _| m + 1) + n
            });
            tot_nr_conds += tot_nr_conds_state;
        }
        tot_nr_conds
    }
    /*---------------------------------------------*/
    fn _calc_tot_nr_events(&self) -> usize {
        let mut tot_nr_events: usize = 0;
        for state_nr in 0..self.the_root.ordering_states.len() {
            let state_index = &self.the_root.ordering_states[state_nr];
            let _state = &self.the_root.all_states[*state_index];
            tot_nr_events += _state.all_events.len();
        }
        tot_nr_events
    }
    /*---------------------------------------------*/
    fn add_mdl_pnt_and_len_and_dir(&self, p1: &Pnt, len: f32, dir: f32) -> Pnt {
        Pnt {
            x: p1.x + len * dir.cos(),
            y: p1.y + len * dir.sin(),
        }
    }
    /*---------------------------------------------*/
    fn arc_to_svg(
        &self,
        center_x: f32,
        center_y: f32,
        radie: f32,
        start_angle: f32,
        end_angle: f32,
        with_arrows: bool,
    ) -> String {
        let large: usize;
        let sweep: usize;
        let small_v: f32;
        let big_v: f32;
        let arrow_dir: f32;
        if end_angle < start_angle {
            small_v = end_angle;
            big_v = start_angle;
            arrow_dir = -1.0;
        } else {
            small_v = start_angle;
            big_v = end_angle;
            arrow_dir = -1.0;
        };
        if (big_v - small_v) > std::f32::consts::PI {
            large = 1;
            sweep = 1;
        } else {
            large = 0;
            sweep = 1;
        };
        let svg_start = Pnt {
            x: center_x + radie * small_v.cos(),
            y: center_y + radie * small_v.sin(),
        };
        let svg_end: Pnt = Pnt {
            x: center_x + radie * big_v.cos(),
            y: center_y + radie * big_v.sin(),
        };

        let resut = format!(
      "<path d='M {},{} A{},{} 0 {} {} {},{}' style='fill:none; stroke:black; stroke-width=\"2\"'/>",
      svg_start.x, svg_start.y, radie, radie, large, sweep,svg_end.x, svg_end.y
    );
        if with_arrows {
            // to is tangent to from
            // start + dir (start-center) rot(+-Pi/2)
            //r = RotationTransform[Pi/2]{x,y} ->{-y,x}
            //r = RotationTransform[-Pi/2]{x,y} ->{y,-x}
            let radie_dir_1 = Pnt {
                x: svg_start.x - center_x,
                y: svg_start.y - center_y,
            };
            let tangent_1 = Pnt {
                x: svg_start.x - arrow_dir * radie_dir_1.y,
                y: svg_start.y + arrow_dir * radie_dir_1.x,
            };
            let radie_dir_2 = Pnt {
                x: svg_end.x - center_x,
                y: svg_end.y - center_y,
            };
            let tangent_2 = Pnt {
                x: svg_end.x - arrow_dir * radie_dir_2.y,
                y: svg_end.y + arrow_dir * radie_dir_2.x,
            };
            let arr_1: String = self.arrow_at_and_dir(svg_start, tangent_1);
            let arr_2: String = self.arrow_at_and_dir(svg_end, tangent_2);
            resut + &arr_1 + &arr_2
        } else {
            resut
        }
    }
    /*----------------------------------------*/
    fn gen_intro(&self, put_init_here: &mut Vec<String>, dims: SetupStyles) {
        put_init_here.push("".to_string());
        put_init_here.push(
            "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">
            <html xmlns=\"http://www.w3.org/1999/xhtml\">"
            .to_string());
        put_init_here.push("<body>".to_string());
        put_init_here.push(format!(
            "<svg height='{}' width='{}' xmlns='http://www.w3.org/2000/svg'>",
            dims.width_and_heigth.y, dims.width_and_heigth.x
        ));
        // style for the html stuff
        put_init_here.push(
            "<defs><style type='text/css'>@namespace 'http://www.w3.org/2000/svg';".to_string(),
        );
        put_init_here
            .push(".LINERESULT_CIRC {stroke:green;fill:none;stroke-width:1.5;}".to_string());
        put_init_here.push(".LINEEARTH_CIRC {stroke:red;fill:none;stroke-width:1.5;}".to_string());
        put_init_here.push(".LINEEVENT_CIRC {stroke:blue;fill:none;stroke-width:1.5;}".to_string());
        put_init_here.push(".STATE_CIRC {stroke:green; stroke-width:2; fill:yellow;}".to_string());
        put_init_here.push(format!(
            ".STATETEXT_CIRC {{font-family: monospace;font-size:{}px;	text-anchor:middle;}}",
            dims.state_font_size
        ));
        put_init_here.push(format!(
            ".RESULTTEXT_CIRC {{font-family: monospace;font-size:{}px; text-anchor:left;}}",
            dims.cond_font_size
        ));
        put_init_here.push(format!(
            ".RESULTTEXT_CIRC_ROT {{font-family: monospace;font-size:{}px; text-anchor:end;}}",
            dims.cond_font_size
        ));
        put_init_here.push(format!(
            ".EVENTTEXT_CIRC {{font-family: monospace;font-size:{}px; text-anchor:middle;}}",
            dims.event_font_size
        ));
        put_init_here.push(format!(
            ".EVENTTEXT_CIRC_ROT {{font-family: monospace;font-size:{}px; text-anchor:middle;}}",
            dims.event_font_size
        ));
        put_init_here.push("</style>".to_string());
        put_init_here.push("</defs>".to_string());
        put_init_here.push(format!(
            "<g transform='rotate(0 0 0)
translate({} {})
skewX(0)
scale(1 1)'>",
            dims.width_and_heigth.x / 2.0,
            dims.width_and_heigth.y / 2.0
        ));
    }
    /*----------------------------------------*/
    fn gen_finish_circle(&self, put_gen_content_here: &mut Vec<String>) -> String {
        put_gen_content_here.push(format!("</g></svg>"));
        put_gen_content_here.push("</body>".to_string());
        put_gen_content_here.push("</html>".to_string());
        put_gen_content_here.join("\n")
    }
    /*---------------------------------------------*/
    fn arc(
        &self,
        center_x: f32,
        center_y: f32,
        radie: f32,
        start_angle: f32,
        end_angle: f32,
        _with_arrows: bool,
    ) -> String {
        fn normalize(p1: Pnt, p2: Pnt) -> Pnt {
            let mut dir = Pnt {
                x: p2.x - p1.x,
                y: p2.y - p1.y,
            };
            let len: f32 = (dir.x * dir.x + dir.y * dir.y).sqrt();
            dir.x /= len;
            dir.y /= len;
            dir
        }
        /*--------------------------------------*/
        fn begin_end(
            center_x: f32,
            center_y: f32,
            radie: f32,
            at_angle: f32,
            soft_corner_radie: f32,
            soft_corner_angle: f32,
        ) -> String {
            let _centerp = Pnt {
                x: (radie - soft_corner_radie) * soft_corner_angle.cos(),
                y: (radie - soft_corner_radie) * soft_corner_angle.sin(),
            };
            let _pstart = Pnt {
                x: (radie - soft_corner_radie) * soft_corner_angle.cos(),
                y: 0.0,
            };
            let _pend = Pnt {
                x: radie * soft_corner_angle.cos(),
                y: radie * soft_corner_angle.sin(),
            };
            let mut pts: Vec<Pnt> = Vec::new();
            mjukcirkel(
                soft_corner_radie,
                _centerp,
                _pstart,
                _pend,
                1.0,
                10,
                &mut pts,
            );
            pts.push(_pend);
            let ut_ett: Vec<String> = pts.iter().map(|xy| format!("{},{}", xy.x, xy.y)).collect();
            let done_ett = format!(
        "<polyline points='{}' stroke='red' fill='none' transform='translate({},{}) rotate({})'/>",
        ut_ett.join(" "),
        center_x,
        center_y,
        at_angle * 180.0 / std::f32::consts::PI
      );
            done_ett
        }
        /*--------------------------------------*/
        fn mjukcirkel(
            radie: f32,
            centerp: Pnt,
            p_start: Pnt,
            p_end: Pnt,
            m_indrag: f32,
            max_pcalc: usize,
            pts: &mut Vec<Pnt>,
        ) {
            // Korda satsen
            // Solve[(2*radie - indrag)*indrag == dist_start_end/2*dist_start_end/2,
            let dist_start_end = ((p_start.x - p_end.x) * (p_start.x - p_end.x)
                + (p_start.y - p_end.y) * (p_start.y - p_end.y))
                .sqrt();
            let roten = -(dist_start_end / 2.0) * (dist_start_end / 2.0) + radie * radie;
            if roten < 0.0 {
                pts.push(p_start.clone());
                pts.push(p_end.clone());
                return;
            }
            if max_pcalc == 0 {
                pts.push(p_start.clone());
                return;
            }
            let mut mittp = Pnt {
                x: (p_start.x + p_end.x) / 2.0,
                y: (p_start.y + p_end.y) / 2.0,
            };
            let dir_mitt = normalize(centerp, mittp);
            let indrag = radie - roten.sqrt();
            mittp.x += dir_mitt.x * indrag;
            mittp.y += dir_mitt.y * indrag;

            if indrag < m_indrag {
                pts.push(p_start.clone());
                pts.push(mittp.clone());
                return;
            }
            mjukcirkel(radie, centerp, p_start, mittp, m_indrag, max_pcalc - 1, pts);
            mjukcirkel(radie, centerp, mittp, p_end, m_indrag, max_pcalc - 1, pts);
        }
        ////////////////
        // arc STARTS HERE
        // draw the arc
        // draw start and end soft corners
        let soft_corner_radie = 20.0;
        let soft_corner_angle = (soft_corner_radie / (radie - soft_corner_radie)).asin();
        let mut outer_circle_points: Vec<Pnt> = Vec::new();
        let outer_circle_points_as_string: Vec<String>; // = Vec::new();

        let centerp = Pnt {
            x: center_x,
            y: center_y,
        };
        let pstart: Pnt;
        let pend: Pnt;
        let d_v = end_angle - start_angle;
        if d_v > 0.0 {
            // Positive rotation
            // ut might be greater than 180 degrees
            if d_v > std::f32::consts::PI {
                // >180 . make rotation as complement
                // dv in [180,360]
                pstart = Pnt {
                    x: centerp.x + radie * (start_angle - soft_corner_angle).cos(),
                    y: centerp.y + radie * (start_angle - soft_corner_angle).sin(),
                };
                pend = Pnt {
                    x: centerp.x + radie * (end_angle + soft_corner_angle).cos(),
                    y: centerp.y + radie * (end_angle + soft_corner_angle).sin(),
                };
                mjukcirkel(
                    radie,
                    centerp,
                    pstart,
                    pend,
                    1.0,
                    10,
                    &mut outer_circle_points,
                );
                outer_circle_points.push(pend);
                outer_circle_points_as_string = outer_circle_points
                    .iter()
                    .map(|xy| format!("{},{}", xy.x, xy.y))
                    .collect();
                let begin_corner = begin_end(
                    center_x,
                    center_y,
                    radie,
                    start_angle,
                    soft_corner_radie,
                    -soft_corner_angle,
                );
                let end_corner = begin_end(
                    center_x,
                    center_y,
                    radie,
                    end_angle,
                    soft_corner_radie,
                    soft_corner_angle,
                );

                format!(
                    "<polyline points='{}' stroke='red' fill='none'/>",
                    outer_circle_points_as_string.join(" ")
                ) + &begin_corner
                    + &end_corner

            //"dv in [180,360]".to_string()
            } else {
                // < 180
                // dv in [0,180]
                pstart = Pnt {
                    x: centerp.x + radie * (start_angle + soft_corner_angle).cos(),
                    y: centerp.y + radie * (start_angle + soft_corner_angle).sin(),
                };
                pend = Pnt {
                    x: centerp.x + radie * (end_angle - soft_corner_angle).cos(),
                    y: centerp.y + radie * (end_angle - soft_corner_angle).sin(),
                };
                mjukcirkel(
                    radie,
                    centerp,
                    pstart,
                    pend,
                    1.0,
                    10,
                    &mut outer_circle_points,
                );
                outer_circle_points.push(pend);
                let begin_corner = begin_end(
                    center_x,
                    center_y,
                    radie,
                    start_angle,
                    soft_corner_radie,
                    soft_corner_angle,
                );
                let end_corner = begin_end(
                    center_x,
                    center_y,
                    radie,
                    end_angle,
                    soft_corner_radie,
                    -soft_corner_angle,
                );
                outer_circle_points_as_string = outer_circle_points
                    .iter()
                    .map(|xy| format!("{},{}", xy.x, xy.y))
                    .collect();

                format!(
                    "<polyline points='{}' stroke='red' fill='none'/>",
                    outer_circle_points_as_string.join(" ")
                ) + &begin_corner
                    + &end_corner

                //"dv in [0,180]".to_string()
            }
        } else {
            // Negative rotation
            if d_v < -std::f32::consts::PI {
                // < - 180 . make rotation as complement
                // dv in [-180,-360]
                pstart = Pnt {
                    x: centerp.x + radie * (start_angle + soft_corner_angle).cos(),
                    y: centerp.y + radie * (start_angle + soft_corner_angle).sin(),
                };
                pend = Pnt {
                    x: centerp.x + radie * (end_angle - soft_corner_angle).cos(),
                    y: centerp.y + radie * (end_angle - soft_corner_angle).sin(),
                };
                mjukcirkel(
                    radie,
                    centerp,
                    pstart,
                    pend,
                    1.0,
                    10,
                    &mut outer_circle_points,
                );
                outer_circle_points.push(pend);
                outer_circle_points_as_string = outer_circle_points
                    .iter()
                    .map(|xy| format!("{},{}", xy.x, xy.y))
                    .collect();
                let begin_corner = begin_end(
                    center_x,
                    center_y,
                    radie,
                    start_angle,
                    soft_corner_radie,
                    soft_corner_angle,
                );
                let end_corner = begin_end(
                    center_x,
                    center_y,
                    radie,
                    end_angle,
                    soft_corner_radie,
                    -soft_corner_angle,
                );
                format!(
                    "<polyline points='{}' stroke='red' fill='none'/>",
                    outer_circle_points_as_string.join(" ")
                ) + &begin_corner
                    + &end_corner
            //        "dv in [-180,-360]".to_string()
            } else {
                // dv in [0,-180]
                pstart = Pnt {
                    x: centerp.x + radie * (start_angle - soft_corner_angle).cos(),
                    y: centerp.y + radie * (start_angle - soft_corner_angle).sin(),
                };
                pend = Pnt {
                    x: centerp.x + radie * (end_angle + soft_corner_angle).cos(),
                    y: centerp.y + radie * (end_angle + soft_corner_angle).sin(),
                };
                mjukcirkel(
                    radie,
                    centerp,
                    pstart,
                    pend,
                    1.0,
                    10,
                    &mut outer_circle_points,
                );
                outer_circle_points.push(pend);
                outer_circle_points_as_string = outer_circle_points
                    .iter()
                    .map(|xy| format!("{},{}", xy.x, xy.y))
                    .collect();
                let begin_corner = begin_end(
                    center_x,
                    center_y,
                    radie,
                    start_angle,
                    soft_corner_radie,
                    -soft_corner_angle,
                );
                let end_corner = begin_end(
                    center_x,
                    center_y,
                    radie,
                    end_angle,
                    soft_corner_radie,
                    soft_corner_angle,
                );

                format!(
                    "<polyline points='{}' stroke='red' fill='none'/>",
                    outer_circle_points_as_string.join(" ")
                ) + &begin_corner
                    + &end_corner
            }
        }
    }
}
