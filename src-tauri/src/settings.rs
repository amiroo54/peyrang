use std::{fs, io, path::Path};

pub fn get_output_folder(folder: &String) -> Result<String, io::Error> { //Change this to something more user freindly. 
    let mut output_folder = Path::new(folder)
        .parent()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned();
    output_folder.push_str("/output");
    if !check_folder(folder)
    {
        return Err(io::Error::other("Failt to create folder"));
    }
    Ok(output_folder)
}

pub fn check_folder(folder: &String) -> bool {
    if !Path::new(folder).exists()
    {
        if let Err(_) = fs::create_dir(folder) {
            return false;
        }
    }
    true
}