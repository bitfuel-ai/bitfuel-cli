use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use std::fs;
use std::env;
use std::path::Path;
use serde_json;
use std::collections::HashMap;

mod save;
mod login;
mod check_login;
mod get;
mod run;


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
    #[structopt(name = "save")]
    Save {
        #[structopt(default_value="https://bashfull-server.vercel.app/profile")]
        url: String
    },
    #[structopt(name = "get")]
    Get {
        #[structopt()]
        descript: String,
    },
    #[structopt(name = "login")]
    Login {
        #[structopt(default_value="https://bashfull-server.vercel.app/profile")]
        url: String
    },
    #[structopt(name = "run")]
    Run {
        #[structopt()]
        descript: String,
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Bashfull::from_args() {
        Bashfull::Save {url} => {
            check_login::check_login().await;
            let res = save::save().await;
        },
        Bashfull::Get {descript} => {
            check_login::check_login().await;
            let res = get::get(descript).await;
        },
        Bashfull::Run {descript} => {
            check_login::check_login().await;
            let res = run::run(descript).await;
        },
        Bashfull::Login {url} => {
            let res = login::login().await;
        },
    }   
    Ok(())
}    
