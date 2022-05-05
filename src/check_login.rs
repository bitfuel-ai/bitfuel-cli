use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use dialoguer::Input;
use std::fs;
use std::env;
use std::path::Path;
use serde_json;
use std::collections::HashMap;

pub async fn check_login() -> Result<(), Box<dyn std::error::Error>> {
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