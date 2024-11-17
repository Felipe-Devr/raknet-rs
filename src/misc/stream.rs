use super::u24::u24;

pub enum Endianness {
    BigEndian,
    LittleEndian,
}

#[derive(Debug, Clone)]
pub struct BinaryStream {
    pub buffer: Vec<u8>,
    position: usize,
}

impl BinaryStream {
    pub fn new(data: Option<Vec<u8>>) -> Self {
        BinaryStream {
            buffer: data.unwrap_or_default(),
            position: 0,
        }
    }

    pub fn finished(&self) -> bool {
        self.position == self.buffer.len()
    }

    pub fn advance(&mut self, offset: usize) {
        self.position += offset;
    }

    pub fn expand(&mut self, size: usize, data: u8) {
        self.buffer.extend_from_slice(&vec![data; size]);
    }

    pub fn size(&self) -> usize {
        self.buffer.len()
    }
}

impl BinaryStream {
    pub fn write(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn write_bool(&mut self, value: bool) {
        self.write_u8(value.into());
    }

    pub fn write_u8(&mut self, data: u8) {
        self.buffer.push(data);
    }

    pub fn write_u16(&mut self, data: u16, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::BigEndian => u16::to_be_bytes(data),
            Endianness::LittleEndian => u16::to_le_bytes(data),
        };
        self.write(&bytes);
    }

    pub fn write_u24(&mut self, data: u24, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::BigEndian => u24::to_u32(data).to_be_bytes(),
            Endianness::LittleEndian => u24::to_u32(data).to_le_bytes(),
        };
        self.write(&bytes);
    }

    pub fn write_u32(&mut self, data: u32, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::BigEndian => u32::to_be_bytes(data),
            Endianness::LittleEndian => u32::to_le_bytes(data),
        };
        self.write(&bytes);
    }

    pub fn write_u64(&mut self, data: u64, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::BigEndian => u64::to_be_bytes(data),
            Endianness::LittleEndian => u64::to_le_bytes(data),
        };
        self.write(&bytes);
    }

    pub fn write_i8(&mut self, data: i8) {
        self.write(&data.to_le_bytes());
    }

    pub fn write_i16(&mut self, data: i16, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::BigEndian => i16::to_be_bytes(data),
            Endianness::LittleEndian => i16::to_le_bytes(data),
        };
        self.write(&bytes);
    }

    pub fn write_i32(&mut self, data: i32, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::BigEndian => i32::to_be_bytes(data),
            Endianness::LittleEndian => i32::to_le_bytes(data),
        };
        self.write(&bytes);
    }

    pub fn write_i64(&mut self, data: i64, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::BigEndian => i64::to_be_bytes(data),
            Endianness::LittleEndian => i64::to_le_bytes(data),
        };
        self.write(&bytes);
    }

    pub fn write_f32(&mut self, data: f32, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::BigEndian => f32::to_be_bytes(data),
            Endianness::LittleEndian => f32::to_le_bytes(data),
        };
        self.write(&bytes);
    }

    pub fn write_f64(&mut self, data: f64, endianness: Endianness) {
        let bytes = match endianness {
            Endianness::BigEndian => f64::to_be_bytes(data),
            Endianness::LittleEndian => f64::to_le_bytes(data),
        };
        self.write(&bytes);
    }

    pub fn write_string(&mut self, data: &str) {
        self.write_u16(data.len() as u16, Endianness::LittleEndian);
        self.write(data.as_bytes());
    }
}

// Read Implementation
impl BinaryStream {
    pub fn read(&mut self, size: usize) -> Option<Vec<u8>> {
        if self.position + size > self.buffer.len() {
            return None;
        }
        let result = self.buffer[self.position..self.position + size].to_vec();
        self.position += size;
        Some(result)
    }

    pub fn read_bool(&mut self) -> Option<bool> {
        let parsed: bool = self.read_u8().map(|n| n == 1).unwrap();

        Some(parsed)
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.position < self.buffer.len() {
            let value = self.buffer[self.position];
            self.position += 1;
            Some(value)
        } else {
            None
        }
    }

    pub fn read_u16(&mut self, endianness: Endianness) -> Option<u16> {
        if self.position + 2 > self.buffer.len() {
            return None;
        }
        let bytes: [u8; 2] = [self.buffer[self.position], self.buffer[self.position + 1]];
        self.position += 2;

        match endianness {
            Endianness::BigEndian => Some(u16::from_be_bytes(bytes)),
            Endianness::LittleEndian => Some(u16::from_le_bytes(bytes)),
        }
    }

