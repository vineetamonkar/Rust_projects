use std::io;
fn main() {

    println!("Enter the input");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if guess == 1 {
        println!("1")
    } else if guess == 2 {
        println!("1")
    } else {
        //use big int.
        let mut num1 = 1;
        let mut num2 = 1;
        let mut count = 2;
        let mut temp;
        while count<guess {
            temp = num2;
            num2 = num2 + num1;
            num1 = temp;
            count = count+1;
        }
        println!("{}",num2);
        
    }

}
