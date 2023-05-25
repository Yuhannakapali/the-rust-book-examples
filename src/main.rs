fn main() {
    let mut my_mark: Vec<i32> = Vec::new();
    my_mark.push(21);
    my_mark.push(222);
    // my_mark.push(-22);

    let third_element = my_mark.get(2);

    let trash = my_mark[0];

    print!("trash is {}", trash);

    match third_element {
        Some(third) => {
            print!("the third element is {} ", third);
        }
        None => {
            print!("the third element doesn't exist")
        }
    }

    // using macro to implement a vec

    let mac_prices = vec![1200, 1000, 850];
    //  after using you can use both get method or access using the indexes
}
