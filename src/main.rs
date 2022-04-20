use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "toast", about = "The best place to stack your JAM")]
enum Bashfull {
    Save {
        #[structopt()]
        descript: String,

        #[structopt()]
        command: String,
    },
}

fn main() {
    let opt = Bashfull::from_args();
    println!("{:?}", opt);
}