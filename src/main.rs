use structopt::StructOpt;

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

// async fn save(descript:String, command:String) -> Result<() >> {
//     let client = reqwest::Client::new();
//     let resp = client.get("http://api.roboflow.com")
//     .send()
//     .await?;

//     dbg! resp

//     println!("saving : {}", descript);

//     if resp.status() != 200 {
//         println!("failed to submit note");
//         return Ok(());
//     }

//     Ok(())
// }

// async fn query(descript:String) {
//     println!("querying : {}", descript);
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let client = reqwest::Client::new();
    let resp = client.get("https://bashfull-server.vercel.app/api/test")
        .send()
        .await?.text().await;

    // if resp.status() != 200 {
    //     println!("failed to submit note");
    //     return Ok(());
    // }


    println!("{:?}", resp);

    Ok(())
}    


// #[tokio::main]
// fn main() {
//     match Bashfull::from_args() {
//         Bashfull::Save {descript, command} => {
//             save(descript, command);
//         },
//         Bashfull::Query {descript} => {
//             query(descript);
//         }
//         _ => (),
//     }
//     //println!("{:?}", opt.Save);
// }
