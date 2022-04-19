use clap::Clap;
use rand::Rng;

/// This program can say 'hello' to you, show you 'quote' of the day
/// and can even help you 'add' up numbers!
#[derive(Clap)]
#[clap(version = "1.0", author = "Robin Wang<mytechtip@github>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Save(Save),
    Search(Search)
}

/// Say hello
#[derive(Clap)]
struct Save {
    description: String,
    command: String
}

#[derive(Clap)]
struct Search {
    description: String
}

pub trait Display {
    fn show(&self) -> String;
}

impl Display for Save {
    fn show(&self) -> String {
        format!("Saving {}...", self.description)
    }
}

impl Display for Search {
    fn show(&self) -> String {
        format!("Searching {}...", self.description)
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Save(t) => {
            println!("{}", t.show());
        }
        SubCommand::Search(t) => {
            println!("{}", t.show());
        }
    }
}
