use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    let args :Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Parase failed : {}", err);
        process::exit(1);
    }); 
   // println!("{:?}", config);
    if let Err(err) = run(config){
        eprintln!("{}", err);
    }
}
