#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::regex::Regex;
use std::{env, fs::{self, File}, io::{BufRead, BufReader, Write}, path::{self, Path}, string};
use itertools::Itertools;


#[tauri::command]
fn generate_svg_with_color_combinations(input_svg_file: String, color_pallete: Vec<String>) -> Option<Vec<String>> {
    let output_folder = get_output_folder(&input_svg_file);
    
    let colors = get_svg_data(input_svg_file.clone()).unwrap();
    println!("{}", output_folder);

    if !check_folder(&output_folder)
    {
        return None;
    }
    
    let mut outputs = vec![];
    let mut index = 0;
    for color in color_pallete.iter().permutations(colors.len()).unique() {
        let filename = format!("{}/out{}.svg", &output_folder, index);
        let mut output = File::create(filename.clone()).unwrap();
        outputs.push(filename);

        let input_file = File::open(input_svg_file.clone()).expect("Failed to open input file");
        let reader = BufReader::new(input_file);

        for line in reader.lines() {
            let mut updated_line = line.unwrap();
            for i in 0..colors.len() {
                updated_line = updated_line.replace(&colors[i], &color[i]);
            }
            writeln!(&mut output, "{}", updated_line).expect("Failed to write to output file");
        }
        index += 1;
    }
    Some(outputs)
}

#[tauri::command]
fn replace_svg_color(input_svg_file: String, color_pallete: Vec<String>) -> Option<String> {
    let output_folder = get_output_folder(&input_svg_file);
    
    let colors = get_svg_data(input_svg_file.clone()).unwrap();
    println!("{}", output_folder);

    if !check_folder(&output_folder)
    {
        return None;
    }


    let filename = format!("{}/out.svg", &output_folder);
    let mut output = File::create(filename.clone()).unwrap();

    let input_file = File::open(input_svg_file.clone()).expect("Failed to open input file");
    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        let mut updated_line = line.unwrap();
        for i in 0..colors.len() {
            updated_line = updated_line.replace(&colors[i], &color_pallete[i]);
        }
        writeln!(&mut output, "{}", updated_line).expect("Failed to write to output file");
    }
    Some(filename)

}

fn get_output_folder(folder: &String) -> String { //Change this to something more user freindly. 
    let mut output_folder = Path::new(folder)
        .parent()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned();
    output_folder.push_str("/output");
    output_folder
}

fn check_folder(folder: &String) -> bool {
    if !path::Path::new(folder).exists()
    {
        if let Err(_) = fs::create_dir(folder) {
            eprintln!("Failed to create folder '{}'", folder);
            return false;
        }
    }
    true
}

fn is_hex_color(color: &String) -> bool {
    let re = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();
    re.is_match(color)
}

#[tauri::command]
fn get_svg_data(input_svg_file: String) -> Option<Vec<String>> {
    println!("Opening {}", input_svg_file.clone());
    let file = File::open(input_svg_file.clone()).expect("Failed to open file");
    let reader = BufReader::new(file);
    
    let mut colors: Vec<String> = Vec::new();
    let mut in_defs = false;
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains("<defs") {
                // This is the start of the defs section
                println!("this is def-enetily where drawing starts");
                in_defs = true;
                continue;
            } 
            if line.contains("</defs>") {
                println!("this is def-enetily where drawing ends");
                in_defs = false;
                continue;
            } 
            if !in_defs {continue;}
            if let Some(index) = line.find("#") {
                let color = line[index..index+7].to_string();
                if !is_hex_color(&color) {
                    continue;
                }
                if colors.contains(&color)
                {
                    println!("Duplicate color: {}.", &color);
                    continue;
                }
                println!("{}", color);
                colors.push(color);
            }
        }
    }
    Some(colors)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_svg_with_color_combinations, replace_svg_color, get_svg_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}