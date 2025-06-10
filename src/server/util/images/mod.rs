pub mod bitmap;

pub struct ImageBuffer(pub Vec<[u8; 3]>, pub (u32, u32));

impl ImageBuffer {
    pub fn new(data: Vec<u8>, dimensions: (u32, u32)) -> Self {
        let mut pixels = Vec::new();
        for pixel in data.chunks(3) {
            if pixel.len() != 3 {
                return ImageBuffer(Vec::new(), (0, 0));
            }
            pixels.push([pixel[0], pixel[1], pixel[2]]);
        }
        ImageBuffer(pixels, dimensions)
    }

    pub fn add_pixel(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.0.push([r, g, b]);
        self
    }


    pub fn to_bytes(&self) -> Vec<u8> {
        let (width, height) = self.1;
        let mut result = Vec::new();

        let padding = (width * 3) % 4;

        for y in 0..height {
            for x in 0..width {
                let pixel = self.0[(y * width + x) as usize];
                result.push(pixel[0]);
                result.push(pixel[1]);
                result.push(pixel[2]);
            }            
            for _ in 0..padding {
                result.push(0);
                result.push(0);
                result.push(0);
            }
        }
        result
    }
}