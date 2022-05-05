use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use std::fs;
use std::env;
use std::path::Path;
use serde_json;
use std::collections::HashMap;

mod describe;
mod login;
mod check_login;
mod recall;


// #[derive(Serialize, Deserialize, Debug)]
// struct APIResponse {
//     name: String,
// }

#[derive(Serialize, Deserialize, Debug)]
struct Req {
  status: u16,
  headers: HashMap<String, String>,
  body: Option<serde_json::Value>
}


#[derive(Debug, StructOpt)]
#[structopt(name = "bashfull", about = "never forget a bash command again")]
enum Bashfull {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(name = "describe")]
    Describe {
        #[structopt(short = "d", long = "description")]
        descript: String,

        #[structopt(short = "c", long = "command")]
        command: String,

    },
    #[structopt(name = "recall")]
    Recall {
        #[structopt(short = "d", long = "description")]
        descript: String,
    },
    #[structopt(name = "login")]
    Login {
        #[structopt(default_value="https://bashfull-server.vercel.app/profile")]
        url: String
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Bashfull::from_args() {
        Bashfull::Describe {descript, command} => {
            check_login::check_login().await;
            let res = describe::describe(descript, command).await;
        },
        Bashfull::Recall {descript} => {
            check_login::check_login().await;
            let res = recall::recall(descript).await;
        },
        Bashfull::Login {url} => {
            let res = login::login().await;
        },
    }   
    Ok(())
}    
