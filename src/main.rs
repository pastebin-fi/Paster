extern crate copypasta;

use copypasta::ClipboardContext;
use std::env;
use std::fs;
use std::collections::HashMap;
use std::path::Path;

use reqwest::blocking::Client;

fn main() {
    let args: Vec<String> = env::args().collect();

    // is filename supplied?
    if args.len() < 2 {
        return;
    }

    let filename = &args[1];

    // is the file real?
    if !Path::new(filename).exists() {
        return;
    }
    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    
    let mut paste = HashMap::new();
    let hidden = "on".to_owned();

    paste.insert("title", filename);
    paste.insert("paste", &contents);
    paste.insert("hidden", &hidden);


    let client = Client::new();

    let resp = client.post("https://pastebin.fi/new")
        .form(&paste)
        .send()
        .unwrap();

    let location = resp.url().as_str();

    println!("{}", location);

    let mut ctx = ClipboardContext::new().unwrap();
    println!("{:?}", ctx.get_contents());
    ctx.set_contents("some string".to_owned()).unwrap();
}
