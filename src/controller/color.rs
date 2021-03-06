extern crate cairo;
#[derive(Clone)]
pub struct RGBColor(u8, u8, u8);

pub const BLACK: RGBColor = RGBColor(0, 0, 0);
pub const WHITE: RGBColor = RGBColor(128, 128, 128);

impl RGBColor {
    pub fn get_rgb(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    pub fn new(r: u8, g: u8, b: u8) -> RGBColor {
        RGBColor (r,g,b)
    }

    pub fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.0 = r;
        self.1 = g;
        self.2 = b;
    }
    
    pub fn as_cairo_pattern(&self) -> cairo::SolidPattern {
        cairo::SolidPattern::from_rgb(self.0 as f64 / 128., self.1 as f64 / 128., self.2 as f64 / 128.)
    }

    pub fn as_usize(&self) -> usize {
        (self.0 as usize) << 16 | (self.1 as usize) << 8 | (self.2) as usize
    }
}