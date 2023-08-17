use std::env;
use std::process;
use std::fs::File;
use std::io::prelude::*;
// mod security;
// mod sms_mail;

struct  Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str>  {
        
        if args.len() < 3{
            return Err("매개변수가 모자라요");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn main(){
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("File is not exist");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("파일 읽는데 몬가.. 몬가가 잘못됨..");

    println!("내용은 :\n {}", contents);
}