#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

// $ rustc --cfg some_condition custom.rs && ./custom

fn main() {
    conditional_function();
    if cfg!(some_condition) {
        println!("met")
    } else {
        println!("not met")
    }
}
