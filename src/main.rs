extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup,usage};
use std::os;

fn print_usage(program: &str, opts: &[OptGroup]) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", usage(brief.as_slice(), opts));
}

#[main]
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

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(program.as_slice(), opts);
        return;
    };

    println!("Hello, World!");
}
