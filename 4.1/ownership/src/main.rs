fn _strings() {
    let mut s = String::from("hello");
    
    s.push_str(", world!");
    
    println!("{}", s);
}

fn _s2() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn _s3() {
    let s1 = "hello";
    let s2 = s1;

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn main() {
    //_strings();
    //_s2();
    _s3();
    
    // Ownership and Functions
}
