use super::graphroot::Pnt;
use super::graphroot::SharedDims;
//use crate::GraphRoot;
use super::graphroot::GraphRoot;
use std::collections::HashMap;
use std::f32;

struct Boxdims {
    from_state_starts_at: usize,
    inevent_starts_at: usize,
    cond_starts_at: usize,
    state_radie: usize,
    right_margin: f32,
}
impl<'a> GraphRoot<'a> {
    pub fn asbox(&mut self, layoutnr: usize) -> String {
        self.setup_basics();
        self.use_layout_nr(layoutnr);
        let mut put_gen_content_here: Vec<String> = Vec::new();
        let mut box_dims = Boxdims {
            from_state_starts_at: 0,
            inevent_starts_at: 0,
            cond_starts_at: 0,
            state_radie: 25,
            right_margin: 300.0,
        };
        box_dims.inevent_starts_at = self.shared_dims.max_state_width;
        box_dims.cond_starts_at = box_dims.inevent_starts_at + self.shared_dims.max_inevent_width;
        let canvas_dims = self.build_box(&mut box_dims, &mut put_gen_content_here);
        let mut putinitrowshere: Vec<String> = Vec::new();
        self.gen_intro_box(&mut putinitrowshere, canvas_dims);
        putinitrowshere.append(&mut put_gen_content_here);
        self.gen_finish_box(&mut putinitrowshere)
    }
    /*--------------------------------------------------*/
    pub fn _as_optimal(&mut self, layoutnr: usize) -> String {
        let mut rows: Vec<String> = Vec::new();
        self.setup_basics();
        self.use_layout_nr(layoutnr);
        for is in &self.the_root.ordering_states {
            let _state = &self.the_root.all_states[*is];
            for es in &_state.ordering_events {
                let _event = &_state.all_events[*es];
                for cs in &_event.ordering_conds {
                    let _res = &_event.all_cond_results[*cs];
                    rows.push(
                        vec![
                            _state.name.clone(),
                            _event.name.clone(),
                            _res.cond.clone(),
                            _res.out_event.clone(),
                            _res.next_state.clone(),
                        ]
                        .join(","),
                    );
                }
            }
        }
        return rows.join("\n");
    }
    /*--------------------------------------------------*/
    fn build_box(&self, box_dims: &mut Boxdims, put_gen_content_here: &mut Vec<String>) -> Pnt {
        let mut _row: usize = 1;
        let mut connect_trans_to_nextstate: Vec<(String, f32, f32)> = Vec::new();
        let mut to_state_names_and_row: HashMap<String, (f32, usize)> = HashMap::new();
        //eprintln!("ordering_states={:?}",self.the_root.ordering_states);
        for is in &self.the_root.ordering_states {
            //          eprintln!("is = {:?}",is);
            let _state = &self.the_root.all_states[*is];

            let mut row_event_line: Vec<f32> = Vec::new();
            //            eprintln!("ordering_events={:?}",_state.ordering_events);
            for es in &_state.ordering_events {
                let _event = &_state.all_events[*es];
                let first_row_cond = _row;
                //                eprintln!("ordering_conds={:?}", _event.ordering_conds);
                for cs in &_event.ordering_conds {
                    let _res = &_event.all_cond_results[*cs];
                    // text cond + outmsg
                    let (real_x, real_y) = self.mdl2realxy(
                        &self.shared_dims,
                        box_dims.cond_starts_at as f32,
                        _row as f32,
                    );
                    put_gen_content_here.push(format!(
                        "<text class='RESULTTEXT' x='{:?}' y='{:?}'>{} / {}</text>",
                        real_x + self.shared_dims.text_of_x,
                        real_y + self.shared_dims.text_of_y,
                        _res.cond,
                        _res.out_event
                    ));
                    // line cond
                    let (real_x2, real_y2) = self.mdl2realxy(
                        &self.shared_dims,
                        box_dims.cond_starts_at as f32
                            + self.shared_dims.max_cond_and_outmsg_width as f32,
                        _row as f32,
                    );
                    put_gen_content_here.push(format!(
                        "<line class='LINERESULT' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}'></line>",
                        real_x, real_y, real_x2, real_y2
                    ));
                    if _res.next_state == self.the_fsm.final_state {
                        // down to erath
                        for i in vec![0, 1, 2].iter() {
                            let dx: f32 = *i as f32 * 5.0;
                            let dy: f32 = (*i - 3) as f32 * 5.0;
                            put_gen_content_here.push(format!(
                "<line class='LINEEARTH' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}'></line>",
                real_x2 + dx,
                real_y2 - dy,
                real_x2 + dx,
                real_y2 + dy
              ));
                        }
                    } else {
                        // remember from end to back line tostate
                        connect_trans_to_nextstate.push((
                            _res.next_state.to_string(),
                            real_x2,
                            real_y2,
                        ));
                    }
                    _row += 1;
                }
                // text event
                let eve_y = first_row_cond as f32 + (_row - 1 - first_row_cond) as f32 / 2.0;
                row_event_line.push(eve_y);
                let (real_x, real_y) =
                    self.mdl2realxy(&self.shared_dims, box_dims.inevent_starts_at as f32, eve_y);
                put_gen_content_here.push(format!(
                    "<text class='EVENTTEXT' x='{:?}' y='{:?}'>{}</text>",
                    real_x + self.shared_dims.text_of_x,
                    real_y + self.shared_dims.text_of_y,
                    _event.name
                ));
                // line event
                let (real_x2, real_y2) =
                    self.mdl2realxy(&self.shared_dims, box_dims.cond_starts_at as f32, eve_y);
                put_gen_content_here.push(format!(
                    "<line class='LINEEVENT' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}'></line>",
                    real_x, real_y, real_x2, real_y2
                ));
                // vertical line connecting results
                let (real_x1, real_y1) = self.mdl2realxy(
                    &self.shared_dims,
                    box_dims.cond_starts_at as f32,
                    first_row_cond as f32 - 0.1,
                );
                let (real_x2, real_y2) = self.mdl2realxy(
                    &self.shared_dims,
                    box_dims.cond_starts_at as f32,
                    (_row - 1) as f32 + 0.1,
                );
                put_gen_content_here.push(format!(
                    "<line class='LINERESULT' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}'></line>",
                    real_x1, real_y1, real_x2, real_y2
                ));
            }
            // text circle
            let start_eve_y = *row_event_line.first().unwrap();
            let end_eve_y = *row_event_line.last().unwrap();
            let eve_y = start_eve_y + (end_eve_y - start_eve_y) / 2.0;

            let (state_real_x, state_real_y) = self.mdl2realxy(
                &self.shared_dims,
                box_dims.from_state_starts_at as f32,
                eve_y,
            );
            put_gen_content_here.push(format!(
                "<circle class='CIRCLESTATE' cx='{:?}' cy='{:?}' r='{}'>{}</circle>",
                state_real_x, state_real_y, box_dims.state_radie as f32, _state.name
            ));
            // text state
            put_gen_content_here.push(format!(
                "<text class='STATETEXT' x='{:?}' y='{:?}'>{}</text>",
                state_real_x,
                state_real_y + self.shared_dims.text_of_y,
                _state.name
            ));
            // line state
            let (real_x2, real_y2) =
                self.mdl2realxy(&self.shared_dims, box_dims.inevent_starts_at as f32, eve_y);
            put_gen_content_here.push(format!(
                "<line class='LINEEVENT' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}'></line>",
                state_real_x + box_dims.state_radie as f32,
                state_real_y,
                real_x2,
                real_y2
            ));
            // vertical line connecting events
            let (real_x1, real_y1) = self.mdl2realxy(
                &self.shared_dims,
                box_dims.inevent_starts_at as f32,
                start_eve_y - 0.1,
            );
            let (real_x2, real_y2) = self.mdl2realxy(
                &self.shared_dims,
                box_dims.inevent_starts_at as f32,
                end_eve_y + 0.1,
            );
            put_gen_content_here.push(format!(
                "<line class='LINEEVENT' x1='{:?}' y1='{:?}' x2='{:?}' y2='{:?}'></line>",
                real_x1, real_y1, real_x2, real_y2
            ));
            let (_, real_y1) = self.mdl2realxy(&self.shared_dims, 0.0, _row as f32);
            // remember proper row for back into this state and colnumber in backplane
            to_state_names_and_row.insert(_state.name.clone(), (real_y1, *is));

            // draw back line
            // draw end path
            // draw arc
            if _state.name != self.the_fsm.initial_state {
                // nothing goes to init state so ...
                // just group the back status line
                let (x, y) = self.mdl2realxy(
                    &self.shared_dims,
                    box_dims.cond_starts_at as f32
                        + self.shared_dims.max_cond_and_outmsg_width as f32,
                    _row as f32,
                );
                let _p4: Pnt = Pnt { x: x, y: y };
                let p5: Pnt = Pnt {
                    x: state_real_x,
                    y: y,
                };
                let p6: Pnt = Pnt {
                    x: p5.x - 80.0,
                    y: p5.y,
                };
                let mut dirp7p6: Pnt = Pnt {
                    x: state_real_x - p6.x,
                    y: state_real_y - p6.y,
                };
                let len: f32 = (dirp7p6.x * dirp7p6.x + dirp7p6.y * dirp7p6.y).sqrt();
                dirp7p6.x /= len;
                dirp7p6.y /= len;
                let p7: Pnt = Pnt {
                    x: state_real_x - dirp7p6.x * box_dims.state_radie as f32,
                    y: state_real_y - dirp7p6.y * box_dims.state_radie as f32,
                };
                put_gen_content_here.push(format!(
                    "<path class='PATHSTATEBACK' d='M {},{} L {},{} Q {},{} {},{}'></path>",
                    _p4.x, _p4.y, p5.x, p5.y, p6.x, p6.y, p7.x, p7.y
                ));
                // and a nice arrow into the state
                put_gen_content_here.push(self.arrow_at_and_dir(
                    p7,
                    Pnt {
                        x: state_real_x,
                        y: state_real_y,
                    },
                ));
            }
            _row += 1;
        }
        let (canvas_dx, canvas_dy) = self.mdl2realxy(
            &self.shared_dims,
            box_dims.cond_starts_at as f32 + self.shared_dims.max_cond_and_outmsg_width as f32,
            _row as f32,
        );

        for (tostate, end_trans_x, end_trans_y) in connect_trans_to_nextstate {
            let (state_back_y, _back_plane_index) = to_state_names_and_row.get(&tostate).unwrap();
            let p0: Pnt = Pnt {
                x: end_trans_x,
                y: end_trans_y,
            };
            let mut down = 1.0;
            let tan_len = 15.0;
            if end_trans_y > *state_back_y {
                down = -1.0;
            }
            let _space_x_backplane = (box_dims.right_margin - 2.0 * tan_len)
                / (self.the_root.ordering_states.len() as f32);
            let _pa = p0.clone();
            let _xbackplane =
                p0.x + 2.0 * tan_len + (*_back_plane_index as f32 * _space_x_backplane) as f32;
            let _pc: Pnt = Pnt {
                x: _xbackplane,
                y: _pa.y,
            };
            let _pb: Pnt = Pnt {
                x: _pc.x - tan_len,
                y: _pc.y,
            };
            let _pd: Pnt = Pnt {
                x: _pc.x,
                y: _pb.y + tan_len * down,
            };
            let _ph: Pnt = Pnt {
                x: _pa.x,
                y: *state_back_y,
            };

            let _pg: Pnt = Pnt { x: _pb.x, y: _ph.y };
            let _pf: Pnt = Pnt { x: _pc.x, y: _pg.y };
            let _pe: Pnt = Pnt {
                x: _pf.x,
                y: _pf.y - tan_len * down,
            };
            put_gen_content_here.push(format!(
              "<path class='PATHRESULT' d='M {},{} L {},{} Q {},{} {},{} L {},{} Q {},{} {},{} L {},{}'></path>",
              _pa.x, _pa.y, _pb.x, _pb.y, _pc.x, _pc.y, _pd.x, _pd.y, _pe.x, _pe.y, _pf.x, _pf.y, _pg.x, _pg.y, _ph.x, _ph.y
          ));
            // and a nice arrow down or up to the to state line
            let _pmitt: Pnt = Pnt {
                x: (_pd.x + _pe.x) / 2.0,
                y: (_pd.y + _pe.y) / 2.0,
            };
            put_gen_content_here.push(self.arrow_at_and_dir(_pmitt, _pe));

            //            eprintln!("{},{},{}",p0.x,_back_plane_index,_xbackplane);
            // distribute into shared_dims.right_margin
            let calc_dx = (p0.y - *state_back_y).abs() / canvas_dy * box_dims.right_margin;
            let p1: Pnt = Pnt {
                x: p0.x + calc_dx,
                y: p0.y,
            };
            let p3: Pnt = Pnt {
                x: p1.x,
                y: *state_back_y,
            };
            let _p2: Pnt = Pnt {
                x: p1.x,
                y: (p1.y + p3.y) / 2.0,
            };
            let _p4: Pnt = Pnt { x: p0.x, y: p3.y };
            // draw from end a trans to back-lin state
            /*
            put_gen_content_here.push(format!(
                "<path class='PATHRESULT' d='M {},{} Q {},{} {},{} Q {},{} {},{}'></path>",
                p0.x, p0.y, p1.x, p1.y, _p2.x, _p2.y, p3.x, p3.y, _p4.x, _p4.y
            ));
            // and a nice arrow down or up to the to state line
            put_gen_content_here.push(self.arrow_at_and_dir(_p2, p3));
            */
        }
        Pnt {
            x: canvas_dx as f32 + box_dims.right_margin,
            y: canvas_dy as f32,
        }
    }
    /*----------------------------------------*/
    fn gen_intro_box(&self, put_init_here: &mut Vec<String>, dims: Pnt) {
        put_init_here.push("".to_string());
        put_init_here.push(
            "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">
            <html xmlns=\"http://www.w3.org/1999/xhtml\">"
            .to_string());
        put_init_here.push("<body>".to_string());
        put_init_here.push(format!(
            "<svg height='{}' width='{}' xmlns='http://www.w3.org/2000/svg'>",
            dims.y, dims.x
        ));
        // style for the html stuff
        put_init_here.push(
            "<defs><style type='text/css'>@namespace 'http://www.w3.org/2000/svg';".to_string(),
        );
        put_init_here.push(".LINERESULT {stroke:green;fill:none;stroke-width:1.5;}".to_string());
        put_init_here.push(".LINEEARTH {stroke:red;fill:none;stroke-width:1.5;}".to_string());
        //
        put_init_here.push(".ARROW {fill:blue;}".to_string());
        put_init_here.push(".PATHRESULT {stroke:green;fill:none;stroke-width:1.5;}".to_string());
        put_init_here.push(".PATHSTATEBACK {stroke:black;fill:none;stroke-width:1.5;}".to_string());

        put_init_here.push(".LINEEVENT {stroke:blue;fill:none;stroke-width:1.5;}".to_string());
        put_init_here.push(".CIRCLESTATE {stroke:green; stroke-width:2; fill:yellow;}".to_string());
        put_init_here.push(
            ".STATETEXT {font-family: monospace;font-size:16px;	text-anchor:middle;}".to_string(),
        );
        put_init_here.push(
            ".RESULTTEXT {font-family: monospace;font-size:14px; text-anchor:left;}".to_string(),
        );
        put_init_here.push(
            ".RESULTTEXT_CIRC {font-family: monospace;font-size:14px; text-anchor:left;}"
                .to_string(),
        );
        put_init_here.push(
            ".RESULTTEXT_CIRC_ROT {font-family: monospace;font-size:14px; text-anchor:end;}"
                .to_string(),
        );
        put_init_here.push(
            ".EVENTTEXT {font-weight: bold;font-family: monospace;font-size:16px; text-anchor:left;}".to_string(),
        );
        put_init_here.push(
            ".EVENTTEXT_CIRC {font-family: monospace;font-size:16px; text-anchor:left;}"
                .to_string(),
        );
        put_init_here.push(
            ".EVENTTEXT_CIRC_ROT {font-family: monospace;font-size:16px; text-anchor:end;}"
                .to_string(),
        );
        put_init_here.push("</style>".to_string());
        put_init_here.push("</defs>".to_string());
    }
    /*----------------------------------------*/
    fn gen_finish_box(&self, put_gen_content_here: &mut Vec<String>) -> String {
        put_gen_content_here.push(format!("</svg>"));
        put_gen_content_here.push("</body>".to_string());
        put_gen_content_here.push("</html>".to_string());
        put_gen_content_here.join("\n")
    }
    /*----------------------------------------*/
    pub fn mdl2realxy(&self, shared_dims: &SharedDims, xin: f32, yin: f32) -> (f32, f32) {
        (
            shared_dims.x_off as f32 + shared_dims.scale_x * xin,
            shared_dims.y_off as f32 + shared_dims.scale_y * yin,
        )
    }
}
