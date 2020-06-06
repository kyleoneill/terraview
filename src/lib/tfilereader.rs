use std::convert::TryInto;
use std::str;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
pub struct TFileReader {
    file: Vec<u8>,
    position: usize
}

impl TFileReader {
    pub fn new(world_path: &Path) -> TFileReader {
        let file = File::open(world_path).expect("Unable to open world file");
        let bytes = file.bytes().collect::<Result<Vec<_>, _>>().unwrap();
        TFileReader {
            file: bytes,
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

    pub fn write_int_32(&mut self, num: i32) {
        let bytes = num.to_le_bytes();
        for b in bytes.iter() {
            self.file[self.position] = *b;
            self.position += 1;
        }
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

    pub fn write_byte(&mut self, byte: u8) {
        self.file.insert(self.position, byte);
        self.position += 1;
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

    pub fn replace_bytes(&mut self, bytes: &[u8]) {
        let mut pos = self.position - bytes.len();
        for b in bytes.iter() {
            self.file[pos] = *b;
            pos += 1;
        }
        assert!(pos == self.position, "Position changed unexpectedly while splicing bytes");
    }

    pub fn replace_byte(&mut self, byte: u8) {
        self.file[self.position - 1] = byte;
    }

    pub fn read_string(&mut self) -> &str {
        let string_len = self.read_byte() as usize;
        let curr_pos = self.position;
        self.position += string_len;
        str::from_utf8(&self.file[curr_pos..curr_pos + string_len]).unwrap()
    }

    pub fn write_string_unknown_len(&mut self, s: &[u8]) {
        //need to optimize how the remove/insert are being called. Should resize the array one time rather than
        //once per char in old str, once per char in new str
        let old_len = self.read_byte();
        for _i in 0..old_len {
            self.file.remove(self.position);
        }
        let new_len = s.len();
        assert!(new_len < 256, "Tried to write a string that is too long");
        self.replace_byte(new_len as u8);
        for i in 0..s.len() {
            self.write_byte(s[i]);
        }
    }

    pub fn write_string_known_len(&mut self, s: &[u8], old_len: usize) {
        //need to optimize how the remove/insert are being called. Should resize the array one time rather than
        //once per char in old str, once per char in new str
        for _i in 0..old_len {
            self.file.remove(self.position);
        }
        let new_len = s.len();
        assert!(new_len < 256, "Tried to write a string that is too long");
        self.replace_byte(new_len as u8);
        for i in 0..s.len() {
            self.write_byte(s[i]);
        }
    }

    pub fn get_position(&mut self) -> usize {
        self.position
    }

    pub fn move_to_position(&mut self, new_pos: usize) {
        assert!(new_pos < self.file.len(), "New position is out of bounds");
        self.position = new_pos;
    }

    pub fn get_file(&self) -> &Vec<u8> {
        &self.file
    }

    pub fn get_file_len(&self) -> usize {
        self.file.len()
    }

}