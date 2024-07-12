#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::regex::Regex;
use std::{env, fs::{self, File}, io::{self, BufRead, BufReader, Error, Write}, path::{self, Path}};
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
    for line in reader.lines() {
        if let Ok(line) = line {
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

struct Lab {l: f64, a: f64, b: f64}
struct RGB {r: f64, g: f64, b: f64}

impl RGB {
    fn from_hex(hex: &String) -> Result<RGB, io::Error> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err(Error::new(io::ErrorKind::InvalidInput, "Invalid hex color format. It should be in the format #RRGGBB"));
        }
    
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap() as f64 / 255.0;
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap() as f64 / 255.0;
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap() as f64 / 255.0;
    
        Ok(RGB { r, g, b })
    }

    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r as u8, self.g as u8, self.b as u8)
    }
}


fn linear_srgb_to_oklab(c: RGB) -> Lab
{
    let l: f64 = 0.4122214708f64 * c.r + 0.5363325363f64 * c.g + 0.0514459929f64 * c.b;
	let m: f64 = 0.2119034982f64 * c.r + 0.6806995451f64 * c.g + 0.1073969566f64 * c.b;
	let s: f64 = 0.0883024619f64 * c.r + 0.2817188376f64 * c.g + 0.6299787005f64 * c.b;

    let l_: f64 = f64::powf(l, 1.0 / 3.0);
    let m_: f64 = f64::powf(m, 1.0 / 3.0);
    let s_: f64 = f64::powf(s, 1.0 / 3.0);

    Lab {
        l: 0.2104542553f64*l_ + 0.7936177850f64*m_ - 0.0040720468f64*s_,
        a: 1.9779984951f64*l_ - 2.4285922050f64*m_ + 0.4505937099f64*s_,
        b: 0.0259040371f64*l_ + 0.7827717662f64*m_ - 0.8086757660f64*s_,
    }
} 

fn oklab_to_linear_srgb(c: Lab) -> RGB
{
    let l_: f64 = c.l + 0.3963377774f64 * c.a + 0.2158037573f64 * c.b;
	let m_: f64 = c.l - 0.1055613458f64 * c.a - 0.0638541728f64 * c.b;
	let s_: f64 = c.l - 0.0894841775f64 * c.a - 1.2914855480f64 * c.b;

    let l: f64 = l_.powf(3.0);
    let m: f64 = m_.powf(3.0);
    let s: f64 = s_.powf(3.0);

    RGB {
        r: 4.0767416621f64 * l - 3.3077115913f64 * m + 0.2309699292f64 * s,
		g: -1.2684380046f64 * l + 2.6097574011f64 * m - 0.3413193965f64 * s,
		b: -0.0041960863f64 * l - 0.7034186147f64 * m + 1.7076147010f64 * s,
    }
} 
#[tauri::command]
fn oklab_shift(input_svg_file: String, shift_type: i8, shift_color: String) -> Option<String>
{
    let output_folder = get_output_folder(&input_svg_file);
    
    let colors = get_svg_data(input_svg_file.clone()).unwrap();
    println!("{}", output_folder);

    if !check_folder(&output_folder)
    {
        return None;
    }
    
    let mut color_pallete: Vec<String> = vec![];
    for color in colors.iter() {
        let mut lab: Lab = linear_srgb_to_oklab(RGB::from_hex(color).unwrap());
        let lab_shift: Lab = linear_srgb_to_oklab(RGB::from_hex(&shift_color).unwrap()); 
        let mut result_color:String = String::default();
        match shift_type {
            0 => {
                lab.l += lab_shift.l;
                lab.l = lab.l.clamp(0.0, 1.0);
                result_color = oklab_to_linear_srgb(lab).to_hex();
            },
            1 => {
                lab.a += lab_shift.a;
                lab.a = lab.a.clamp(0.0, 1.0);
                result_color = oklab_to_linear_srgb(lab).to_hex();
            },
            2 => {
                lab.b += lab_shift.b;
                lab.b = lab.b.clamp(0.0, 1.0);
                result_color = oklab_to_linear_srgb(lab).to_hex();
            },
            _ => println!("Wront input")
        }
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