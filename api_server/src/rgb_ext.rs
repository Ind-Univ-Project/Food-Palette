use image::Rgb;

pub struct HexCode {
    pub value: u32,
}
impl HexCode {
    pub fn new(value: u32) -> Self {
        assert_eq!(0xFF000000 & value, 0, "value: {} is too big", value);
        Self { value }
    }
}
trait ToHexCode {
    fn to_hex_code(&self) -> HexCode;
}

impl ToHexCode for Rgb<u8> {
    fn to_hex_code(&self) -> HexCode {
        HexCode {
            value: (self[0] as u32) << 16 | (self[1] as u32) << 8 | (self[2] as u32),
        }
    }
}

impl From<HexCode> for Rgb<u8> {
    fn from(hex: HexCode) -> Self {
        let r = ((hex.value & 0xFF0000) >> 16) as u8;
        let g = ((hex.value & 0x00FF00) >> 8) as u8;
        let b = ((hex.value & 0x0000FF) >> 0) as u8;
        Rgb::from([r, g, b])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn to_hex_code_test() {
        let rgb1 = Rgb::from([0x7b as u8, 0x11, 0xaa]);
        assert_eq!(rgb1.to_hex_code().value, 0x7b11aa);

        let rgb2 = Rgb::from([0xca as u8, 0xfe, 0xdd]);
        assert_eq!(rgb2.to_hex_code().value, 0xcafedd);
    }

    #[test]
    fn from_hex_code_test() {
        let rgb = Rgb::from(HexCode::new(0xcafebe));

        assert_eq!(rgb[0], 0xca);
        assert_eq!(rgb[1], 0xfe);
        assert_eq!(rgb[2], 0xbe);
    }

    #[test]
    #[should_panic]
    fn too_big_hexcode_gen_test() {
        let _ = HexCode::new(0x01FFFFFF);
    }
}
