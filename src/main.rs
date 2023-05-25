fn main() {
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
    println!("{}", s4)
}
