// rustc --cfg some_condition custom.rs && ./custom

fn main() {
    conditional_function();
}

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}
