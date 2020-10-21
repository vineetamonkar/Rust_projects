use std::io;

fn main() {
    println!("Enter two numbers");
    let num1 = read_number();
    let num2 = read_number();
    let sum: i64 = sum_2(num1,num2);
    println!("The sum is {}",sum);
}

fn sum_2(num1:i64,num2:i64) -> i64 {
    num1+num2
}

fn read_number() -> i64 {
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("error");
    let num1 : i64 = num1.trim().parse().expect("error");
    num1
}
