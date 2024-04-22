use super::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        let pixels = vec![vec![]];

        for w_p in w {
            for w_h in h {}
        }

        Self {
            width: w,
            height: h,
            pixels: Vec::with_capacity(w),
        }
    }
}
