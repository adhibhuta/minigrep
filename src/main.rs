use std::env;
use std::fs;
use std::io::prelude::*;
use std::process;
use std::error::Error;

struct Args{
    query : String,
    filename: String
}

impl Args{
    fn new(args: &[String]) -> Result<Args, &'static str>{
        if args.len() < 3{return Err("Not enough arguments");}
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Args{query, filename})
    }
}

fn search<'a>(q: &str, contents: &'a str) -> Vec<&'a str>{
    let mut s = vec![];
    for line in contents.lines(){
        if line.contains(q){s.push(line);}
    }
    s
}

fn run(args: Args)-> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string(args.filename)?;

    for line in search(&args.query, &contents){println!("{}", line);}
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args  = Args::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing the args: {}", err);
        process::exit(1)
    });
    
    if let Err(e) = run(args){
        println!("Application error: {}", e);
        process::exit(1);
    }
    
}
