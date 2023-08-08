use unicode_segmentation::UnicodeSegmentation;


fn main() {

    let mut s = "hello".to_string();
    s.push_str(", world!");
    println!("{}", s);

    //  updating string
    let mut string_to_modify = "नमस्ते".to_string();
    string_to_modify.push_str(" Nepal");
    println!("{}", string_to_modify);

    // using macro to do string manipulation

    let s1 = "Hello ".to_string();
    let s2 = "World !!!".to_string();

    // here + act as an a macro which work as following
    // fn add(self, s: &str) -> String {
    let s3 = s1 + &s2;

    println!("{}", s3);

    // macro for the  string
    // format!();

    let tic = String::from("Tick");
    let tack = String::from("Tack");
    let toe = String::from("Toe");

    let s4 = format!("{tic} - {tack} - {toe}");
    println!("{}", s4);


    // accessing the individual character of the string
    let  string_to_modify = "नमस्ते".to_string();
    
    // bytes value
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    // 224, 165, 135]

    // scaler value
    // ['न', 'म', 'स', '्', 'त', 'े']

    // grapheme clusters
    // ["न", "म", "स्", "ते"]


    // accessing the individual character of the string in bytes
    
    print!("bytes value\n");
    for b in string_to_modify.bytes() {
        println!("{}", b);
    }
    println!("character cluter \n");
    // accessing the individual character of the string in scaler value
    for c in string_to_modify.chars() {
        println!("{}", c);
    }

    // accessing the individual character of the string in grapheme clusters
    // for g in string_to_modify.graphemes(true) {
    //     println!("{}", g);
    // }
    // this doesn't work in rust too keep he code lean 
    // we have to use external crate unicode-segmentation
    
    print!("Grapheme clusters: \n");

    for g in string_to_modify.graphemes(true) {
        println!("{}", g);
    }


}
