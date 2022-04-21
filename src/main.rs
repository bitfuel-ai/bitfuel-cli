use structopt::StructOpt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    name: String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "bashfull", about = "never forget a bash command again")]
enum Bashfull {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(name = "save")]
    Save {
        #[structopt(short = "d", long = "description")]
        descript: String,

        #[structopt(short = "c", long = "command")]
        command: String,

    },
    #[structopt(name = "query")]
    Query {
        #[structopt(short = "d", long = "description")]
        descript: String,
    },
}

async fn save(descript:String, command:String) -> Result<(), Box<dyn std::error::Error>> {
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
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };

    Ok(())
}

async fn query(descript:String) -> Result<(), Box<dyn std::error::Error>> {
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
            println!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Bashfull::from_args() {
        Bashfull::Save {descript, command} => {
            save(descript, command).await;
        },
        Bashfull::Query {descript} => {
            query(descript).await;
        },
    }   

    Ok(())
}    
