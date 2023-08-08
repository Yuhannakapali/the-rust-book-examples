// hashmaps
use std::collections::HashMap;
fn main() {

    let team1 = String::from("Team Fyki");
    let team2 = String::from("Team Blue");
    let mut scores = HashMap::new();
    scores.insert(team1, 11);
    scores.insert(team2, 22);

    // inset only of there is no value assigned to the key
    scores.entry(String::from("Test")).or_insert(30);
    scores.entry("Test".to_string()).or_insert(100);


    let nameofteam = String::from("Team fyki");
    let scoreofteam = scores.get(&nameofteam);
    match scoreofteam {
        Some(value)=>{
            println!("value is {value}");
        },
        None =>{
            println!("there is no value!")
        }
    }

    // iterating over the hashmap

    // for (k, v ) in scores{
    //     println!("key: {k}, value: {v}")
    // }

    scores.insert("k".to_string(), 11);

    let mut  map = HashMap::new();
    let the_sentence = "this world is the worderfull world".to_string();
    for word in the_sentence.split_whitespace() {
       let count =  map.entry(word).or_insert(0);
       *count += 1;
    }
    
    println!("{:?}", map)
}
