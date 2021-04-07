fn identify_line_type(str: String) -> String {
    /*
    lines can have...
    key/values
    multiple docs can be in a yaml file, separated by: ---
    end of doc can be marked by: ...
    or the line can be comments
    */

    String::from("it's a line")
}

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
fn file_to_string() -> String {
    // we could probably get the $(pwd) out of std::env::current_dir()
    // but for now let's just hard code it...
    
    //lol I don't even understand string concatenation
    //let yaml_dir = "/mnt/vmshares/q_win10_share/gryaml/yaml_files"

    //just hardcode the whole dumb file path
    let file_path = 
      Path::new("/mnt/vmshares/q_win10_share/gryaml/yaml_files/example.yaml");
    let display = file_path.display();

    println!("try to open: {}", display);
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("could not read {}: {}", display, why),
        Ok(_) => print!("{} contents...\n{}", display, s),
    }

    s
}

fn main() {
    // settings for paths? yaml files to play with...

    // we need file i/o

    // open a yaml file

    // parse a yaml file -> loop over lines? id what "kind" of line it is

    // do stuff depending on what kind of line it is

    //let stringy_file = file_to_string();
    //println!("stringified file... {}", stringy_file);
}
