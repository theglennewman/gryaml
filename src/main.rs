
/* TODO

to start?
- let's loop over a file, print every line
- then we'll start identifying line types

place to document the yaml rules?
- stuff like... spaces are allowed, tabs are not
*/

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

// trying something: pass a string in here, append cat to it, return it...
fn cat_str(str: &mut String){
    str.push_str(" cat");
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
    let mut stringy = String::from("glen");
    println!("stringy is: {}", stringy);

    let mut i_want_this_many_cats = 3;
    while i_want_this_many_cats !=0 {
        cat_str(&mut stringy);
        i_want_this_many_cats -= 1;
    }

    println!("stringy is now: {}\n", stringy);

    println!("what is stringy anyway...");

    let stringy_is = identify_line_type(stringy);

    println!("... oh right: {}", stringy_is);

    // settings for paths? yaml files to play with...

    // we need file i/o

    // open a yaml file

    // parse a yaml file -> loop over lines? id what "kind" of line it is

    // do stuff depending on what kind of line it is

    //let stringy_file = file_to_string();
    //println!("stringified file... {}", stringy_file);
}
