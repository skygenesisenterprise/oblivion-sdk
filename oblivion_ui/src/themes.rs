#[derive(Clone)]
pub struct Theme {
    pub primary_color: (u8, u8, u8),
    pub secondary_color: (u8, u8, u8),
    pub background_color: (u8, u8, u8),
    pub text_color: (u8, u8, u8),
    pub font_size: u32,
    pub is_dark: bool,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            primary_color: (0, 122, 255),
            secondary_color: (142, 142, 147),
            background_color: (255, 255, 255),
            text_color: (0, 0, 0),
            font_size: 14,
            is_dark: false,
        }
    }
}

impl Theme {
    pub fn dark() -> Self {
        Theme {
            primary_color: (10, 132, 255),
            secondary_color: (142, 142, 147),
            background_color: (28, 28, 30),
            text_color: (255, 255, 255),
            font_size: 14,
            is_dark: true,
        }
    }
}