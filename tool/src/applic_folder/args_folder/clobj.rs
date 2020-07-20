use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

#[derive(Debug, Clone)]
pub struct Tags {
    pub name: &'static str,
    pub occurs: usize,
    pub short_name: char,
}

impl Default for Tags {
    fn default() -> Tags {
        Tags {
            name: "",
            occurs: 0,
            short_name: '#',
        }
    }
}
#[derive(Debug)]
pub struct Opt2beused {
    name: String,
    //  name: &'static str,
    pub occurs: usize,
    inuse: bool,
    values: Vec<String>,
}
#[derive(Debug)]
pub struct Retvalueoption {
    was_used: bool,
    pub values: Vec<String>,
}
//////
#[derive(Debug)]
struct Ifc2commandline {
    args: Vec<String>,
    pos: usize,
    args_len: usize,
}
/*
#[derive(Debug)]
enum KindofOptParsing {
    ShortOpt,
    SingleOrLastShortOpt,
    LongOpt,
}
*/
#[derive(Debug)]
pub struct Clobj {
    errors: Vec<String>,
    ifc: Ifc2commandline,
    legal_opts: Vec<Tags>,
    used_ops: HashMap<String, Opt2beused>,
    map_short_2_normal: HashMap<char, String>,
    found_names: Vec<String>,
    help_found: bool,
}
//------------------------
impl Clobj {
    pub fn new() -> Clobj {
        Clobj {
            errors: Vec::new(),
            ifc: Ifc2commandline {
                args: Vec::new(),
                pos: 0,
                args_len: 0,
            },
            legal_opts: Vec::new(),
            used_ops: HashMap::new(),
            map_short_2_normal: HashMap::new(),
            found_names: Vec::new(),
            help_found: false,
        }
    }
    //------------------------
    pub fn get_typed_options(&mut self) -> HashSet<String> {
        let mut typed: HashSet<String> = HashSet::new();
        for (key, _) in self.used_ops.iter().filter(|&(_, val)| val.inuse) {
            typed.insert(key.to_string());
        }
        return typed;
    }
    //------------------------
    pub fn help_seen(&mut self) -> bool {
        return self.help_found;
    }
    //------------------------
    pub fn has_option(
        &mut self,
        typed_option: &mut HashSet<String>, // "--longname -l and other options"
        the_option: &str,
    ) -> Result<Retvalueoption, bool> {
        match self.used_ops.get(&the_option.to_string()) {
            Some(an_opt2beused) => {
                if an_opt2beused.inuse {
                    typed_option.remove(the_option);
                    return Result::Ok(Retvalueoption {
                        was_used: true,
                        values: an_opt2beused.values.clone(),
                    });
                } else {
                    return Result::Err(false);
                }
            }
            None => (return Result::Err(false)),
        }
    }
    //------------------------
    pub fn get_commands(&mut self) -> Vec<String> {
        return self.found_names.clone();
    }
    //------------------------
    pub fn collectparams(&mut self, key: &String, the_equal_param: &str) {
        let mut an_opt2beused = self.used_ops.get_mut(key).unwrap();
        let user_occurs = an_opt2beused.occurs;
        if the_equal_param.len() > 0 {
            if user_occurs > 0 {
                // required but we already have on command line
                // JOBB split -ijk=anana,sksksk,jsjsj on comma
                let opt_args: Vec<&str> = the_equal_param.split(|c| c == ',' || c == ' ').collect();
                if opt_args.len() > an_opt2beused.occurs {
                    self.errors.push("ERROR(93) To many args given".to_string());
                } else {
                    an_opt2beused.values =
                        opt_args.iter().map(|x| x.to_string()).collect::<Vec<_>>();
                }
            } else {
                // have some =values in command but the option does not care
                self.errors.push(format!(
                    "ERROR(3) param given but feasable option {:?}={:?}",
                    key, the_equal_param
                ));
            }
            self.consume_item();
        } else {
            if user_occurs == 0 {
                // no args
                self.consume_item();
            } else {
                // now consume some args from cli (until EOL or a "-")
                self.ifc.pos += 1; // can't do self.consume_item(); here
                                   //                self.consume_item();
                let mut temp: Vec<String> = Vec::new();
                let mut occ_cnt = 0;
                while occ_cnt < user_occurs
                    && self.ifc.pos < self.ifc.args_len
                    && !self.ifc.args[self.ifc.pos].starts_with("-")
                {
                    //                    an_opt2beused
                    //                        .values
                    temp.push(self.ifc.args[self.ifc.pos].clone());
                    occ_cnt += 1;
                    self.ifc.pos += 1; // can't do self.consume_item(); here
                                       //self.consume_item();
                }
                // if --option aaa,bbb ccc --nextoption
                // we get have ["aaa,bbb","ccc"]
                for x in temp {
                    let y: Vec<&str> = x.split(",").collect();
                    for z in y {
                        an_opt2beused.values.push(z.to_string())
                    }
                }
                // how did it end
                if occ_cnt == user_occurs {
                    // ok
                } else if user_occurs == std::usize::MAX && occ_cnt > 0 {
                    //ok at least one
                } else {
                    self.errors.push(format!(
                        "ERROR(31) Too few params givenfor option '{}'. Required is max '{:?}'",
                        key, user_occurs
                    ));
                }
            }
        }
    }
    //------------------------
    pub fn consume_item(&mut self) {
        self.ifc.pos += 1;
    }
    //------------------------
    pub fn dump(&mut self) {
        //        println!("found_names={:?}", self.found_names);
    }
    //------------------------
    pub fn parse_one_dash_tag(
        &mut self,
        the_key: &String,
        key_as_one_char: char,
        use_case: usize,
        the_equal_param: &str,
    ) {
        // use case 1. used for each a,b,c in -abcd
        // use case 2. used for d in -abcd
        // use case 3. used for d or dlong in -d or --dlong
        // if we have -d=par1,par2... the the_equal_param hs value

        match self.used_ops.get_mut(the_key) {
            Some(an_opt2beused) => {
                // found
                if an_opt2beused.inuse {
                    // duplicate use
                    self.errors
                        .push(format!("ERROR(5) duplicate use of option {:?}", the_key));
                    if use_case == 2 || use_case == 3 {
                        self.consume_item();
                    }
                } else {
                    // first time seen
                    an_opt2beused.inuse = true;
                    // 1. Ensure no parms required because we can't give them here
                    if use_case == 1 {
                        if an_opt2beused.occurs != 0 {
                            self.errors.push(format!(
                                "ERROR(49) Option need parameter value(s) {:?}",
                                the_key
                            ));
                            if use_case == 2 || use_case == 3 {
                                self.consume_item();
                            }
                            return;
                        } else {
                            return; // done use_case 1
                        }
                    } else {
                        // use case 2 and 3
                        self.collectparams(&the_key, &the_equal_param);
                    }
                }
            }
            None => {
                // notfound
                if use_case == 3 {
                    self.errors
                        .push(format!("ERROR(1): option '{}' not found ", the_key));
                    self.consume_item();
                    return;
                }
                // now in use case 1 and 2

                // we have a short name and short key
                // look up in map_short_2_normal
                match self.map_short_2_normal.get(&key_as_one_char) {
                    Some(longoptionvalue) => {
                        let an_opt2beused = self.used_ops.get_mut(longoptionvalue).unwrap();
                        if an_opt2beused.inuse {
                            self.errors.push(format!(
                                "ERROR(67A) duplicate use of option {:?}",
                                longoptionvalue
                            ));
                            self.consume_item();
                        } else {
                            an_opt2beused.inuse = true;
                            if use_case == 1 {
                                if an_opt2beused.occurs != 0 {
                                    self.errors.push(format!(
                                        "ERROR(49) option need parameter value(s) {:?}",
                                        key_as_one_char
                                    ));
                                } else {
                                    // done (NO PARAMS TO PARS)
                                    // collect params
                                    let longoptionvalue_cloned = longoptionvalue.clone();
                                    self.collectparams(&longoptionvalue_cloned, &the_equal_param);
                                }
                            } else {
                                // use case
                                // collect params
                                let longoptionvalue_cloned = longoptionvalue.clone();
                                self.collectparams(&longoptionvalue_cloned, &the_equal_param);
                            }
                        }
                    }
                    None => {
                        self.errors.push(format!(
                            "ERROR(7) short not found at all {:?}",
                            key_as_one_char
                        ));
                        if use_case == 2 {
                            self.consume_item();
                        }
                    }
                }
            }
        }
    }
    //------------------------
    pub fn set_legal_opts(&mut self, legal_opts: &Vec<Tags>) {
        self.legal_opts = legal_opts.to_vec();
        self.build_key_collection();
        self.build_short_2_normal_map();
    }
    //------------------------
    pub fn go(&mut self) -> Vec<String> {
        self.ifc.args = env::args().collect();
        self.ifc.args_len = self.ifc.args.len();
        self.ifc.pos = 1;
        while self.ifc.pos < self.ifc.args_len {
            let the_arg = &self.ifc.args[self.ifc.pos];
            if !the_arg.starts_with("-") {
                self.found_names.push(self.ifc.args[self.ifc.pos].clone());
                self.consume_item();
            } else if the_arg.starts_with("--") {
                if the_arg.len() == 2 {
                    self.consume_item(); // dismiss "--" empty
                    continue;
                }
                self.go4longtags(); // consume the "--something[ more]
            } else {
                if the_arg.len() == 1 {
                    self.consume_item(); // dismiss "-" empty
                    continue;
                }
                self.go4shorttags(); // consume the "-abc[ more]
            }
        }
        return self.errors.clone();
    }
    //-----------------------
    fn go4shorttags(&mut self) {
        let mut the_arg: String = self.ifc.args[self.ifc.pos].clone();
        the_arg.remove(0); // skip -
        let key_and_vals: Vec<&str> = the_arg.split("=").collect();
        let the_equal_param: &str;
        if key_and_vals.len() > 1 {
            the_equal_param = key_and_vals[1];
        } else {
            the_equal_param = "";
        }

        let key = key_and_vals[0].to_string();
        let key_as_bytes = key.as_bytes();
        // walk through each in -xabc except last one
        // FIRST PART -xab (no value allowd. impossible)
        for indx in 0..key_as_bytes.len() - 1 {
            let one_letter_key: String = String::from_utf8(vec![key_as_bytes[indx]]).unwrap();
            if one_letter_key == "h".to_string() {
                self.help_found = true;
            }
            let one_letter_key_as_char: char = key_as_bytes[indx] as char;

            self.parse_one_dash_tag(&one_letter_key, one_letter_key_as_char, 1, the_equal_param);
            continue;
        }
        // 2
        // LAST part of -xabc is -c (or the first and only)
        //        let one_letter_key:char = key_as_bytes[key_as_bytes.len()-1] as char;

        let one_letter_key: String =
            String::from_utf8(vec![key_as_bytes[key_as_bytes.len() - 1]]).unwrap();
        if one_letter_key == "h".to_string() {
            self.help_found = true;
        }
        let one_letter_key_as_char: char = key_as_bytes[key_as_bytes.len() - 1] as char;
        self.parse_one_dash_tag(&one_letter_key, one_letter_key_as_char, 2, the_equal_param);
    }
    //-----------------------
    fn go4longtags(&mut self) {
        let the_arg = &self.ifc.args[self.ifc.pos];
        let arg = the_arg.get(2..).unwrap().to_string(); // skip --
        let key_and_vals: Vec<&str> = arg.split("=").collect();
        let the_equal_param: &str;
        if key_and_vals.len() > 1 {
            the_equal_param = key_and_vals[1];
        } else {
            the_equal_param = "";
        }
        let key = key_and_vals[0].to_string();
        if key == "help".to_string() {
            self.help_found = true;
        }
        self.parse_one_dash_tag(&key, ' ', 3, the_equal_param);
    }
    //-----------------------
    fn build_key_collection(&mut self) {
        for user_opt in self.legal_opts.clone() {
            let _abc = Opt2beused {
                name: user_opt.name.to_string(),
                occurs: user_opt.occurs,
                inuse: false,
                values: Vec::new(),
            };
            self.used_ops.insert(_abc.name.clone(), _abc);
        }
    }
    //-----------------------
    fn build_short_2_normal_map(&mut self) {
        for user_opt in self.legal_opts.clone() {
            if user_opt.short_name != '#' {
                self.map_short_2_normal
                    .insert(user_opt.short_name, user_opt.name.to_string());
            }
        }
    }
}
