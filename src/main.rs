use regex::Regex;

fn main() {
    // Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_subs = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();

    println!("Ingresa tu expresion: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // multiplicacion
    loop {

        let caps = re_mult.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
    
        let caps = caps.unwrap();
    
        let cap_expression:&str = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
        let multiplication = left_value * right_value;
        
        expression = expression.replace(cap_expression, &multiplication.to_string())
        }

    // suma
    loop {

    let caps = re_add.captures(expression.as_str());
    if caps.is_none() {
        break;
    }

    let caps = caps.unwrap();

    let cap_expression:&str = caps.get(0).unwrap().as_str();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    let addition = left_value + right_value;
    
    expression = expression.replace(cap_expression, &addition.to_string())
    }

    loop {

        let caps = re_subs.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
    
        let caps = caps.unwrap();
    
        let cap_expression:&str = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
        let subtraction = left_value + right_value;
        
        expression = expression.replace(cap_expression, &subtraction.to_string())
        }

loop {
    let caps = re_div.captures(&expression);
    if caps.is_none() {
        break;
    }
    
    let caps = caps.unwrap();
    
    let cap_expression = caps.get(0).unwrap().as_str();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let division = left_value / right_value; // Asumiendo que quieres dividir de esta manera
    
    expression = expression.replace(cap_expression, &division.to_string());
}
   println!("result {}", expression.trim())
}
