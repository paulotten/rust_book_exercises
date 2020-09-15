fn main() {
    let number = 7;
    let number2 = if number < 5 {
        println!("condtion was true");
        1
    } else {
        println!("condtion was false");
        2
    };
    
    println!("The value of number2 is: {}", number2);
}
