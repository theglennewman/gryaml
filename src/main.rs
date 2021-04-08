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
    let correct_filename =
        "/mnt/vmshares/q_win10_share/gryaml/yaml_files/example.yaml";
    println!("file path should be:\n\t{}", correct_filename);

    // we could probably get the $(pwd) out of std::env::current_dir()
    // but for now let's just hard code it...

    //hmmm.... current_dir returns a "std::result::Result<PathBuf, std::io::Error"
    // which does not have a display field... So I'm supposed to be doing something with this
    // more properly...
    let cur_dir = std::env::current_dir().display;
    println!("current_dir is: {}", cur_dir.display());
    
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

    // read file into s
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("could not read {}: {}", display, why),
    //     Ok(_) => print!("{} contents...\n{}", display, s),
    // }

    // or just hard code it...
    let s = String::from("<totally file contents>");

    s
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
