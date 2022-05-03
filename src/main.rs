use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use std::fs;
use std::env;
use std::path::Path;
use serde_json;
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    name: String,
}

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

async fn describe(_descript:String, _command:String) -> Result<(), Box<dyn std::error::Error>> {
    check_login().await;

    let home = match env::var_os("HOME") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$HOME is not set")
    };

    let home = home.to_owned();
    let home_dir = home + "/.bashfull";

    let api_key_file = home_dir + "/key.txt";

    let key = fs::read_to_string(api_key_file)?;

    
    let client = reqwest::Client::new();
    let response = client
        .get("https://bashfull-server.vercel.app/api/describe".to_owned() + "?token=" + &key + "&descript=" + &_descript + "&command=" + &_command)
        .send().await?;
    //println!("Success! {:?}", response);

    let mut hm = HashMap::new();
    for (key, val) in response.headers().into_iter() {
      hm.insert(key.as_str().to_owned(), val.to_str().ok().unwrap_or("").to_owned());
    }

    let req = Req {status: response.status().as_u16(), body: response.json().await.ok(), headers: hm};
    let body = req.body;

    //let req = await response.json();

    //println!("{}", serde_json::to_string(&req).unwrap_or("".to_owned()));
    println!("{}", serde_json::to_string(&body).unwrap_or("".to_owned()));

    // match response.status() {
    //     reqwest::StatusCode::OK => {
    //         // on success, parse our JSON to an APIResponse
    //         match response.json::<APIResponse>().await {
    //             Ok(parsed) => println!("Success! {:?}", parsed),
    //             Err(_) => println!("Hm, the response didn't match the shape we expected."),
    //         };
    //     }
    //     reqwest::StatusCode::UNAUTHORIZED => {
    //         println!("Request was unauthorized...");
    //     }
    //     other => {
    //         panic!("Uh oh! Something unexpected happened: {:?}", other);
    //     }
    // };

    Ok(())
}

async fn check_login() -> Result<(), Box<dyn std::error::Error>> {
    let home = match env::var_os("HOME") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$HOME is not set")
    };

    let home = home.to_owned();
    let home_dir = home + "/.bashfull";

    let mut rs:bool=true;

    let api_key_file = home_dir + "/key.txt";
    let api_key_file2 = api_key_file.clone();

    rs = Path::new(&api_key_file).exists();

    if rs == false {
        panic!("Your credentials have not been set up - please login with bashfull login.");
    } else {
        
        Ok(())
    }
}

async fn recall(_descript:String) -> Result<(), Box<dyn std::error::Error>> {

    check_login().await;

    let home = match env::var_os("HOME") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$HOME is not set")
    };

    let home = home.to_owned();
    let home_dir = home + "/.bashfull";

    let api_key_file = home_dir + "/key.txt";

    let key = fs::read_to_string(api_key_file)?;

    let client = reqwest::Client::new();

    let url = "https://bashfull-server.vercel.app/api/recall".to_owned() + "?token=" + &key + "&prompt=" + &_descript;
    let response = client
        .get(url)
        .send().await?;
    // println!("Success! {:?}", response);

    let mut hm = HashMap::new();
    for (key, val) in response.headers().into_iter() {
      hm.insert(key.as_str().to_owned(), val.to_str().ok().unwrap_or("").to_owned());
    }

    let req = Req {status: response.status().as_u16(), body: response.json().await.ok(), headers: hm};
    let body = req.body;

    //let req = await response.json();

    //println!("{}", serde_json::to_string(&req).unwrap_or("".to_owned()));
    println!("{}", serde_json::to_string(&body).unwrap_or("".to_owned()));

    // match req.status {
    //     req::status::200 => {
    //         // on success, parse our JSON to an APIResponse
    //         match response.json::<APIResponse>().await {
    //             Ok(parsed) => println!("Recalling command {:?}", parsed),
    //             Err(_) => println!("Hm, the response didn't match the shape we expected."),
    //         };
    //     }
    //     reqwest::StatusCode::UNAUTHORIZED => {
    //         println!("Request was unauthorized...");
    //     }
    //     other => {
    //         panic!("Uh oh! Something unexpected happened: {:?}", other);
    //     }
    // };

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
    let home_dir2 = home_dir.clone();
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
            let res = describe(descript, command).await;
        },
        Bashfull::Recall {descript} => {
            let res = recall(descript).await;
        },
        Bashfull::Login {url} => {
            let res = login().await;
        },
    }   
    Ok(())
}    
