fn main(){
    println!("🎯 Option & Result Handling in Rust");

    simple_option();
    create_option();

    another_option();
    option_methods();
    option_methods_two();

    result_example();
    result_parsing_loop();
    result_states_and_methods();
    test_operator();
}

// Option Examples
fn simple_option(){
    println!("\nSimple Option Example");

    let debansu_num: Option<String> = get_phn_num("Debansu");
    let debadutta_num = get_phn_num("Debadutta");

    println!("\nDebansu's Phone number: {:?}", debansu_num);
    println!("Debadutta's Phone number: {:?}\n", debadutta_num);

    match debansu_num {
        Some(num) => println!("Debansu's Number: {num}"),
        None => println!("No Number Found For Debansu."),
    }

    match debadutta_num {
        Some(num) => println!("Debadutta's Number: {num}"),
        None => println!("No Number Found For Debadutta."),
    }
}

fn get_phn_num(name: &str) -> Option<String> {
    match name {
        "Debansu" => Some("9114472604".to_string()),
        "Debadutta" => Some("1234567890".to_string()),
        "Das" => Some("0987654321".to_string()),
        _ => None,
    }
}

fn create_option(){
    println!("\nCreating Option Example");

    let has_val = Some(42);
    let no_val: Option<i32> = None;

    // If let syntax
    if let Some(num) = has_val {
        println!("Has Value: {num}") // Can skip the semicolon
    }

    // If let else syntax
    if let Some(num) = no_val {
        println!("Should Not Print: {num}")
    } else {
        println!("No Value Present")
    }
}

fn another_option(){
    println!("\nOption With Vector Access");

    let nums = vec![10, 20, 30, 40, 50];
    println!("Vector: {:?}", nums);

    // Accessing elements via indexing
    let first = nums[0];
    // let invalid = nums[10];

    println!("\nFirst Number: {first}");
    // println!("Invalid Number: {invalid}"); // This will panic

    // Accessing elements via get method
    let second = nums.get(1);
    let invalid = nums.get(10);

    println!("\nSecond Number: {:?}", second);
    println!("Invalid Number: {:?}", invalid);
}

fn option_methods(){
    println!("\nOption Methods");

    let some_num = Some(42);
    let no_num: Option<i32> = None;

    println!("Is Some: {}", some_num.is_some());
    println!("Is None: {}", no_num.is_none());
}

fn option_methods_two() {
    println!("\nOption Unwrap Variants");

    let val = Some(42);
    let none: Option<i32> = None;

    // unwrap_or returns the value inside Some, or the provided default if None
    println!("\nUnwrap Or Default (Some): {}", val.unwrap_or(0));
    println!("Unwrap Or Default (None): {}", none.unwrap_or(0));

    // expect is like unwrap, but with a custom panic message if None
    println!("\nUnwrap With Expect: {}", val.expect("Expected A Number."));

    // unwrap_or_else lazily computes the default value if None
    let lazy_default = none.unwrap_or_else(|| {
        println!("\nCalculating Fallback Value...");
        100
    });
    println!("Unwrap Or Else Result: {lazy_default}");

    // unwrap_or_default returns the contained value or the type's default
    let default_string: Option<String> = None;
    println!("\nDefault String: {}", default_string.unwrap_or_default());

    // map applies a function to the value inside Some, if present
    let mapped = val.map(|x| x * 2);
    println!("\nMapped Value (Double): {:?}", mapped);

    // filter returns Some if the predicate is true, otherwise None
    let filtred = val.filter(|x| *x > 50);
    println!("\nFiltred (>50): {:?}", filtred);
}

