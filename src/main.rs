use clap::{Arg, Command};
use regex::Regex;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let matches = Command::new("Pickle Formatter")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Formats Nickel code into various target languages")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Input .ncl file containing Nickel code")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FORMAT")
            .help("Output format (lua, c, cpp, js, go, python)")
            .required(true))
        .get_matches();

    let input_file = matches.get_one::<String>("input").expect("Input file is required");
    let output_format = matches.get_one::<String>("output").expect("Output format is required");

    let nickel_code = match read_file(input_file) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
            return;
        }
    };

    let formatted_code = match output_format.as_str() {
        "lua" => convert_to_lua(&nickel_code),
        "c" => convert_to_c(&nickel_code),
        "cpp" => convert_to_cpp(&nickel_code),
        "js" => convert_to_js(&nickel_code),
        "go" => convert_to_go(&nickel_code),
        "python" => convert_to_python(&nickel_code),
        _ => {
            eprintln!("Unsupported format: {}", output_format);
            return;
        }
    };

    println!("{}", formatted_code);
}

fn read_file(path: &str) -> std::io::Result<String> {
    let path = Path::new(path);
    let mut file = fs::File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn convert_to_lua(nickel_code: &str) -> String {
    let mut lua_code = String::from("-- Lua code\n");
    
    let re = Regex::new(r"let (\w+) = (\d+);").unwrap();
    lua_code += &re.replace_all(nickel_code, |caps: &regex::Captures| {
        let var = &caps[1];
        let val = &caps[2];
        format!("local {} = {}\n", var, val)
    }).to_string();
    
    lua_code
}

fn convert_to_c(nickel_code: &str) -> String {
    let mut c_code = String::from("#include <stdio.h>\n\n");
    
    let re = Regex::new(r"let (\w+) = (\d+);").unwrap();
    c_code += &re.replace_all(nickel_code, |caps: &regex::Captures| {
        let var = &caps[1];
        let val = &caps[2];
        format!("int {} = {};\n", var, val)
    }).to_string();
    
    c_code += "\nint main() {\n    printf(\"Program output\\n\");\n    return 0;\n}\n";
    
    c_code
}

fn convert_to_cpp(nickel_code: &str) -> String {
    let mut cpp_code = String::from("#include <iostream>\n\n");
    
    let re = Regex::new(r"let (\w+) = (\d+);").unwrap();
    cpp_code += &re.replace_all(nickel_code, |caps: &regex::Captures| {
        let var = &caps[1];
        let val = &caps[2];
        format!("int {} = {};\n", var, val)
    }).to_string();
    
    cpp_code += "\nint main() {\n    std::cout << \"Program output\" << std::endl;\n    return 0;\n}\n";
    
    cpp_code
}

fn convert_to_js(nickel_code: &str) -> String {
    let mut js_code = String::from("// JavaScript code\n");
    
    let re = Regex::new(r"let (\w+) = (\d+);").unwrap();
    js_code += &re.replace_all(nickel_code, |caps: &regex::Captures| {
        let var = &caps[1];
        let val = &caps[2];
        format!("let {} = {};\n", var, val)
    }).to_string();
    
    js_code += "\nconsole.log(\"Program output\");\n";
    
    js_code
}

fn convert_to_go(nickel_code: &str) -> String {
    let mut go_code = String::from("package main\n\nimport \"fmt\"\n\nfunc main() {\n");
    
    let re = Regex::new(r"let (\w+) = (\d+);").unwrap();
    go_code += &re.replace_all(nickel_code, |caps: &regex::Captures| {
        let var = &caps[1];
        let val = &caps[2];
        format!("    var {} = {}\n", var, val)
    }).to_string();
    
    go_code += "    fmt.Println(\"Program output\")\n}\n";
    
    go_code
}

fn convert_to_python(nickel_code: &str) -> String {
    let mut python_code = String::from("# Python code\n");
    
    let re = Regex::new(r"let (\w+) = (\d+);").unwrap();
    python_code += &re.replace_all(nickel_code, |caps: &regex::Captures| {
        let var = &caps[1];
        let val = &caps[2];
        format!("{} = {}\n", var, val)
    }).to_string();
    
    python_code += "\nprint(\"Program output\")\n";
    
    python_code
}
