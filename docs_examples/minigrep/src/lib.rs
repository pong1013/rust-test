use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> 代表函式會回傳有實作 Error 特徵的型別，
    // 但我們不必指定回傳值的明確型別。這增加了回傳錯誤數值的彈性，
    // 其在不同錯誤情形中可能有不同的型別。dyn 關鍵字是
    // 「動態（dynamic）」的縮寫。
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn parse_config(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        // 第一個引數會是我們要搜尋的字串
        // 第二個引數會是檔案路徑

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        // 第三個變數為判別是否為 sensitive case
        // 我們可以呼叫 env::var 函式並傳入環境變數 IGNORE_CASE 的名稱。如果有設置環境變數的話，這就會是包含環境變數數值的成功 Ok變體；如果環境變數沒有設置的話，這就會回傳 Err 變體
        // is_ok 方法來檢查區分大小寫的搜尋是否設置。如果 IGNORE_CASE 環境變數沒有設置任何數值的話，is_ok 會回傳否，所以程式就會進行區分大小寫的搜尋。我們不在乎環境變數的數值，只在意它有沒有被設置而已，所以我們使用 is_ok 來檢查而非使用 unwrap、expect 或其他任何我們看過的 Result 方法。
        // ⭐️設置方法： `IGNORE_CASE=1 cargo run -- to data/poem.txt`
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<_> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<_> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "RuSt";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
