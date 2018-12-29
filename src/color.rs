pub struct Color([u8; 4]);

impl Color {
    pub fn black() -> Color {
        Color([0x30, 0x36, 0x33, 0xff])
    }

    pub fn gray() -> Color {
        Color([0x7f, 0x7b, 0x82, 0xff])
    }

    pub fn green() -> Color {
        Color([0x7c, 0xae, 0x7a, 0xff])
    }

    pub fn to_a(&self) -> [f32; 4] {
        [
            self.0[0] as f32/0xff as f32,
            self.0[1] as f32/0xff as f32,
            self.0[2] as f32/0xff as f32,
            self.0[3] as f32/0xff as f32,
        ]
    }
}
