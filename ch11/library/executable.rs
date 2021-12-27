// # Where library.rlib is the path to the compiled library, assumed that it's
// # in the same directory here:
// $ rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable

// extern crate rary; // May be required for Rust 2015 edition or earlier

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}
