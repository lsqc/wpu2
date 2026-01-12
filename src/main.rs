use std::fs::File;
use std::io;

use std::path;
use std::path::PathBuf;
use std::process;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut url: String = String::new();
    let mut dest: String = String::new();

    if args.len() > 2 {
        // great, we've got enough arguments
        url = args.get(1).expect("something fucky happened [1]").to_string();
        dest = args.get(2).expect("something fucky happened [2]").to_string();
    }
    else {
        println!("usage: {} <url> <dest>", args.get(0).unwrap().to_string());
        process::exit(0x1);
        
    }

    let path_buf: PathBuf = PathBuf::from(&dest);

    let absolute_dest = path::absolute(&path_buf).expect("could not create absolute path");
    let absolute_path = absolute_dest.display();

    // debug
    println!("debug: absolute path: {}", &absolute_path);
    println!("saving {} to {}", url, &absolute_path);

    // get
    let resp = reqwest::get(url.to_string())
        .await
        .expect("could not complete request");

    let body = resp.text().await.expect("response body invalid");
    let mut out = File::create(&dest).expect("could not create file");

    // save file
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
}
