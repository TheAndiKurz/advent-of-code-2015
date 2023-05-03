use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn get_data(path_str: &str) -> String {
    // create a path to the file
    let path = Path::new(path_str);
    let display = path.display();

    // open the path in read-only mode
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut data = String::new();

    if let Err(why) = file.read_to_string(&mut data) {
        panic!("couldn't read {}: {}", display, why)
    }

    data
}
