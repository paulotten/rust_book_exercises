fn _loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn _while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn _for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn _for2() {
    for number in (1..4).rev() {
        // KISS...
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn main() {
    //_loop();
    //_while();
    //_for();
    _for2();
}
