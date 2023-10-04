fn clean_num_chars(inst: &str) -> Vec<f32> {
    let sortee: Vec<char> = inst.chars().collect();
    let mut result = Vec::new();
    for item in sortee {
        match 0 as u32 + item.to_digit(10).unwrap() {
            Ok(r) => result.push(item as f32),
            Err(_) => (),
        }
    }
    return result;
}

fn parse_words(exp: &String) -> Vec<f32> {
    let words: Vec<&str> = exp.trim().split(" ").collect();
    let mut nums: Vec<f32> = Vec::new();
    for str in words {
        nums = clean_num_chars(str);
    }
    return nums;
}

fn word_math_convert(exp: String) -> Option<String> {
    let num_vec: Vec<f32> = parse_words(&exp);

    let mut validity = 0.0;
    for item in &num_vec {
        validity += item;
    }
    if validity == 0.0 {
        println!("This is not a valid entry.");
        return None;
    }

    let num_vec_pure: Vec<&f32> = num_vec.iter().filter(|i| i != &&0.0).collect();

    #[derive(Debug)]
    enum Operator {
        Plus,
        Minus,
        Multiply,
        Divide,
        Exponent,
        Logarithm,
    }

    let mut op_vec: Vec<Operator> = Vec::new();
    for opcheck in exp.split(" ") {
        match opcheck.as_ref() {
            "plus" | "sum" | "add" | "addition" => op_vec.push(Operator::Plus),
            "minus" | "subtraction" | "subtract" | "difference" => op_vec.push(Operator::Minus),
            "multiplied" | "multiplication" | "product" | "multiply" | "times" => {
                op_vec.push(Operator::Multiply)
            }
            "divided" | "divide" | "dividend" | "division" => op_vec.push(Operator::Divide),
            "power" | "exponential" | "exponent" => op_vec.push(Operator::Exponent),
            "logarithm" | "logarithmic" | "log" => op_vec.push(Operator::Logarithm),
            &_ => (),
        }
    }
    let mut runs = 0;
    let mut calc: f32 = 0.0;
    for idx in op_vec {
        let item1 = *num_vec_pure.get(runs)?;
        let item2 = *num_vec_pure.get(runs + 1)?;
        match idx {
            Operator::Plus => calc = item1 + item2,
            Operator::Minus => calc = item1 - item2,
            Operator::Multiply => calc = item1 * item2,
            Operator::Divide => calc = item1 / item2,
            Operator::Exponent => calc = item1.powf(*item2),
            Operator::Logarithm => calc = item1.log(*item2),
        }
        runs += 2;
        if runs >= num_vec_pure.len() {
            break;
        }
    }
    return Some(format!("Calculation : {}", calc));
}

fn main() {
    let mut entry = String::new();
    println!("Please enter a math expression using word notation");
    println!("\n");
    println!(r#"Example : "What is 3 plus 8?""#);
    println!("\n");
    println!("Please enter numbers in number notation only.");
    match std::io::stdin().read_line(&mut entry) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }

    match word_math_convert(entry) {
        Some(result) => println!("{}", result),
        None => println!("Calculation failed."),
    }
}
