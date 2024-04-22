use rerun::Image;
use ndarray::{s, Array, ShapeBuilder};

use super::Canvas;

pub struct RerunViewer{

}

impl RerunViewer{
    pub fn from_canvas(canvas: &Canvas) -> Image{
        let mut image_bytes = Array::<u8, _>::zeros((canvas.height, canvas.width, 3).f());

        for (n_row, row) in canvas.pixels.iter().enumerate(){
            for (n_col, pixel) in row.iter().enumerate(){
                image_bytes.slice_mut(s![n_row..n_row+1, n_col..n_col+1,0]).fill((pixel.r * 255.0) as u8);
                image_bytes.slice_mut(s![n_row..n_row+1, n_col..n_col+1,1]).fill((pixel.g * 255.0) as u8);
                image_bytes.slice_mut(s![n_row..n_row+1, n_col..n_col+1,2]).fill((pixel.b * 255.0) as u8);
            }
        }

        Image::try_from(image_bytes).unwrap()
    }
}