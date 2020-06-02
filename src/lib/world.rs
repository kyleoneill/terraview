use super::TFileReader;
use super::Header;
use std::convert::TryInto;
use std::path::Path;

pub struct World {
    pub tfile_reader: TFileReader,
    pub header: Header
}

impl World {
    pub fn new(file_path: &Path) -> World {
        let mut tfile_reader = TFileReader::new(file_path);
        let header = Header::new(&mut tfile_reader);
        World {
            tfile_reader,
            header
        }
    }

    pub fn world_name(&mut self) -> String{
        let current_pos = self.tfile_reader.get_position();
        self.tfile_reader.move_to_position(self.header.array_of_pointers.header);
        let name = self.tfile_reader.read_string().to_string();
        self.tfile_reader.move_to_position(current_pos);
        name
    }

    pub fn iterate_tiles(&mut self) -> u32 {
        let mut running_total = 0;
        self.tfile_reader.move_to_position(self.header.array_of_pointers.tiles);
        while self.tfile_reader.get_position() < self.header.array_of_pointers.chests {
            let modified = self.read_tile();
            running_total += modified;
        }
        running_total
    }

    pub fn read_tile(&mut self) -> u32 {
        let tile_to_check = 12; //12 is for crystal hearts - this is a test value
        let flag_1 = self.tfile_reader.read_byte();
        let can_replace_block = (flag_1 & 1) != 0; //Bit 0 must be set if the block can be modified
        let block_is_two_bytes = (flag_1 & 33) != 0; //If bit 5 is set, the block is stored over 2 bytes rather than 1
        if can_replace_block {
            let flag_2 = self.tfile_reader.read_byte(); //Flag2 always has to exist here
            let third_flag_present = (flag_2 & 1) != 0;
            if third_flag_present {
                let _flag_3 = self.tfile_reader.read_byte(); //Flag3 may or may not exist
            }
            if block_is_two_bytes {
                let mut tile_bytes: [u8; 2] = [0; 2];
                self.tfile_reader.read_multiple_bytes(&mut tile_bytes);
                let tile = u16::from_le_bytes(tile_bytes.try_into().unwrap());
                if tile == tile_to_check {
                    return 1
                }
            }
            else {
                let tile = self.tfile_reader.read_byte();
                if tile == tile_to_check as u8 {
                    return 1
                }
            }
        }
        0
    }
}
