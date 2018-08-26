extern crate cairo;
pub struct RGBColor(u8, u8, u8);

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
    
    pub fn get_cairo_pattern(&self) -> cairo::SolidPattern {
        cairo::SolidPattern::from_rgb(self.0 as f64 / 128., self.1 as f64 / 128., self.2 as f64 / 128.)
    }
}