fn main() {
    for i in 1..51 {
        let fizz = (i % 3) == 0;
        let buzz = (i % 5) == 0;

        let mut output = String::new();

        if fizz || buzz {
            if fizz {
                output.push_str("fizz");
            }
            if buzz {
                output.push_str("buzz");
            }
        } else {
            output = i.to_string();
        }

        println!("{}", output);
    }
}
