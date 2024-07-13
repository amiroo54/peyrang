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

struct Lab {l: f32, a: f32, b: f32}
struct RGB {r: f32, g: f32, b: f32}

impl RGB {
    fn from_hex(hex: &String) -> Result<RGB, io::Error> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err(Error::new(io::ErrorKind::InvalidInput, "Invalid hex color format. It should be in the format #RRGGBB"));
        }
    
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap() as f32 / 255.0;
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap() as f32 / 255.0;
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap() as f32 / 255.0;
    
        Ok(RGB { r, g, b })
    }

    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", (self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
    }
}

impl Lab {
    fn get_hue(&self) -> f32
    {
        self.b.atan2(self.a)
    }

    fn get_chroma(&self) -> f32
    {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    fn set_values(&mut self, hue: f32, chroma: f32)
    {
        self.a = chroma * hue.cos();
        self.b = chroma * hue.sin();
    }
}

fn linear_srgb_to_oklab(c: RGB) -> Lab
{
    let l: f32 = 0.4122214708f32 * c.r + 0.5363325363f32 * c.g + 0.0514459929f32 * c.b;
	let m: f32 = 0.2119034982f32 * c.r + 0.6806995451f32 * c.g + 0.1073969566f32 * c.b;
	let s: f32 = 0.0883024619f32 * c.r + 0.2817188376f32 * c.g + 0.6299787005f32 * c.b;

    let l_: f32 = f32::powf(l, 1.0 / 3.0);
    let m_: f32 = f32::powf(m, 1.0 / 3.0);
    let s_: f32 = f32::powf(s, 1.0 / 3.0);

    Lab {
        l: 0.2104542553f32*l_ + 0.7936177850f32*m_ - 0.0040720468f32*s_,
        a: 1.9779984951f32*l_ - 2.4285922050f32*m_ + 0.4505937099f32*s_,
        b: 0.0259040371f32*l_ + 0.7827717662f32*m_ - 0.8086757660f32*s_,
    }
} 

fn oklab_to_linear_srgb(c: Lab) -> RGB
{
    let l_: f32 = c.l + 0.3963377774f32 * c.a + 0.2158037573f32 * c.b;
	let m_: f32 = c.l - 0.1055613458f32 * c.a - 0.0638541728f32 * c.b;
	let s_: f32 = c.l - 0.0894841775f32 * c.a - 1.2914855480f32 * c.b;

    let l: f32 = l_.powf(3.0);
    let m: f32 = m_.powf(3.0);
    let s: f32 = s_.powf(3.0);

    RGB {
        r: 4.0767416621f32 * l - 3.3077115913f32 * m + 0.2309699292f32 * s,
		g: -1.2684380046f32 * l + 2.6097574011f32 * m - 0.3413193965f32 * s,
		b: -0.0041960863f32 * l - 0.7034186147f32 * m + 1.7076147010f32 * s,
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
    println!("The input color was {}", shift_color);
    let mut color_pallete: Vec<String> = vec![];
    for color in colors.iter() {
        let mut lab: Lab = linear_srgb_to_oklab(RGB::from_hex(color).unwrap());
        let lab_shift: Lab = linear_srgb_to_oklab(RGB::from_hex(&shift_color).unwrap()); 
        let mut result_color:String = String::default();
        match shift_type {
            0 => {
                lab.l += lab_shift.l;
                lab.l %= 1.0;
                
                lab.a += lab_shift.a;
                lab.a %= 1.0;

                lab.b += lab_shift.b;
                lab.b %= 1.0;

                result_color = oklab_to_linear_srgb(lab).to_hex();
            },
            1 => {
                println!("Lab result for {} is L: {}, a: {}, b: {}", color, lab.l, lab.a, lab.b);
                let chroma = lab.get_chroma();
                let mut hue = lab.get_hue();

                hue += lab_shift.get_hue();

                lab.set_values(lab_shift.get_hue(), chroma);

                result_color = oklab_to_linear_srgb(lab).to_hex();
            },
            2 => {
                lab.b += lab_shift.b;
                lab.b = lab.b.clamp(0.0, 1.0);
                result_color = oklab_to_linear_srgb(lab).to_hex();
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