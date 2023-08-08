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
            println!("the third element is {} ", third);
        }
        None => {
            println!("the third element doesn't exist")
        }
    }

    // using macro to implement a vec

    let mut mac_prices = vec![1200, 1000, 850];
    //  after using you can use both get method or access using the indexes

    // iteration over vec

    for i in &mac_prices {
        println!("mac prices are as followings:  {}", i)
    }

    // iterating to change the value
    for i in &mut mac_prices {
        *i += 50;
    }

    println!("new Mac Prices after increase are: {:?}", mac_prices);

    // using enum to hold the multiple type of data in vec

    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let mut row1 = Vec::new();

    row1.push(SpreadsheetCell::Text(String::from("Hello My friend")));
    row1.push(SpreadsheetCell::Float(22.23));

    let mut _row2 = vec![
        SpreadsheetCell::Int(11),
        SpreadsheetCell::Float(22.22),
        SpreadsheetCell::Text(String::from("Hello there Vec macro")),
    ];

    // using match to get the value from enum
    match row1[1] {
        SpreadsheetCell::Int(value) => {
            println!("the value is {}", value);
        }
        _ => {
            println!("the value is not float");
        }
    }
}
