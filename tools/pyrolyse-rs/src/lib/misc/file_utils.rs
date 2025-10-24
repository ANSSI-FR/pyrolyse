use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// pub fn file_double<P: AsRef<Path>>(file_path: P) -> Result<String, String> {
//     let mut file = try!(File::open(file_path).map_err(|e| e.to_string()));
//     let mut contents = String::new();
//     try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
//     // let lines = contents.lines();
//     Ok(contents)
// }

pub fn write_in_file_path_s(path_s: &str, s: &str) {
    let path = Path::new(&path_s);
    let mut f = match File::create(path_s) {
        Ok(f) => f,
        Err(err) => {
            println!("Error creating file {:?}: {}", path_s, err);
            match File::open(path) {
                Ok(f) => f,
                Err(err) => panic!("Error opening file {:?}: {}", path, err),
            }
        }
    };
    match f.write_all(s.as_bytes()) {
        Ok(f) => f,
        Err(err) => panic!("Error writing data: {}", err),
    };
}

pub fn write_in_file_path(path: &Path, s: &str) {
    let mut f = match File::create(path) {
        Ok(f) => f,
        Err(err) => {
            println!("Error creating file {:?}: {}", path, err);
            match File::open(path) {
                Ok(f) => f,
                Err(err) => panic!("Error opening file {:?}: {}", path, err),
            }
        }
    };

    match f.write_all(s.as_bytes()) {
        Ok(f) => f,
        Err(err) => panic!("Error writing data: {}", err),
    };
}
