pub fn generate(init: i32, end: i32) {
    println!("FizzBuzz in rust!");
    for count in init..end {
        let mut expression: String = "".to_owned();
        if count % 3 == 0 {
            expression = "Fizz".to_owned();
        }
        if count % 5 == 0 {
            expression = expression + "Buzz"
        }

        if expression == "" {
            expression = format!("{}", count)
        }
        println!("{}", expression);
    }
}
