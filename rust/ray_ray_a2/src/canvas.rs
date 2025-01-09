use crate::Tuple;
use anyhow::anyhow;

#[derive(Default)]
pub struct Canvas{
    // Rows x Height
    pub pixels: Vec<Tuple>,
    pub width: usize, 
    pub height: usize,
}



impl Canvas{
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Tuple::default(); width * height];
        Canvas{
            pixels,
            width,
            height,
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> Result<usize, anyhow::Error> {
        let n_pixels = self.pixels.len();
        if x * y > n_pixels{
            return Err(anyhow!("Pixel {x}, {y} out of range {n_pixels}"));
        }

        let index = (y * self.height) + x;
        Ok(index)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: impl Into<Tuple>) -> Result<(), anyhow::Error> {
        let pixel_index = self.get_index(x, y)?;
        let pixel_opt = self.pixels.get_mut(pixel_index);
        match pixel_opt{
            Some(pixel) => {
                let color: Tuple = color.into();
                pixel.x = color.r();
                pixel.y = color.g();
                pixel.z = color.b();
                return Ok(());
            },
            None => {
                return Err(anyhow!("Failed to find pixel at {x}, {y}"));
            }

        }

    }

    pub fn get_pixel_ref(&self, x: usize, y: usize) -> Result<&Tuple, anyhow::Error> {
        let pixel_index = self.get_index(x, y)?;
        
        match self.pixels.get(pixel_index){
            Some(pixel) => Ok(pixel),
            None => Err(anyhow!("Pixel not found at {x}, {y}")),
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<Tuple, anyhow::Error> {
        let pixel_index = self.get_index(x, y)?;
        
        match self.pixels.get(pixel_index){
            Some(pixel) => Ok(pixel.clone()),
            None => Err(anyhow!("Pixel not found at {x}, {y}")),
        }
    }


}

#[cfg(test)]
mod test{
    use crate::Color;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_new_canvas(){
        let canvas = Canvas::new(10,20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for (p_i, pixel) in canvas.pixels.into_iter().enumerate() {
            assert_eq!(pixel,  Color::new(0, 0, 0), "Pixel {p_i} is not zero!");
        }
    }

    #[test]
    pub fn test_set_get_color(){
        let mut canvas = Canvas::new(10,20);
        let red = Color::new(255, 0, 0);        
        let write_pixel = canvas.set_pixel(2, 3, red);
        assert_eq!(write_pixel.is_ok(), true);

        let read_pixel = canvas.get_pixel_ref(2, 3);
        assert_eq!(read_pixel.is_ok(), true);

        let read_pixel = read_pixel.unwrap();
        assert_eq!(read_pixel.r(), 1.0);


    }
}