use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec;
use std::fmt::Debug;

//There is so much happening here. I just kinda added stuff until the
//compiler stopped yelling at me about it.
fn type_of<T: ?Sized>(_: &T) -> &str {
    std::any::type_name::<T>()
}
// I like the dbg! macro, but sometimes I want to see a variable's TYPE
fn gdbg<T: Debug>(x: &[T]) {
    dbg!(&x);
    println!("  ^-- type might be: {}", type_of(&x));
}

// fn identify_line_type(str: String) -> String {
//     lines can have...
//     key/values
//     multiple docs can be in a yaml file, separated by: ---
//     end of doc can be marked by: ...
//     or the line can be comments
//
//     String::from("it's a line")
// }

// relative to: <current_dir>/yaml_files/
fn open_gryaml_file(filename: &str) -> File {

    let dirname = "yaml_files";
    let mut file_path = match env::current_dir() {
        Err(why) => panic!("could not determine current dir: {}", why),
        Ok(file_path) => file_path,
    };
    file_path.push(dirname);
    file_path.push(filename);

    // dbg!(&file_path);

    let mut yaml_file = match File::open(&file_path) {
        Err(why) => panic!("could not open {}: {}", file_path.display(), why),
        Ok(yaml_file) => yaml_file,
    };

    yaml_file
}

//get my file's contents as a String
fn gryaml_file_string() -> String {
    //open the file
    let mut yaml_file = open_gryaml_file("example.yaml");

    // read the file into a String
    let mut file_contents = String::new();
    match yaml_file.read_to_string(&mut file_contents) {
        Err(why) => panic!("could not read {:?}: {}", yaml_file, why),
        Ok(_) => println!("done reading {:?}", yaml_file),
    }

    file_contents
}

//lines in the file are printed and loaded into a Vector of Strings
fn gryaml_file_lines_to_vector(f: File) -> Vec<String> {
    let reader = BufReader::new(f);
    let mut vec: Vec<String> = Vec::new();
    
    for line in reader.lines() {
        println!("{:?}", line);
    }

    //placeholder
    vec!(String::from("yo this is your vector"))
}

fn main() {
    let my_gryaml_file = open_gryaml_file("example.yaml");

    let my_vec = gryaml_file_lines_to_vector(my_gryaml_file);

    gdbg(&my_vec);

    // parse a yaml file -> loop over lines? id what "kind" of line it is

    // do stuff depending on what kind of line it is

    //println!("{}", gryaml_file_string());
    
    //gryaml_file_string();
}
