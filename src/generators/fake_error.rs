// Import rand to randomize file extension and code
use rand::prelude::*;

// Returns a file type for the fake error
pub fn fe_file_extend() -> String {
    let mut rng = rand::rng();

    let fe = [".c", ".cpp", ".js", ".rs", ".cs", ".py"];

    if let Some(ext) = fe.choose(&mut rng) {
        ext.to_string()
    } else {
        String::from(".rs") // fallback if array is empty (shouldn't happen here)
    }
}

// Returns a random piece of code with an error in it
pub fn fe_code_text() -> String {
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
pub fn fe_get_errors() -> String {
    let mut rng = rand::rng();

    let mut nums: Vec<u8> = (1..100).collect();
    nums.shuffle(&mut rng);
    let line_num = nums[0];

    let file = fe_file_extend();
    let code = fe_code_text();

    let error_text = format!("Compiler Error on line {} in file \"code{}\"\n   {}   {}\nVariable \"varable\" has not been declared!",line_num,file,line_num,code);
    return error_text;
}
