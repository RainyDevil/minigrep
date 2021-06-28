use std::error::Error;
use std::fs;
use std::env;
use colored::*;
#[derive(Debug)]
pub struct Config{
    pub query : String,
    pub file : String,
    pub case_insenstive : bool
}

impl Config{
    pub fn new(args : &[String]) -> Result<Config , &'static str>{
        if args.len() < 3{
            return Err("No enough args ！");
        }
        let query = args[1].clone();
        let file = args[2].clone();
        //is_err()只关注有没有环境变量
        let case_insenstive = env::var("CASE_INSENSTIVE").is_err();
        Ok(Config{query, file, case_insenstive})
    }
}

pub fn search<'a>(query : &str, content : & 'a str)->Vec<&'a str>{
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(&query){
            result.push(line);
        }
    }
    result
}
pub fn search_case_insentive<'a>(query : &str, content : & 'a str)->Vec<&'a str>{
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }
    result
}

pub fn run(config : Config)->Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file)?;
    let result = if config.case_insenstive{
        search_case_insentive(&config.query, &contents)
    }
    else{
        search(&config.query, &contents)
    };
    for line in result {
        println!("{}",  line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
    #[test]
    fn one_result_insentive() {
        let query = "dUct";
        let contents = "\
Rust:
safe, fast, proDuctive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, proDuctive."],
            search_case_insentive(query, contents)
        );
    }
}


