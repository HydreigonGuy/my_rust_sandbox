
pub fn handle_request(input: String) {
    let vec_input: Vec<&str> = input.split(' ').collect();
    
    if vec_input.len() == 3 {
        match (vec_input[0].parse::<i32>(), vec_input[1], vec_input[2].parse::<i32>()) {
            (Ok(num1), "+", Ok(num2)) => println!("{}", num1 + num2),
            (Ok(num1), "*", Ok(num2)) => println!("{}", num1 * num2),
            (Ok(num1), "-", Ok(num2)) => println!("{}", num1 - num2),
            (Ok(num1), "/", Ok(num2)) => println!("{}", num1 / num2),
            _ => println!("Invalid request.")
        }
    } else {
        println!("Invalid request.")
    }
}
