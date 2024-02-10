use core::panic;
use std::env::{args, Args};

fn operate(operator: char, first_num:f32, second_num:f32) -> f32 {
    match operator {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '*' | 'X' | 'x' => first_num * second_num,
        '/' => first_num / second_num, 
        _ => panic!("Some error ocurred")
    }
}

fn output(operator: char, first_num:f32, second_num:f32, result:f32) -> String {
    return format!("{} {} {} = {}", first_num, operator, second_num, result);
}

fn main() {
    let mut args: Args = args();

    println!("{:?}", args);
    
    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_num:f32 = first.parse().unwrap();
    let second_num:f32 = second.parse().unwrap();
    let result:f32 = operate(operator, first_num, second_num);
    
    println!("{}", output(operator, first_num, second_num, result));
}
