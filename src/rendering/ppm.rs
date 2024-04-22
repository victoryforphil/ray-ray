use std::{fmt::format, fs::File, io::Write, path::PathBuf};

use super::{Canvas, Color};

pub struct PPMFile {
    pub rows: Vec<String>,
}

impl Default for PPMFile {
    fn default() -> Self {
        Self {
            rows: Default::default(),
        }
    }
}

impl PPMFile {
    pub fn from_canvas(canvas: &Canvas) -> Self {
        let mut empty = PPMFile::default();
        empty.add_headers((canvas.width, canvas.height));
        empty.add_pixels(&canvas.pixels);
        empty
    }

    fn add_headers(&mut self, size: (usize, usize)) {
        self.rows.push(format!("P3"));
        self.rows.push(format!("{} {}", size.0, size.1));
        self.rows.push("255".to_string());
    }

    fn add_pixels(&mut self, pixels: &Vec<Vec<Color>>) {
        for row in pixels {
            let mut row_string = "".to_string();
            for px in row {
                let r = (px.r * 255.0) as u8;
                let g = (px.g * 255.0) as u8;
                let b = (px.b * 255.0) as u8;
              
                if row_string.len() >= 70 -3 {
                    self.rows.push(row_string.clone());
                    row_string = "".to_string();
                }

                row_string = row_string + format!("{} ", r).as_str();

                if row_string.len() >= 70 -3 {
                    self.rows.push(row_string.clone());
                    row_string = "".to_string();
                }
                
                row_string = row_string + format!("{} ", g).as_str();
                if row_string.len() >= 70 -3 {
                    self.rows.push(row_string.clone());
                    row_string = "".to_string();
                }
                
                row_string = row_string + format!("{} ", b).as_str();


            }
            self.rows.push(row_string);
        }
    }

    pub fn to_string(&self) -> String {
        let mut final_string: String = "".to_string();
        for row in &self.rows {
            let row = format!("{}", row).to_string();
            final_string = final_string + row.as_str() + "\n";
        }
        final_string = final_string + "\n";
        final_string
    }

    pub fn save_file(&self, path: &PathBuf){
        let mut file = File::create(path).unwrap();
        file.write_all(self.to_string().as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use log::info;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_ppm_default() {
        let ppm = PPMFile::default();
        assert_eq!(ppm.rows.len(), 0);
    }

    #[test]
    pub fn test_ppm_headers() {
        let canvas = Canvas::new(5, 3);
        let ppm = PPMFile::from_canvas(&canvas);
        assert_eq!(ppm.rows[0], "P3");
        assert_eq!(ppm.rows[1], "5 3");
        assert_eq!(ppm.rows[2], "255");
    }

    #[test]
    pub fn test_ppm_pixel_data() {
        env_logger::init();
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel([1.5, 0.0, 0.0].into(), (0, 0));
        canvas.write_pixel([0.0, 0.5, 0.0].into(), (2, 1));
        canvas.write_pixel([-0.5, 0.0, 1.0].into(), (4, 2));
        let ppm = PPMFile::from_canvas(&canvas);
        assert_eq!(ppm.rows[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 ");
        assert_eq!(ppm.rows[4], "0 0 0 0 0 0 0 127 0 0 0 0 0 0 0 ");
        assert_eq!(ppm.rows[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 ");
    }

    #[test]
    pub fn test_ppm_max_70() {
        env_logger::init();
        let mut canvas = Canvas::new(10, 2);
        canvas.set_all([1.0, 0.8, 0.6].into());

        let ppm = PPMFile::from_canvas(&canvas);
        assert_eq!(
            ppm.rows[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 "
        );
        assert_eq!(ppm.rows[4], "153 255 204 153 255 204 153 255 204 153 255 204 153 ");
        assert_eq!(
            ppm.rows[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 "
        );
        assert_eq!(ppm.rows[6], "153 255 204 153 255 204 153 255 204 153 255 204 153 ");
    }

}
