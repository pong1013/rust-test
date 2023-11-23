use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    // insert
    scores.insert(String::from("藍隊"), 10);
    // entry: Check whether the key exists.
    scores.entry(String::from("黃隊")).or_insert(50);
    scores.entry(String::from("藍隊")).or_insert(50);

    let team_name = String::from("藍隊");
    // get_score_unwrap
    let score = get_score_unwrap(&scores, &team_name);

    // insert value
    let red_team = String::from("紅隊");
    let r_score = 30;
    scores.insert(red_team, r_score); // insert 後所有權轉移到`hashmap`上

    // for loop
    for (k, v) in &scores {
        println!("{k}: {v}");
    }
}

fn get_score_unwrap(scores: &HashMap<String, i32>, team_name: &String) -> i32 {
    let score = scores.get(team_name).copied().unwrap_or(0);
    println!("{}: {}", team_name, score);

    score
}
