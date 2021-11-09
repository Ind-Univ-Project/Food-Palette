use image::Rgb;

use tide::log::info;

use async_std::sync::Mutex;

pub struct PixelData {
    r_div: u8,
    g_div: u8,
    b_div: u8,
    //R G B 영역을 각 *_div 로 나눠 전체 색 영역을 분할한다
    color_count: Vec<Vec<Vec<Mutex<u32>>>>,
}

impl PixelData {
    /// PixelData 인스턴스를 생성
    /// 
    /// # Arguments
    /// 
    /// * `*_div` - r,g,b 영역을 분할하는 값
    pub fn new(r_div: u8, g_div: u8, b_div: u8) -> Self {
        let mut color_count = Vec::new();

        for _ in 0..r_div {
            color_count.push(Vec::new());
        }

        for r in &mut color_count {
            for _ in 0..g_div {
                let mut v = Vec::new();
                for _ in 0..b_div {
                    v.push(Mutex::new(0));
                }
                r.push(v);
            }
        }

        Self {
            r_div,
            g_div,
            b_div,
            color_count,
        }
    }

    /// R, G, B값을 축으로 하는 3차원 영역에서
    /// 해당 색을 포함하는 영역의 인덱스를 구하는 함수
    /// 
    /// # Arguments
    /// 
    /// * `color` - R, G, B 중 한개의 색
    /// * `div` - 색 영역을 몇 분할 할지 결정하는 값
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
    pub fn get_area_index(color: u8, div: u8) -> usize {
        (color / ((u8::MAX as u16 + 1) / div as u16) as u8) as usize
    }

    /// 가장 많이 나온 색이 포함된 영역의 인덱스를 구하는 함수
    async fn index_of_max(&self) -> (usize, usize, usize) {
        let mut max_ind = (0, 0, 0);
        let mut max_val = 0u32;

        for (r, v) in self.color_count.iter().enumerate() {
            for (g, v) in v.iter().enumerate() {
                for (b, v) in v.iter().enumerate() {
                    let val = *v.lock().await;
                    if val > max_val {
                        max_ind = (r, g, b);
                        max_val = val;
                    }
                }
            }
        }

        max_ind
    }

    /// RGB값 하나를 입력 받아 해당 색의 개수를 1 증가 시킨다.
    pub async fn count_color(&mut self, rgb: Rgb<u8>) {
        let r_index = PixelData::get_area_index(rgb[0], self.r_div);
        let g_index = PixelData::get_area_index(rgb[1], self.g_div);
        let b_index = PixelData::get_area_index(rgb[2], self.b_div);

        *self.color_count[r_index][g_index][b_index].lock().await += 1;

        info!(
            "Increase Index: [r: {}, g:{}, b:{}]",
            r_index, g_index, b_index
        );
    }

    /// 전체 색 영역 중 가장 많이 나온 색 영역 순서대로
    /// 인덱스를 문자열로 반환하는 함수
    /// # Arguments
    /// 
    /// * `len` - 반환 될 문자열이 몇개의 색 영역을 포함할지 결정하는 값
    pub async fn into_string(self, len: usize) -> String {
        let mut result = String::from("");

        for _ in 0..len {
            let (r, g, b) = self.index_of_max().await;
            let index_string = format!("{:0x}{:02x}{:02x}", r, g, b);
            result.push_str(&index_string);

            *self.color_count[r][g][b].lock().await = 0;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::rgb_ext::HexCode;
    use super::*;

    use async_std::test as async_test;
    #[async_test]
    async fn get_area_index_test() {
        let rgb = Rgb::from(HexCode::new(0x0044FF));
        let r = PixelData::get_area_index(rgb[0], 4);
        let g = PixelData::get_area_index(rgb[1], 4);
        let b = PixelData::get_area_index(rgb[2], 4);
        assert_eq!(r, 0);
        assert_eq!(g, 1);
        assert_eq!(b, 3);
    }

    #[async_test]
    async fn counter_test() {
        let mut pixel_data = PixelData::new(4, 4, 4);
        let rgb1 = Rgb::from([0x00u8, 0x88, 0xFF]);
        let rgb2 = Rgb::from([0xAAu8, 0xDD, 0x55]);

        pixel_data.count_color(rgb1).await;
        pixel_data.count_color(rgb1).await;
        pixel_data.count_color(rgb2).await;

        let ind = pixel_data.index_of_max().await;

        assert_eq!((0, 2, 3), ind);

        pixel_data.count_color(rgb2).await;
        pixel_data.count_color(rgb2).await;
        pixel_data.count_color(rgb2).await;

        let ind = pixel_data.index_of_max().await;

        assert_eq!((2, 3, 1), ind);
        
        let color_string = pixel_data.into_string(2).await;
        assert_eq!("020301000203", color_string);
    }
}
