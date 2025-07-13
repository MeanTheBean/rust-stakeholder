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

// "Variable \"varable\" has not been declared!";



