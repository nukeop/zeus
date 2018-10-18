pub struct ScreenDriver {
    pub width: u8,
    pub height: u8,
    pub pixels: [u8; 40]
}

impl ScreenDriver {
    pub fn new(w: u8, h: u8) -> ScreenDriver {
        ScreenDriver {
            width: w,
            height: h,
            pixels: [0; 40]
        }
    }
}
