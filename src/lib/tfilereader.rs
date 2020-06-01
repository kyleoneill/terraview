use std::convert::TryInto;
use std::str;
pub struct TFileReader {
    file: Vec<u8>,
    position: usize
}

impl TFileReader {
    pub fn new(file: Vec<u8>) -> TFileReader {
        TFileReader {
            file,
            position: 0
        }
    }

    pub fn read_int_16(&mut self) -> i16 {
        let curr_pos = self.position;
        self.position += 2;
        i16::from_le_bytes(self.file[curr_pos..self.position].try_into().unwrap())
    }

    pub fn read_int_32(&mut self) -> i32 {
        let curr_pos = self.position;
        self.position += 4;
        i32::from_le_bytes(self.file[curr_pos..self.position].try_into().unwrap())
    }

    pub fn read_int_64(&mut self) -> i64 {
        let curr_pos = self.position;
        self.position += 8;
        i64::from_le_bytes(self.file[curr_pos..self.position].try_into().unwrap())
    }

    pub fn read_byte(&mut self) -> u8 {
        let curr_pos = self.position;
        self.position += 1;
        self.file[curr_pos]
    }

    pub fn read_multiple_bytes(&mut self, bytes: &mut [u8]) {
        let curr_pos = self.position;
        let amount = bytes.len();
        assert!(curr_pos + amount < self.file.len(), "Tried to read out of bounds");
        self.position += amount;
        for i in curr_pos..self.position {
            bytes[i - curr_pos] = self.file[i];
        }
    }

    pub fn read_string(&mut self) -> &str {
        let string_len = self.read_byte() as usize;
        let curr_pos = self.position;
        self.position += string_len;
        str::from_utf8(&self.file[curr_pos..curr_pos + string_len]).unwrap()
    }

    pub fn increment_position(&mut self, amount: usize) {
        assert!(self.position + amount < self.file.len(), "Tried to increment reader out of bounds");
        self.position += amount;
    }

    pub fn decrement_position(&mut self, amount: usize) {
        self.position -= amount;
    }

    pub fn get_position(&mut self) -> usize {
        self.position
    }

    pub fn move_to_position(&mut self, new_pos: usize) {
        assert!(new_pos < self.file.len(), "New position is out of bounds");
        self.position = new_pos;
    }

}