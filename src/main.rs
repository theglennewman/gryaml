// fn identify_line_type(str: String) -> String {
    
//     lines can have...
//     key/values
//     multiple docs can be in a yaml file, separated by: ---
//     end of doc can be marked by: ...
//     or the line can be comments
    

//     String::from("it's a line")
// }

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::env;
fn file_to_string() -> String {

    // for now, looking for this file:
    let filename = "example.yaml";
    // ...under current_dir/yaml_files/
    let dirname = "yaml_files";
    let mut file_path = match env::current_dir() {
        Err(why) => panic!("could not determine current dir: {}", why),
        Ok(file_path) => file_path,
    };
    file_path.push(dirname);
    file_path.push(filename);

    // open the yaml file
    let mut yaml_file = match File::open(&file_path) {
        Err(why) => panic!("could not open {}: {}", file_path.display(), why),
        Ok(yaml_file) => yaml_file,
    };

    dbg!(&yaml_file);

    // read the file into a String
    let mut file_contents = String::new();
    match yaml_file.read_to_string(&mut file_contents) {
         Err(why) => panic!("could not read {:?}: {}", file_path.display(), why),
         Ok(_) => println!("done reading {:?}", yaml_file),
    }

    file_contents
}

fn main() {
    // settings for paths? yaml files to play with...

    // we need file i/o

    // open a yaml file

    // parse a yaml file -> loop over lines? id what "kind" of line it is

    // do stuff depending on what kind of line it is

    println!("{}", file_to_string());
    //println!("stringified file... {}", stringy_file);
}
