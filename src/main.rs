use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "bashfull", about = "never forget a bash command again")]
enum Bashfull {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name

    Save {
        #[structopt(short = "d", long = "description")]
        descript: String,

        #[structopt(short = "c", long = "command")]
        command: String,

    },
    Query {
        #[structopt(short = "d", long = "description")]
        descript: String,
    },
}

fn main() {
    let opt = Bashfull::from_args();
    print_type_of(&opt);
    println!("{:?}", opt);
    //dbg!(opt);
}