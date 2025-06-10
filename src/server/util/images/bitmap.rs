use super::ImageBuffer;

const BITMAP_FILE_HEADER_SIZE: usize = 14;
const BITMAP_INFO_HEADER_SIZE: usize = 40;

#[derive(Debug, PartialEq, Eq)]
struct ByteHeader(Vec<u8>);

impl ByteHeader {
    pub fn new(size: usize) -> Self {
        ByteHeader(vec![0 ; size])
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn set_u16(&mut self, index: usize, value: u16) {
        self.0[index] = value as u8;
        self.0[index + 1] = (value >> 8) as u8;
    }

    pub fn set_u32(&mut self, index: usize, value: u32) {
        self.0[index] = value as u8;
        self.0[index + 1] = (value >> 8) as u8;
        self.0[index + 2] = (value >> 16) as u8;
        self.0[index + 3] = (value >> 24) as u8;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
    
    fn get_u32(&self, index: usize) -> u32 {
        self.0[index] as u32 | (self.0[index + 1] as u32) << 8 | (self.0[index + 2] as u32) << 16 | (self.0[index + 3] as u32) << 24
    }
}

#[derive(Debug, PartialEq, Eq)]
struct FileHeader(ByteHeader);

impl FileHeader {
    pub fn new() -> Self {
        FileHeader(ByteHeader::new(BITMAP_FILE_HEADER_SIZE))
    }

    fn write_file_signature(&mut self) {
        let value: u16 = ('M' as u16) << 8 | ('B' as u16);
        self.0.set_u16(0, value);
    }

    fn write_file_size(&mut self, size: u32) {
        self.0.set_u32(2, (BITMAP_FILE_HEADER_SIZE + BITMAP_INFO_HEADER_SIZE) as u32 + size);
    }

    pub fn get_file_size(&self) -> u32 {
        self.0.get_u32(2)
    }

    fn write_reserved(&mut self) {
        self.0.set_u32(6, 0);
    }

    fn write_total_header_size(&mut self) {
        self.0.set_u32(10, (BITMAP_FILE_HEADER_SIZE + BITMAP_INFO_HEADER_SIZE) as u32);
    }

    fn update_file_header(&mut self, size: u32) {
        self.write_file_signature();
        self.write_file_size(size);
        self.write_reserved();
        self.write_total_header_size();
    }

}

impl FromIterator<u8> for FileHeader {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        FileHeader(ByteHeader(iter.into_iter().collect()))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct InfoHeader(ByteHeader);

impl InfoHeader {
    pub fn new() -> Self {
        InfoHeader(ByteHeader::new(BITMAP_INFO_HEADER_SIZE))
    }

    fn write_height(&mut self, height: u32) {
        self.0.set_u32(8, height);
    }

    fn get_height(&self) -> u32 {
        self.0.get_u32(8)
    }

    fn write_width(&mut self, width: u32) {
        self.0.set_u32(4, width);
    }

    fn get_width(&self) -> u32 {
        self.0.get_u32(4)
    }

    fn write_info_header_size(&mut self) {
        self.0.set_u32(0, BITMAP_INFO_HEADER_SIZE as u32);
    }

    fn write_bpp(&mut self, bpp: u16) {
        self.0.set_u16(14, bpp);
    }

    fn write_num_planes(&mut self) {
        self.0.set_u16(12, 1);
    }

    fn write_resolution(&mut self) {
        self.0.set_u32(24, 1000);
        self.0.set_u32(28, 1000);
    }

    fn get_resolution(&self) -> (u32, u32) {
        (self.0.get_u32(24), self.0.get_u32(28))
    }

    fn write_num_colors(&mut self) {
        self.0.set_u32(36, 0);
    }

    fn get_num_colors(&self) -> u32 {
        self.0.get_u32(36)
    }

    fn write_colors_used(&mut self) {
        self.0.set_u32(32, 0);
    }

    fn write_pixel_data_size(&mut self, width: u32, height: u32, bits_per_pixel: u32) {
        let bytes = bits_per_pixel / 8;
        let padding = (width * bytes) % 4;
        self.0.set_u32(20, (3 * (width + padding) * height) as u32);
    }

    pub fn update_info_header(&mut self, width: u32, height: u32, bits_per_pixel: u32) {
        self.write_width(width);
        self.write_height(height);
        self.write_info_header_size();
        self.write_bpp(bits_per_pixel as u16);
        self.write_num_planes();
        self.write_resolution();
        self.write_num_colors();
        self.write_colors_used();
        self.write_pixel_data_size(width, height, bits_per_pixel);
    }
}

impl FromIterator<u8> for InfoHeader {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        InfoHeader(ByteHeader(iter.into_iter().collect()))
    }
}

pub struct Bitmap {
    pub file_header: FileHeader,
    pub info_header: InfoHeader,
    pub image_buffer: ImageBuffer,
    pub dimensions: (u32, u32),
}

impl Bitmap {
    pub fn new(data: Vec<u8>, dimensions: (u32, u32), bits_per_pixel: u32) -> Self {
        let mut file_header = FileHeader::new();
        let mut info_header = InfoHeader::new();
        let (width, height) = dimensions;
        let padding = width * 3 % 4;
        file_header.update_file_header(3 * (width + padding) * height);
        info_header.update_info_header(width, height, bits_per_pixel);

        Bitmap {
            file_header,
            info_header,
            image_buffer: ImageBuffer::new(data, dimensions),
            dimensions,
        }
    }

    pub fn write_bitmap(&self) -> Vec<u8> {
        vec![self.file_header.0.to_bytes(), self.info_header.0.to_bytes(), self.image_buffer.to_bytes()].concat()
    }
}