use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use super::applicoptions::get_legal_options;
use super::applicoptions::get_options;
use super::applicoptions::get_usage;

use super::args_folder::clobj::Clobj;
use super::args_folder::clobj::Tags;

use super::fsm_folder::fsm::Fsm;
use super::fsm_folder::graph_folder::graphroot::GraphRoot;

/*-------------- read_machine from file --------------------------------*/
fn read_machine(filename: &String) -> Result<Vec<Vec<String>>, String> {
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();
    let mut res: Vec<Vec<String>> = Vec::new();
    match File::open(&path) {
        Err(why) => {
            return Result::Err(format!(
                "couldn't open '{}' , '{}'",
                display,
                why.to_string()
            ))
        }
        Ok(f2) => {
            let f3 = BufReader::new(f2);
            for line in f3.lines() {
                let x = line.unwrap();
                if x.starts_with("/") {
                    continue;
                }
                if x.len() <= 4 {
                    continue;
                };
                let columns = x.split(",");
                let mut vec: Vec<String> = Vec::new();
                for y in columns {
                    vec.push(y.to_string());
                }
                if vec.len() < 4 {
                    continue;
                };
                res.push(vec);
            }
        }
    };
    return Result::Ok(res);
}
//---------------------------------
fn _show_usage() {
    for a_row in get_usage() {
        println!("{}", a_row);
    }
}
//---------------------------------
fn _show_options() {
    for a_row in get_options() {
        println!("{}", a_row);
    }
}
//---------------------------------
fn show_errors(errors: Vec<String>) {
    for a_row in errors {
        println!("{}", a_row);
    }
}
//---------------------------------
fn ensure_one_in_choice(
    choices: Vec<&str>,
    typed_options: &mut HashSet<String>,
) -> Result<String, String> {
    let mut or_opts = HashSet::new();
    let error_answer: String;
    for an_opt in choices.iter() {
        or_opts.insert(an_opt.to_string());
    }
    let which_was_set: HashSet<String> = &*typed_options & &mut or_opts;
    // just one choice is allowed
    if which_was_set.len() != 1 {
        let x: String = choices
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" | ");
        error_answer = format!("Options must be one of '{}' ", x);
        return Result::Err(error_answer);
    } else {
        let mut the_option_typed_as_string: String = String::new();
        // there is only one value
        for a_value in which_was_set.iter() {
            the_option_typed_as_string = a_value.to_string();
        }
        typed_options.remove(&the_option_typed_as_string);
        return Result::Ok(the_option_typed_as_string);
    }
}
//---------------------------------
pub fn ensure_no_more_options(
    one_taken: &HashSet<String>,
    in_use: &HashSet<String>,
) -> HashSet<String> {
    return in_use - one_taken;
}
//---------------------------------
pub fn applicroot() {
    //    println!("applicroot start!");
    let check_empty = |typed_options: HashSet<String>| -> bool {
        if !typed_options.is_empty() {
            eprintln!("No options meaningsful here but got {:?}", typed_options);
            return false;
        } else {
            return true;
        }
    };
    let mut theclobj: Clobj = Clobj::new();
    let my_legal_opts: Vec<Tags> = get_legal_options();
    theclobj.set_legal_opts(&my_legal_opts);

    let errors = theclobj.go();
    if errors.len() != 0 {
        theclobj.dump();
        show_errors(errors);
        return;
    } else if theclobj.help_seen() {
        _show_usage();
        _show_options();
        return;
    }
    // do we have some kind of commands
    // or all is govern by -/-- options
    let commands = theclobj.get_commands();
    // here we have <file> <command> [options]
    if commands.len() != 2 {
        eprintln!(
            "Error: fsm_file_name and one command expected. Got {:?}",
            commands
        );
        return;
    }
    let mut typed_options = theclobj.get_typed_options();
    // pick infilename
    let mut machine_as_rows: Vec<Vec<String>>;
    match read_machine(&commands[0]) {
        Ok(_rows) => machine_as_rows = _rows,
        Err(why) => {
            eprintln!("{}", why);
            return;
        }
    }
    let mut use_fsm_obj = Fsm::new(String::from("server"));
    // check the source
    use_fsm_obj.set_rows(machine_as_rows.to_vec());
    if use_fsm_obj.check_details(&machine_as_rows) == false {
        return;
    }
    // take care of --invert
    let invert_wanted: bool = match theclobj.has_option(&mut typed_options, "invert") {
        Ok(_) => true,
        Err(_) => false,
    };
    if invert_wanted {
        // invert and overwrite the read input rows
        machine_as_rows = use_fsm_obj.mirror_direct(&machine_as_rows);
        // reset and use the mirror
        use_fsm_obj.set_rows(machine_as_rows.to_vec());
        if use_fsm_obj.check_details(&machine_as_rows) == false {
            return;
        }
    }
    //
    // pick command
    //
    let the_command = commands[1].as_str();
    match the_command {
        "show" => {
            // ensure one of  --astty | --ascsv | --asmatrix | --groupby
            // the optional [--invert] is removed above
            let the_choosen_option;
            match ensure_one_in_choice(
                vec!["astty", "ascsv", "asmatrix", "groupby"],
                &mut typed_options,
            ) {
                Ok(value) => {
                    the_choosen_option = value;
                }
                Err(txt) => {
                    eprintln!("{}", txt);
                    return;
                }
            }
            // go now
            if the_choosen_option == "groupby" {
                let keys = match theclobj.has_option(&mut typed_options, "keys") {
                    Ok(a_retvalueoption) => a_retvalueoption.values,
                    Err(_not_in_use) => vec![],
                };
                if keys.len() == 0 {
                    eprintln!("Missing --keys k1,k2");
                    return;
                }
                let cols = match theclobj.has_option(&mut typed_options, "cols") {
                    Ok(a_retvalueoption) => a_retvalueoption.values,
                    Err(_not_in_use) => vec![],
                };
                if cols.len() == 0 {
                    eprintln!("Missing --cols c1,c2");
                    return;
                }
                if !check_empty(typed_options) {
                    return;
                }
                println!("{}", use_fsm_obj.as_groupby(keys, cols));
            } else {
                if !check_empty(typed_options) {
                    return;
                }
                if the_choosen_option == "astty" {
                    println!("{}", use_fsm_obj.as_tty());
                } else if the_choosen_option == "ascsv" {
                    use_fsm_obj.as_csv()
                } else if the_choosen_option == "asmatrix" {
                    println!("{}", use_fsm_obj.as_matrix());
                }
            }
        }
        "genc" => {
            let dest_dir_name: String = match theclobj.has_option(&mut typed_options, "destproject")
            {
                Ok(a_retvalueoption) => a_retvalueoption.values[0].to_string(),
                Err(_not_in_use) => "".to_string(),
            };
            if dest_dir_name.len() == 0 {
                eprintln!("Missing destproject name");
                return;
            }
            let new_or_update;
            match ensure_one_in_choice(vec!["new", "update"], &mut typed_options) {
                Ok(value) => {
                    new_or_update = value;
                }
                Err(txt) => {
                    eprintln!("{}", txt);
                    return;
                }
            }
            let what_only_wanted: bool = match theclobj.has_option(&mut typed_options, "what") {
                Ok(_) => true,
                Err(_) => false,
            };
            if !check_empty(typed_options) {
                return;
            }
            // mirror required. build a local one
            let mut client_as_fsm_obj = Fsm::new(String::from("client"));
            let invers_4_gen_or_chat_as_rows = use_fsm_obj.mirror_direct(&machine_as_rows);
            client_as_fsm_obj.set_rows(invers_4_gen_or_chat_as_rows.to_vec());
            if client_as_fsm_obj.check_details(&invers_4_gen_or_chat_as_rows) == false {
                return;
            }
            Fsm::genc(
                use_fsm_obj,
                client_as_fsm_obj,
                dest_dir_name,
                new_or_update,
                what_only_wanted,
            );
        }
        "genrust" => {
            let dest_dir_name: String = match theclobj.has_option(&mut typed_options, "destproject")
            {
                Ok(a_retvalueoption) => a_retvalueoption.values[0].to_string(),
                Err(_not_in_use) => "".to_string(),
            };
            if dest_dir_name.len() == 0 {
                eprintln!("Missing destproject name");
                return;
            }
            let new_or_update;
            match ensure_one_in_choice(vec!["new", "update"], &mut typed_options) {
                Ok(value) => {
                    new_or_update = value;
                }
                Err(txt) => {
                    eprintln!("{}", txt);
                    return;
                }
            }
            let what_only_wanted: bool = match theclobj.has_option(&mut typed_options, "what") {
                Ok(_) => true,
                Err(_) => false,
            };
            if !check_empty(typed_options) {
                return;
            }
            // mirror required. build a local one
            let mut client_as_fsm_obj = Fsm::new(String::from("client"));
            let invers_4_gen_or_chat_as_rows = use_fsm_obj.mirror_direct(&machine_as_rows);
            client_as_fsm_obj.set_rows(invers_4_gen_or_chat_as_rows.to_vec());
            if client_as_fsm_obj.check_details(&invers_4_gen_or_chat_as_rows) == false {
                return;
            }
            Fsm::gen_sources(
                use_fsm_obj,
                client_as_fsm_obj,
                dest_dir_name,
                new_or_update,
                what_only_wanted,
            );
        }
        "graph" => {
            // ensure the optional '[ --layoutnumber n ]'
            let layoutnumber: usize = match theclobj.has_option(&mut typed_options, "layoutnumber")
            {
                Ok(a_retvalueoption) => a_retvalueoption.values[0].parse().unwrap(),
                Err(_not_in_use) => 0,
            };
            // --asflat | --ascircle
            let the_choosen_option;
            match ensure_one_in_choice(vec!["asbox", "ascircle"], &mut typed_options) {
                Ok(value) => {
                    the_choosen_option = value;
                }
                Err(txt) => {
                    eprintln!("{}", txt);
                    return;
                }
            }
            // go now
            if !check_empty(typed_options) {
                return;
            }
            let mut as_graph = GraphRoot::new(&mut use_fsm_obj);
            if the_choosen_option == "ascircle" {
                println!("{}", as_graph.ascircle(layoutnumber, 75));
            } else if the_choosen_option == "asbox" {
                println!("{}", as_graph.asbox(layoutnumber));
            }
        }
        "calc" => {
            if !check_empty(typed_options) {
                return;
            }
            let mut as_graph = GraphRoot::new(&mut use_fsm_obj);
            as_graph.find_optimum_better();
        }
        "genspectext" => {
            if !check_empty(typed_options) {
                return;
            }
            println!("{}", use_fsm_obj.gen_spec(&commands[0]));
        }
        "chat" => {
            if !check_empty(typed_options) {
                return;
            }
            // mirror required. build a local one
            let mut client_as_fsm_obj = Fsm::new(String::from("client"));
            let invers_4_gen_or_chat_as_rows = use_fsm_obj.mirror_direct(&machine_as_rows);
            client_as_fsm_obj.set_rows(invers_4_gen_or_chat_as_rows.to_vec());
            if client_as_fsm_obj.check_details(&invers_4_gen_or_chat_as_rows) == false {
                return;
            }
            Fsm::dial(&mut use_fsm_obj, &mut client_as_fsm_obj);
        }
        _ => {
            eprintln!("Error: Command '{}' not usefule", commands[1]);
            return;
        }
    }
}