// Result Examples
fn result_example(){
    println!("\nResult Example");

    let good_parse = "42".parse::<i32>();
    let bad_parse = "abc".parse::<i32>();

    println!("\nGood Parse Result: {:?}", good_parse);
    println!("Bad Parse Result: {:?}\n", bad_parse);

    // enum Result<i32, ParseIntError> {
    //     Ok(i32),  // Operation is success and return ok with type T( T can be any type)
    //     Err(ParseIntError)  // Operation is failed and return err with type E( E can be any type NotfoundError, ParseIntError, etc.)
    // } // Internal Working of parse::<i32>()

    match good_parse { // Gets partially moved
        Ok(value) => println!("Parsed Value: {value}"),
        Err(e) => println!("Failed To Parse: {e}"),
    }

    if let Ok(val) = bad_parse {
        println!("Parsed Value: {val}")
    } else {
        println!("Failed To Parse")
    }
}

fn result_parsing_loop(){
    println!("\nResult Parsing Loop");

    println!("\nParsing Test");
    let parsing_test = vec!["42", "abc", "100", "xyz"];

    for text in parsing_test{
        match text.parse::<i32>(){
            Ok(val) => println!("Parsed Value: {val}"),
            Err(e) => println!("Failed To Parse: {e}"),
        }
    }

    print!("\nMath Test");
    let math_test = vec![
        ("10 / 2", safe_division(10, 2)),
        ("10 / 0", safe_division(10, 0)),
        ("20 / 5", safe_division(20, 5)),
        ("20 / 0", safe_division(20, 0))
    ];

    for (description, result) in math_test {
        match result{
            Ok(val) => println!("{description} = {val}"),
            Err(e) => println!("Error In {description} = {e}"),
        }
    }
}

fn safe_division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Zero Division Error!!!".to_string())
    } else {
        Ok(a / b)
    }
}

fn result_states_and_methods(){
    println!("\nResult States & Methods");

    let ok_val: Result<i32, &str> = Ok(10);
    let err_val: Result<i32, &str> = Err("Failed");

    println!("\nResult States");
    println!("ok_val.is_ok(): {:?}", ok_val.is_ok());
    println!("ok_val.is_err(): {:?}", ok_val.is_err());
    println!("err_val.is_ok(): {:?}", err_val.is_ok());
    println!("err_val.is_err(): {:?}", err_val.is_err());

    println!("\nUnwrap Or");
    println!("ok_val.unwrap_or(0): {:?}", ok_val.unwrap_or(0));
    println!("err_val.unwrap_or(0): {:?}", err_val.unwrap_or(0));

    // map & mapp_err
    let mapped = ok_val.map(|x| x * 2);
    println!("\nMapped Result: {:?}", mapped);

    let mapped_er = err_val.map_err(|e| format!("Custom Error: {:?}", e));
    println!("\nMapped Error: {:?}", mapped_er);

    // and_then chaining
    let chained = Ok(20).and_then(|x| safe_division(x, 2));
    println!("\nChained Result: {:?}", chained);
}

// ? Operator
fn parsing_num(text: &str) -> Result<i32, String> {
    match text.parse::<i32>(){
        Ok(val) => Ok(val),
        Err(_) => Err("Failed To Parse Number".to_string())
    }
    // Or text.parse::<i32>().map_err(|_| "Failed To Parse Number".to_string())
}

fn double_string_num(text: &str) -> Result<i32, String> {
    let num = parsing_num(text)?;
    Ok(num * 2)

    // If ? not used then

    // Err(format!("{}", num * 2));
    // match num {
    //     Ok(val) => Ok(val * 2),
    //     Err(e) => Err(e)
    // }
    // let double_num_str = (num * 2).to_string();
    // println!("Double String Number: {double_num_str}");
}

fn test_operator(){
    println!("\nUsing the ? Operator");

    println!("\nWith parsing_num()"); 
    match parsing_num("42") {
        Ok(val) => println!("Parsed Value: {val}"),
        Err(e) => println!("Error: {e}"),
    }

    match parsing_num("abc") {
        Ok(val) => println!("Parsed Value: {val}"),
        Err(e) => println!("Error: {e}"),
    }

    println!("\nWith double_string_num()"); 
    match double_string_num("42") {
        Ok(val) => println!("Double Value: {val}"),
        Err(e) => println!("Error: {e}"),
    }

    match double_string_num("abc") {
        Ok(val) => println!("Double Value: {val}"),
        Err(e) => println!("Error: {e}"),
    }
}