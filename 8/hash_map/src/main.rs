use std::collections::HashMap;

fn main() {
    let mut score = HashMap::new();

    score.insert(String::from("A"), 4);
    score.insert(String::from("B"), 0);

    let team_name = String::from("B");
    println!("A: {}", score.get("A").unwrap());
    println!("B: {}", score.get(&team_name).unwrap());

    println!("{:?}", score);
    println!("{:?}", score);
}
