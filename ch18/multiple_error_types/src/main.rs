use std::error;
use std::error::Error as _;
use std::fmt;
use std::num::ParseIntError;

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {}", double_first1(numbers));
    // println!("The first doubled is {}", double_first1(empty));
    // Error 1: the input vector is empty
    // println!("The first doubled is {}", double_first1(strings));
    // Error 2: the element doesn't parse to a number

    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {:?}", double_first2(numbers));
    // println!("The first doubled is {:?}", double_first2(empty));
    // Error 1: the input vector is empty
    // println!("The first doubled is {:?}", double_first2(strings));
    // Error 2: the element doesn't parse to a number

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {:?}", double_first3(numbers));
    println!("The first doubled is {:?}", double_first3(empty));
    println!("The first doubled is {:?}", double_first3(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    print4(double_first4(numbers));
    print4(double_first4(empty));
    print4(double_first4(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    print5(double_first5(numbers));
    print5(double_first5(empty));
    print5(double_first5(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    print6(double_first6(numbers));
    print6(double_first6(empty));
    print6(double_first6(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    print7(double_first7(numbers));
    print7(double_first7(empty));
    print7(double_first7(strings));
}

fn double_first1(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

fn double_first2(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

fn double_first3(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    opt.map_or(Ok(None), |r| r.map(Some))
}

type Result4<T> = std::result::Result<T, DoubleError4>;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
struct DoubleError4;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for DoubleError4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first4(vec: Vec<&str>) -> Result4<i32> {
    vec.first()
        // Change the error to our new type.
        .ok_or(DoubleError4)
        .and_then(|s| {
            s.parse::<i32>()
                // Update to the new error type here also.
                .map_err(|_| DoubleError4)
                .map(|i| 2 * i)
        })
}

fn print4(result: Result4<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// Change the alias to `Box<error::Error>`.
type Result5<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec5;

impl fmt::Display for EmptyVec5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec5 {}

fn double_first5(vec: Vec<&str>) -> Result5<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec5.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) // Converts to Box
                .map(|i| 2 * i)
        })
}

fn print5(result: Result5<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// Change the alias to `Box<dyn error::Error>`.
type Result6<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec6;

impl fmt::Display for EmptyVec6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec6 {}

// The same structure as before but rather than chain all `Results`
// and `Options` along, we `?` to get the inner value out immediately.
fn double_first6(vec: Vec<&str>) -> Result6<i32> {
    let first = vec.first().ok_or(EmptyVec6)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print6(result: Result6<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

type Result7<T> = std::result::Result<T, DoubleError7>;

#[derive(Debug)]
enum DoubleError7 {
    EmptyVec,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError7 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError7::EmptyVec => write!(f, "please use a vector with at least one element"),
            // The wrapped error contains additional information and is available
            // via the source() method.
            DoubleError7::Parse(..) => write!(f, "the provided string could not be parsed as int"),
        }
    }
}

impl error::Error for DoubleError7 {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError7::EmptyVec => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            DoubleError7::Parse(ref e) => Some(e),
        }
    }
}

// Implement the conversion from `ParseIntError` to `DoubleError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `DoubleError`.
impl From<ParseIntError> for DoubleError7 {
    fn from(err: ParseIntError) -> DoubleError7 {
        DoubleError7::Parse(err)
    }
}

fn double_first7(vec: Vec<&str>) -> Result7<i32> {
    let first = vec.first().ok_or(DoubleError7::EmptyVec)?;
    // Here we implicitly use the `ParseIntError` implementation of `From` (which
    // we defined above) in order to create a `DoubleError`.
    let parsed = first.parse::<i32>()?;

    Ok(2 * parsed)
}

fn print7(result: Result7<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        }
    }
}
