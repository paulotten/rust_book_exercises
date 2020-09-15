This is an implementation [Fizz buzz](https://en.wikipedia.org/wiki/Fizz_buzz).

I dislike "Rust By Example"'s [version](https://doc.rust-lang.org/rust-by-example/flow_control/while.html) for two reasons  
1) It has 4 different println! macro calls.  
2) It has a divide by 15 check. A cleanly written version should only have divide by 3 and 5 checks. Imagine if we need to add a divide by 7 check. The permutations quickly get out of hand.