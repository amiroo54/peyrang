#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod oklab;
mod settings;

use tauri::regex::Regex;
use std::{env, fs::{self, File}, io::{BufRead, BufReader, Write}, path::{self, Path}};
use itertools::Itertools;
use oklab::{Lab, RGB};
use settings::*;

#[tauri::command]
fn generate_svg_with_color_combinations(input_svg_file: String, color_pallete: Vec<String>) -> Option<Vec<String>> {
    let output_folder = get_output_folder(&input_svg_file).expect("folder creation failed");
    
    let colors = get_svg_data(input_svg_file.clone()).unwrap();
    
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
    let output_folder = get_output_folder(&input_svg_file).expect("folder creation failed");
    
    let colors = get_svg_data(input_svg_file.clone()).unwrap();

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
    for line in reader.lines() {
        if let Ok(line) = line {
            for (index, _) in line.match_indices("#") {
                if line.len() <= index+7
                {
                    continue;
                }
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



#[tauri::command]
fn oklab_shift(input_svg_file: String, shift_type: i8, shift_color: String) -> Option<String>
{
    let output_folder = get_output_folder(&input_svg_file).expect("folder creation failed");
    
    let colors = get_svg_data(input_svg_file.clone()).unwrap();
    
    println!("The input color was {}", shift_color);
    let mut color_pallete: Vec<String> = vec![];
    for color in colors.iter() {
        let mut lab: Lab =RGB::from_hex(color).unwrap().to_oklab();
        let lab_shift: Lab = RGB::from_hex(&shift_color).unwrap().to_oklab(); 
        let mut result_color:String = String::default();
         
        match shift_type {
            0 => { //set the chroma
                let hue = lab.get_hue();

                lab.set_values(hue, lab_shift.get_chroma());

                result_color = lab.to_linear_srgb().to_hex();
            },
            1 => { //set the hue
                let chroma = lab.get_chroma();

                lab.set_values(lab_shift.get_hue(), chroma);

                result_color = lab.to_linear_srgb().to_hex();
            },
            2 => { //set the luminance
                lab.l = lab_shift.l;
                
                result_color = lab.to_linear_srgb().to_hex();
            },
            _ => println!("Wront input")
        }
        println!("Result of {} color was {}", color, result_color);
        color_pallete.push(result_color);
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
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_svg_with_color_combinations, replace_svg_color, oklab_shift, get_svg_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
