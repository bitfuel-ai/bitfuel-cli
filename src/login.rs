use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use std::fs;
use std::env;
use std::path::Path;
use serde_json;
use std::collections::HashMap;

pub async fn login() -> Result<(), Box<dyn std::error::Error>> {

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