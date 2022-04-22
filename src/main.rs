use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use std::fs;
use std::env;




#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    name: String,
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



async fn describe(_descript:String, _command:String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://bashfull-server.vercel.app/api/test")
        .send().await?;
    //println!("Success! {:?}", response);

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => println!("Success! {:?}", parsed),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Request was unauthorized...");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };

    Ok(())
}

async fn recall(_descript:String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://bashfull-server.vercel.app/api/test")
        .send().await?;
    //println!("Success! {:?}", response);

    match response.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match response.json::<APIResponse>().await {
                Ok(parsed) => println!("Success! {:?}", parsed),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Request was unauthorized...");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };

    Ok(())
}

fn duplicate<T>(x: T) -> T { x }

async fn login() -> Result<(), Box<dyn std::error::Error>> {

    let home = match env::var_os("HOME") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$HOME is not set")
    };

    let home = home.to_owned();
    let home_dir = home + "/.bashfull";
    let home_dir2 = duplicate(home_dir);
    fs::create_dir_all(home_dir)?;

    println!("Please visit bashfull.com/profile and paste your api key here.");
    let api_key : String = Input::new()
        .with_prompt("YOUR API KEY HERE")
        .with_initial_text("")
        .interact_text()?;

    fs::write(home_dir2 + "/key.txt", api_key).expect("Unable to save api key.");

    //to add: hit api route to validate key
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Bashfull::from_args() {
        Bashfull::Describe {descript, command} => {
            describe(descript, command).await;
        },
        Bashfull::Recall {descript} => {
            recall(descript).await;
        },
        Bashfull::Login {url} => {
            login().await;
        },
    }   
    Ok(())
}    
