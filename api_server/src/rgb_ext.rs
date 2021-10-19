use image::Rgb;

pub struct HexCode {
    pub value: u32,
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
}
