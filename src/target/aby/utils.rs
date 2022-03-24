//! Utility functions to write compiler output to ABY

use std::fs;
<<<<<<< HEAD
use std::io::{prelude::*};
use std::path::Path;
use std::path::PathBuf;

/// Given PathBuf `path_buf` and String denominator `lang`, return the filename of the path
pub fn get_path(path_buf: &PathBuf, lang: &String, t: &String) -> String {
    let filename = Path::new(&path_buf.iter().last().unwrap().to_os_string())
=======
use std::io::prelude::*;
use std::path::Path;

/// Given Path `path` and String denominator `lang`, return the filename of the path
pub fn get_path(path: &Path, lang: &str, t: &str) -> String {
    let filename = Path::new(&path.iter().last().unwrap().to_os_string())
>>>>>>> 75572c6... C Frontend (#22)
        .file_stem()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

<<<<<<< HEAD
    let name = format!("{}_{}", filename, lang);

    // TODO: clean
<<<<<<< HEAD
    let path = format!(
        "third_party/ABY/src/examples/{}_{}_tmp.txt",
        name,
        t
    );
=======
    let path = format!("third_party/ABY/src/examples/{}_{}_tmp.txt", name, t);
>>>>>>> 75572c6... C Frontend (#22)
=======
    match fs::create_dir_all("scripts/aby_tests/tests") {
        Err(why) => panic!("couldn't create {}: {}", "scripts/aby_tests/tests", why),
        Ok(file) => file,
    };
>>>>>>> 8fed29b... ABY VM and Interpreter (#47)

    let name = format!("{}_{}", filename, lang);
    let path = format!("scripts/aby_tests/tests/{}_{}.txt", name, t);
    match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path, why),
        Ok(file) => file,
    };
    path
}

/// Write circuit output to temporary file
<<<<<<< HEAD
<<<<<<< HEAD
pub fn write_line_to_file(path: &String, line: &String) {
    if !Path::new(&path).exists() {
        fs::File::create(&path).expect("Failed to create tmp file");
    }
    
    let mut file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open(path)
      .expect("Failed to open circuit_tmp file");

    file.write_all(line.as_bytes()).expect("Failed to write to circuit_tmp file");
=======
pub fn write_line_to_file(path: &str, line: &str) {
=======
pub fn write_lines_to_file(path: &str, lines: &[String]) {
>>>>>>> 13f9a09... Updated ABY VM to include `IN` bytecode instruction (#65)
    if !Path::new(&path).exists() {
        fs::File::create(&path).expect(&*format!("Failed to create: {}", path));
    }

    let data = lines.join("");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("Failed to open circuit_tmp file");

    file.write_all(data.as_bytes())
        .expect("Failed to write to circuit_tmp file");
>>>>>>> 75572c6... C Frontend (#22)
}
