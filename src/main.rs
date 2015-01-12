extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup,short_usage,Matches};
use std::os;

fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("{}", program);
    print!("{}", short_usage(brief.as_slice(), opts));
}

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    let opts = &[
        optflag("b", "", ""),
        optflag("e", "", ""),
        optflag("n", "", ""),
        optflag("t", "", ""),
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => {
            print!("{}: {}\n\n", program, f.to_string());
            print_usage(program.as_slice(), opts); return;
        }
    };

    if matches.free.is_empty() {
        print_usage(program.as_slice(), opts);
    };

    scanfiles(matches);
}

fn scanfiles(matches: Matches) {

    let files = if !matches.free.is_empty() {
        matches.free.clone()
    } else {
        print!("Nada");
        return;
    };

    for file in files.iter() {
        print!("{}\n", file);
    }
}
