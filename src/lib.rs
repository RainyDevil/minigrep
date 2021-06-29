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
#[derive(Debug)]
pub struct Line<'a>{
    line : Vec<&'a str>,//出现匹配字符的行
    index : Vec<usize>,//记录字符第一次出现位置的索引
    line_num : Vec<u64>,//匹配单词出现在第几行
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

pub fn search<'a>(query : &str, content : & 'a str)->Line<'a>{
    let mut result = Vec::new();
    let mut index : Vec<usize> = Vec::new();
    let mut line_num : Vec<u64> = Vec::new();
    let mut cnt : u64 = 1;
    for line in content.lines() {
        if line.contains(&query){
            match line.find(&query){
                None => (),
                Some(i) => index.push(i),
            }
            result.push(line);
            line_num.push(cnt);
        }
        cnt += 1;
    }
    Line{line : result, index : index, line_num : line_num}
}
pub fn search_case_insentive<'a>(query : &str, content : & 'a str)->Line<'a>{
    let query = query.to_lowercase();
    let mut result = Vec::new();
    let mut index : Vec<usize> = Vec::new();
    let mut line_num : Vec<u64> = Vec::new();
    let mut cnt : u64 = 1;
    for line in content.lines(){
        if line.to_lowercase().contains(&query){
            match line.to_lowercase().find(&query){
                None => (),
                Some(i) => index.push(i),
            }
            result.push(line);
            line_num.push(cnt);
        }
        cnt += 1;
    }
    Line{line : result , index : index, line_num : line_num}
}
//TODO 每行所有词匹配
pub fn run(config : Config)->Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file)?;
    let  lines = if config.case_insenstive{
        search_case_insentive(&config.query, &contents)
    }
    else{
        search(&config.query, &contents)
    };
    for index in (0..lines.line.len()) {
        let line = lines.line[index];
        let idx = &lines.index[index];
        let length : usize = config.query.len();
        let front = &line[..*idx];
        let bold = &line[*idx..(length+*idx)].red().bold();
        let end = &line[(*idx+length)..];
        let header = lines.line_num[index];
        println!("--{}:{}{}{}", header, front, bold, end);
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


