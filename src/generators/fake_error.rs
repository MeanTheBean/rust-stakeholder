// Import rand to randomize file extension and code
use rand::prelude::*;

// Returns a file type for the fake error
pub fn FE_FILE_EXTEND() -> String {
    let mut rng = rand::rng();

    let fe = [".c", ".cpp", ".js", ".rs", ".cs", ".py"];

    if let Some(ext) = fe.choose(&mut rng) {
        ext.to_string()
    } else {
        String::from(".rs") // fallback if array is empty (shouldn't happen here)
    }
}

// Returns a random piece of code with an error in it
pub fn FE_CODE_TEXT() -> String {
    let mut rng = rand::rng();

    let ft = [
        "std::cout << varable;",
        "print(varable)",
        "debug.log(varable);",
        "Console.WriteLine(varable);",
        "println!(varable);"
    ];

    if let Some(ext) = ft.choose(&mut rng) {
        ext.to_string()
    } else {
        String::from("varable") // fallback if array is empty (shouldn't happen here)
    }
}

// Get the full fake error
pub fn FE_GET_ERRORS() -> String {
    let mut rng = rand::rng();

    let mut nums: Vec<u8> = (1..100).collect();
    nums.shuffle(&mut rng);
    let line_num = nums[0];

    let file = FE_FILE_EXTEND();
    let code = FE_CODE_TEXT();

    let error_text = format!("Compiler Error on line {} in file \"code{}\"\n   {}   {}\nVariable \"varable\" has not been declared!",line_num,file,line_num,code);
    return error_text;
}
