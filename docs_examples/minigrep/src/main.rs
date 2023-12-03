use minigrep::Config;
use std::env;
use std::process;
fn main() {
    // 1. 讀取命令引數
    let args: Vec<String> = env::args().collect();
    // env::args() 是 iterator，返回的迭代器內的元素是 String
    // collect() 則可以將 env::args() 產生的字符串迭代器的所有元素收集到一个 Vec<String> 中
    dbg!(&args);

    // 2. 儲存引數
    let config = Config::parse_config(&args).unwrap_or_else(|err| {
        eprintln!("解析引數時出現問題：{err}");
        process::exit(1);
        // process::exit 函式會立即停止程式並回傳給予的數字來作為退出狀態碼。
        // 這與 panic! 來處理的方式類似，但我們不再顯示多餘的輸出結果。
    });

    // println!("搜尋 {}", config.query);
    // println!("目標檔案為 {}", config.file_path);

    // 3.讀取檔案
    if let Err(e) = minigrep::run(config) {
        eprintln!("應用程式錯誤：{e}");
        process::exit(1);
    }
    // run 函式沒有回傳數值，所以我們不必像處理 Config::build 得用 unwrap 取得 Config
}
