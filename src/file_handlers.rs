use std::{fs, fs::File, io::prelude::*};

pub fn write_to(filepath: &str, contents: String) -> Result<(), std::io::Error> {
    let mut file = File::create(filepath)?;

    file.write_all(format_args!("{}", contents).to_string().as_bytes())?;
    Ok(())
}

pub fn read_from(filepath: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(filepath);

    if contents.is_ok() {
        Ok(contents.unwrap())
    } else {
        Err(contents.unwrap_err())
    }
}