    pub fn read_u24(&mut self, endianness: Endianness) -> Option<u24> {
        if self.position + 3 > self.buffer.len() {
            return None;
        }
        let bytes: [u8; 4] = [
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            0,
        ];
        self.position += 3;

        match endianness {
            Endianness::BigEndian => Some(u24::from_u32(u32::from_be_bytes(bytes))),
            Endianness::LittleEndian => Some(u24::from_u32(u32::from_le_bytes(bytes))),
        }
    }

    pub fn read_u32(&mut self, endianness: Endianness) -> Option<u32> {
        if self.position + 4 > self.buffer.len() {
            return None;
        }
        let bytes: [u8; 4] = [
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            self.buffer[self.position + 3],
        ];
        self.position += 4;

        match endianness {
            Endianness::BigEndian => Some(u32::from_be_bytes(bytes)),
            Endianness::LittleEndian => Some(u32::from_le_bytes(bytes)),
        }
    }

    pub fn read_u64(&mut self, endianness: Endianness) -> Option<u64> {
        if self.position + 8 > self.buffer.len() {
            return None;
        }
        let bytes: [u8; 8] = [
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            self.buffer[self.position + 3],
            self.buffer[self.position + 4],
            self.buffer[self.position + 5],
            self.buffer[self.position + 6],
            self.buffer[self.position + 7],
        ];
        self.position += 8;

        match endianness {
            Endianness::BigEndian => Some(u64::from_be_bytes(bytes)),
            Endianness::LittleEndian => Some(u64::from_le_bytes(bytes)),
        }
    }

    pub fn read_f32(&mut self, endianness: Endianness) -> Option<f32> {
        if self.position + 4 > self.buffer.len() {
            return None;
        }
        let bytes: [u8; 4] = [
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            self.buffer[self.position + 3],
        ];
        self.position += 4;

        match endianness {
            Endianness::BigEndian => Some(f32::from_be_bytes(bytes)),
            Endianness::LittleEndian => Some(f32::from_le_bytes(bytes)),
        }
    }

    pub fn read_f64(&mut self, endianness: Endianness) -> Option<f64> {
        if self.position + 8 > self.buffer.len() {
            return None;
        }
        let bytes: [u8; 8] = [
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            self.buffer[self.position + 3],
            self.buffer[self.position + 4],
            self.buffer[self.position + 5],
            self.buffer[self.position + 6],
            self.buffer[self.position + 7],
        ];
        self.position += 8;

        match endianness {
            Endianness::BigEndian => Some(f64::from_be_bytes(bytes)),
            Endianness::LittleEndian => Some(f64::from_le_bytes(bytes)),
        }
    }

    pub fn read_string(&mut self) -> Option<&str> {
        let length = self.read_u16(Endianness::LittleEndian)?;

        println!("{}", length);
        if self.position + length as usize > self.buffer.len() {
            return None;
        }
        let string =
            std::str::from_utf8(&self.buffer[self.position..self.position + length as usize]);
        self.position += length as usize;

        if let Ok(s) = string {
            Some(s)
        } else {
            None
        }
    }

    pub fn read_i8(&mut self) -> Option<i8> {
        if self.position + 1 > self.buffer.len() {
            return None;
        }
        let value = self.buffer[self.position];
        self.position += 1;
        Some(value as i8)
    }

    pub fn read_i16(&mut self, endianness: Endianness) -> Option<i16> {
        if self.position + 2 > self.buffer.len() {
            return None;
        }
        let bytes: [u8; 2] = [self.buffer[self.position], self.buffer[self.position + 1]];
        self.position += 2;

        match endianness {
            Endianness::BigEndian => Some(i16::from_be_bytes(bytes)),
            Endianness::LittleEndian => Some(i16::from_le_bytes(bytes)),
        }
    }

    pub fn read_i32(&mut self, endianness: Endianness) -> Option<i32> {
        if self.position + 4 > self.buffer.len() {
            return None;
        }
        let bytes: [u8; 4] = [
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            self.buffer[self.position + 3],
        ];
        self.position += 4;

        match endianness {
            Endianness::BigEndian => Some(i32::from_be_bytes(bytes)),
            Endianness::LittleEndian => Some(i32::from_le_bytes(bytes)),
        }
    }

    pub fn read_i64(&mut self, endianness: Endianness) -> Option<i64> {
        if self.position + 8 > self.buffer.len() {
            return None;
        }
        let bytes: [u8; 8] = [
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            self.buffer[self.position + 3],
            self.buffer[self.position + 4],
            self.buffer[self.position + 5],
            self.buffer[self.position + 6],
            self.buffer[self.position + 7],
        ];

        self.position += 8;
        match endianness {
            Endianness::BigEndian => Some(i64::from_be_bytes(bytes)),
            Endianness::LittleEndian => Some(i64::from_le_bytes(bytes)),
        }
    }

    pub fn read_to_end(&mut self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.buffer[self.position..]);
        self.position = self.buffer.len();
    }
}
