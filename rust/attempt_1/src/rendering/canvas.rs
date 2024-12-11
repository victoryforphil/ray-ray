
use super::Color;
#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        let mut pixels = Vec::with_capacity(h);

        for _ in 0..h {
            let mut col = Vec::with_capacity(w);
            for _ in 0..w {
                col.push(Color::default());
            }

            pixels.push(col);
        }

        Self {
            width: w,
            height: h,
            pixels: pixels,
        }
    }

    pub fn write_pixel(&mut self, color: Color, pos: (usize, usize)) {
        if pos.1 > self.height {
            return;
        }

        if pos.0 > self.width {
            return;
        }

        self.pixels[pos.1][pos.0] = color;
    }

    pub fn pixel_at(&self, pos: (usize, usize)) -> &Color {
        &self.pixels[pos.1][pos.0]
    }

    pub fn set_all(&mut self, color: Color) {
        for h in 0..self.height {
            for w in 0..self.width {
                self.write_pixel(color.clone(), (w, h));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_canvas_new() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for pw in canvas.pixels {
            for ph in pw {
                assert_eq!(ph, Color::default())
            }
        }
    }

    #[test]
    pub fn test_canvas_set() {
        let mut canvas = Canvas::new(10, 20);
        canvas.write_pixel([1.0, 0.0, 0.0].into(), (2, 3));

        let wrote_pixel = canvas.pixel_at((2, 3));
        assert_eq!(wrote_pixel, &[1.0, 0.0, 0.0].into());
    }
}
