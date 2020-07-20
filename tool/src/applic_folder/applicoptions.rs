// https://doc.rust-lang.org/rust-by-example/macros/variadics.html
// https://stackoverflow.com/questions/28951503/how-can-i-create-a-function-with-a-variable-number-of-arguments

use super::args_folder::clobj::Tags;

pub fn get_legal_options() -> Vec<Tags> {
    return vec![
        Tags {
            name: "help",
            short_name: 'h',
            ..Default::default()
        },
        Tags {
            name: "astty",
            ..Default::default()
        },
        Tags {
            name: "ascsv",
            ..Default::default()
        },
        Tags {
            name: "asmatrix",
            ..Default::default()
        },
        Tags {
            name: "asbox",
            ..Default::default()
        },
        Tags {
            name: "ascircle",
            ..Default::default()
        },
        Tags {
            name: "invert",
            short_name: 'i',
            ..Default::default()
        },
        Tags {
            name: "asflat",
            ..Default::default()
        },
        Tags {
            name: "asflat",
            ..Default::default()
        },
        Tags {
            name: "destproject",
            short_name: 'd',
            occurs: 1,
            ..Default::default()
        },
        Tags {
            name: "new",
            short_name: 'n',
            occurs: 0,
            ..Default::default()
        },
        Tags {
            name: "update",
            short_name: 'u',
            occurs: 0,
            ..Default::default()
        },
        Tags {
            name: "layoutnumber",
            short_name: 'l',
            occurs: 1,
            ..Default::default()
        },
        Tags {
            name: "xyz",
            short_name: 'x',
            //            occurs: 2,
            occurs: std::usize::MAX,
            ..Default::default()
        },
        Tags {
            name: "groupby",
            ..Default::default()
        },
        Tags {
            name: "keys",
            short_name: 'k',
            occurs: std::usize::MAX,
            ..Default::default()
        },
        Tags {
            name: "cols",
            short_name: 'c',
            occurs: std::usize::MAX,
            ..Default::default()
        },
        Tags {
            name: "what",
            ..Default::default()
        },
        Tags {
            name: "W",
            ..Default::default()
        },
    ];
}
pub fn get_usage() -> Vec<&'static str> {
    return vec![
        "<fsm> [-i] [options] show ( --astty | --ascsv | --asmatrix )",
        "<fsm> [-i] [options] show --groupby --keys key1,.. --cols col1,...",
        "<fsm> [-i] [options] genrust (--new | --update ) [ --what ] --destproject <directory>",
        "<fsm> [-i] [options] genc (--new | --update ) [ --what ] --destproject <directory>",
        "<fsm> [-i] [options] genspectext",
        "<fsm> [-i] [options] calc",
        "<fsm> [-i] [options] chat",
        "<fsm> [-i] [options] graph [ --layoutnumber n ] ( --asbox | --ascircle )",
        "<fsm> [-i] chat",
    ];
}
pub fn get_options() -> Vec<&'static str> {
    return vec![
        "<fsm> a finite state machine source file",
        "--invert,-i       Will work on first inverting the ",
        "--help,-h         Show help",
        "--astty           Simple more pretty form of transactions",
        "--ascsv           Raw inputformat",
        "--asmatrix        Tabulated in event and state",
        "--groupby         Grouped by --keys, -k showing specified --cols, -c",
        "--destproject     Will generate source and drivers for a
    Server and Client fsm talking to each other",
        "--new             Will generate all required sources",
        "--update          Will generate the sources for the FSM object only",
        "--what            Will show what files to be generated. But no files generated",
        "--layoutnumber,-l Use a certain optional layoutnumber got from calc command  ",
    ];
}
