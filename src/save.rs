use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use std::fs;
use std::env;
use std::path::Path;
use serde_json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Req {
  status: u16,
  headers: HashMap<String, String>,
  body: Option<serde_json::Value>
}

pub async fn save() -> Result<(), Box<dyn std::error::Error>> {

    let _command : String = Input::new()
    .with_prompt("The command to remember")
    .with_initial_text("")
    .interact_text()?;

    let _descript : String = Input::new()
    .with_prompt("A description of the command")
    .with_initial_text("")
    .interact_text()?;


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

    println!("{}", &body.as_ref().unwrap()["message"].as_str().unwrap());

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