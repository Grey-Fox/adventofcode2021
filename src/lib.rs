use std::env;
use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines() -> io::Result<impl Iterator<Item = String>> {
    let mut args = env::args();
    let prog = args.next().unwrap();
    let filename = if let Some(param) = args.next() {
        param
    } else {
        let base_name = prog.rsplit("/").next().unwrap();
        format!("test_data/{}.txt", base_name)
    };
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|s| s.ok()))
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
