use std::num::ParseIntError;

fn main() {
    let twenty = multiply1("10", "2");
    println!("double is {}", twenty);

    // let tt = multiply1("t", "2"); // panics
    // println!("double is {}", tt);

    // This still presents a reasonable answer.
    let twenty = multiply2("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply2("t", "2");
    print(tt);

    // This still presents a reasonable answer.
    let twenty = multiply3("10", "2");
    print(twenty);

    // The following now provides a much more helpful error message.
    let tt = multiply3("t", "2");
    print(tt);

    print(multiply4("10", "2"));
    print(multiply4("t", "2"));

    print(multiply5("10", "2"));
    print(multiply5("t", "2"));

    print(multiply6("10", "2"));
    print(multiply6("t", "2"));

    // print7(multiply7("10", "2"));
    // print7(multiply7("t", "2"));
}

fn multiply1(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

// fn main() -> Result<(), ParseIntError> {
//     let number_str = "10";
//     let number = match number_str.parse::<i32>() {
//         Ok(number) => number,
//         Err(e) => return Err(e),
//     };
//     println!("{}", number);
//     Ok(())
// }

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// As with `Option`, we can use combinators such as `map()`.
// This function is otherwise identical to the one above and reads:
// Modify n if the value is valid, otherwise pass on the error.
fn multiply3(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

// Define a generic alias for a `Result` with the error type `ParseIntError`.
type AliasedResult<T> = Result<T, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn multiply4(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn multiply5(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number) => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number) => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn multiply6(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

// fn multiply7(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
//     let first_number = try!(first_number_str.parse::<i32>());
//     let second_number = try!(second_number_str.parse::<i32>());

//     Ok(first_number * second_number)
// }

// fn print7(result: Result<i32, ParseIntError>) {
//     match result {
//         Ok(n)  => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }
