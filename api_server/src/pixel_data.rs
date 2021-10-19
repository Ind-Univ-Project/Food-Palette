use image::Rgb;

use tide::log::info;

pub struct PixelData {
    r_div: u8,
    g_div: u8,
    b_div: u8,
    //R G B 영역을 각 x_div 로 나눠 전체 색 영역을 분할한다
    color_count: Vec<Vec<Vec<u32>>>,
}

impl PixelData {
    pub fn new(r_div: u8, g_div: u8, b_div: u8) -> Self {
        let color_count = vec![vec![vec![0; b_div.into()]; g_div.into()]; r_div.into()];
        Self {
            r_div,
            g_div,
            b_div,
            color_count,
        }
    }

    /// R, G, B값을 축으로 하는 3차원 영역에서
    /// 특정 색을 포함하는 영역의 인덱스를 구하는 함수
    ///
    /// # Example
    /// R = 63, div = 4 일 때 0-63, 64-127, 128-191, 192-255 각 영역 중
    ///
    /// R은 0~63인 0번째 인덱스 영역에 포함되어야 한다.
    ///
    /// ``` rust
    /// let R = 63;
    /// assert_eq!(get_area_index(R, 4), 0);
    /// ```
    fn get_area_index(color: u8, div: u8) -> usize {
        (color / ((u8::MAX + 1) / div)) as usize
    }

    pub fn count_color(&mut self, rgb: Rgb<u8>) {
        let r_index = PixelData::get_area_index(rgb[0], self.r_div);
        let g_index = PixelData::get_area_index(rgb[1], self.g_div);
        let b_index = PixelData::get_area_index(rgb[2], self.b_div);

        self.color_count[r_index][g_index][b_index] =
            self.color_count[r_index][g_index][b_index] + 1;

        info!(
            "Increase Index: [r: {}, g:{}, b:{}]",
            r_index, g_index, b_index
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn color_count_test() {
        let pixel_data = PixelData::new(2, 2, 2,);
    }
}
