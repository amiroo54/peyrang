#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::regex::Regex;
use std::{env, fs::{self, File}, io::{BufRead, BufReader, Write}, path::{self, Path}, string};
use itertools::Itertools;


#[tauri::command]
fn generate_svg_with_color_combinations(input_svg_file: String, color_pallete: Vec<String>) -> Option<Vec<String>> {
    let binding = input_svg_file.clone();
    let mut output_folder = Path::new(&binding)
        .parent()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned();
    output_folder.push_str("/output");//Change this to something more user freindly.
    
    let colors = get_svg_data(input_svg_file.clone()).unwrap();
    println!("{}", output_folder);

    if !path::Path::new(&output_folder).exists()
    {
        if let Err(_) = fs::create_dir(&output_folder) {
            eprintln!("Failed to create folder '{}'", &output_folder);
            return None;
        }
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

fn is_hex_color(color: &str) -> bool {
    let re = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();
    re.is_match(color)
}

#[tauri::command]
fn get_svg_data(input_svg_file: String) -> Option<Vec<String>>
{
    println!("Opening {}", input_svg_file.clone());
    let file = File::open(input_svg_file.clone()).expect("Failed to open file");
    let reader = BufReader::new(file);
    
    let mut colors: Vec<String> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(index) = line.find("#") {
                let color = &line[index..index+7];
                if !is_hex_color(color) {
                    continue;
                }
                println!("{}", color);
                colors.push(color.to_string());
            }
        }
    }
    Some(colors)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_svg_with_color_combinations, get_svg_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}