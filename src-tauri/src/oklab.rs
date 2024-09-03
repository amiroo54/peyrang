use std::io;
pub struct Lab {pub l: f32, pub a: f32, pub b: f32}
pub struct RGB {pub r: f32, pub g: f32, pub b: f32}

impl RGB {
    pub fn from_hex(hex: &String) -> Result<RGB, io::Error> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid hex color format. It should be in the format #RRGGBB"));
        }
    
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap() as f32 / 255.0;
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap() as f32 / 255.0;
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap() as f32 / 255.0;
    
        Ok(RGB { r, g, b })
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", (self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
    }

    pub fn to_oklab(&self) -> Lab
    {
        let l: f32 = 0.4122214708f32 * self.r + 0.5363325363f32 * self.g + 0.0514459929f32 * self.b;
        let m: f32 = 0.2119034982f32 * self.r + 0.6806995451f32 * self.g + 0.1073969566f32 * self.b;
        let s: f32 = 0.0883024619f32 * self.r + 0.2817188376f32 * self.g + 0.6299787005f32 * self.b;

        let l_: f32 = f32::powf(l, 1.0 / 3.0);
        let m_: f32 = f32::powf(m, 1.0 / 3.0);
        let s_: f32 = f32::powf(s, 1.0 / 3.0);

        Lab {
            l: 0.2104542553f32*l_ + 0.7936177850f32*m_ - 0.0040720468f32*s_,
            a: 1.9779984951f32*l_ - 2.4285922050f32*m_ + 0.4505937099f32*s_,
            b: 0.0259040371f32*l_ + 0.7827717662f32*m_ - 0.8086757660f32*s_,
        }
    }
}

impl Lab {
    pub fn get_hue(&self) -> f32
    {
        self.b.atan2(self.a)
    }

    pub fn get_chroma(&self) -> f32
    {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    pub fn set_values(&mut self, hue: f32, chroma: f32)
    {
        self.a = chroma * hue.cos();
        self.b = chroma * hue.sin();
    }

    pub fn to_linear_srgb(&self) -> RGB
    {
        let l_: f32 = self.l + 0.3963377774f32 * self.a + 0.2158037573f32 * self.b;
        let m_: f32 = self.l - 0.1055613458f32 * self.a - 0.0638541728f32 * self.b;
        let s_: f32 = self.l - 0.0894841775f32 * self.a - 1.2914855480f32 * self.b;
    
        let l: f32 = l_.powf(3.0);
        let m: f32 = m_.powf(3.0);
        let s: f32 = s_.powf(3.0);
    
        RGB {
            r: 4.0767416621f32 * l - 3.3077115913f32 * m + 0.2309699292f32 * s,
            g: -1.2684380046f32 * l + 2.6097574011f32 * m - 0.3413193965f32 * s,
            b: -0.0041960863f32 * l - 0.7034186147f32 * m + 1.7076147010f32 * s,
        }
    }
}
