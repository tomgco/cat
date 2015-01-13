extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup,short_usage,Matches};
use std::os;
use std::io::{File, stdout};

fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("{}", program);
    print!("{}", short_usage(brief.as_slice(), opts));
}

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    let opts = &[
        optflag("u", "", ""),
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

    cat(matches);
}

fn cat(matches: Matches) {

    let files = if !matches.free.is_empty() {
        matches.free.clone()
    } else {
        return;
    };

    for file in files.iter() {
        let contents = File::open(&Path::new(file)).read_to_end();
        for line in contents.iter() {
            stdout().write(line.as_slice());
        }
    }
}
