use super::TFileReader;
use super::Header;
use std::convert::TryInto;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[repr(u8)]
pub enum FilePointerArray {
    Header = 0,
    Tiles = 1,
    Chests = 2,
    Signs = 3,
    Npc = 4,
    Entities = 5,
    Footer = 6,
    Unknown1 = 7,
    Unknown2 = 8,
    Unknown3 = 9,
    Unknown4 = 10
}

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

    pub fn change_world_name(&mut self, new_name: &str) {
        let pos = self.tfile_reader.get_position();
        let old_name = self.world_name();

        let new_name = new_name.as_bytes();
        self.tfile_reader.move_to_position(self.header.array_of_pointers.header as usize);
        self.tfile_reader.write_string_unknown_len(new_name);

        self.update_file_pointers(new_name.len() as i32 - old_name.len() as i32, FilePointerArray::Header);

        self.tfile_reader.move_to_position(self.tfile_reader.get_file_len() - (4 + old_name.len()));
        self.tfile_reader.write_string_known_len(new_name, old_name.len());

        self.tfile_reader.move_to_position(pos);
    }

    pub fn update_file_pointers(&mut self, bytes_added: i32, area_added_to: FilePointerArray) {
        let area_added_to = area_added_to as u8;
        let pos = self.tfile_reader.get_position();
        self.tfile_reader.move_to_position(self.header.array_start_address);
        self.tfile_reader.read_int_32();

        if area_added_to < FilePointerArray::Tiles as u8 {
            self.header.array_of_pointers.tiles += bytes_added;
            self.tfile_reader.write_int_32(self.header.array_of_pointers.tiles);
        }
        if area_added_to < FilePointerArray::Chests as u8 {
            self.header.array_of_pointers.chests += bytes_added;
            self.tfile_reader.write_int_32(self.header.array_of_pointers.chests);
        }
        if area_added_to < FilePointerArray::Signs as u8 {
            self.header.array_of_pointers.signs += bytes_added;
            self.tfile_reader.write_int_32(self.header.array_of_pointers.signs);
        }
        if area_added_to < FilePointerArray::Npc as u8 {
            self.header.array_of_pointers.npc += bytes_added;
            self.tfile_reader.write_int_32(self.header.array_of_pointers.npc);
        }
        if area_added_to < FilePointerArray::Entities as u8 {
            self.header.array_of_pointers.entities += bytes_added;
            self.tfile_reader.write_int_32(self.header.array_of_pointers.entities);
        }
        if area_added_to < FilePointerArray::Footer as u8 {
            self.header.array_of_pointers.footer += bytes_added;
            self.tfile_reader.write_int_32(self.header.array_of_pointers.footer);
        }
        if area_added_to < FilePointerArray::Unknown1 as u8 {
            self.header.array_of_pointers.unknown_1 += bytes_added;
            self.tfile_reader.write_int_32(self.header.array_of_pointers.unknown_1);
        }
        if area_added_to < FilePointerArray::Unknown2 as u8 {
            self.header.array_of_pointers.unknown_2 += bytes_added;
            self.tfile_reader.write_int_32(self.header.array_of_pointers.unknown_2);
        }
        if area_added_to < FilePointerArray::Unknown3 as u8 {
            self.header.array_of_pointers.unknown_3 += bytes_added;
            self.tfile_reader.write_int_32(self.header.array_of_pointers.unknown_3);
        }
        if area_added_to < FilePointerArray::Unknown4 as u8 {
            self.header.array_of_pointers.unknown_4 += bytes_added;
            self.tfile_reader.write_int_32(self.header.array_of_pointers.unknown_4);
        }
        self.tfile_reader.move_to_position(pos);
    }

    pub fn save_world(&mut self, file_path: &Path) -> Result<i32, Box<dyn Error>> {
        let mut f = File::create(file_path).expect("Unable to create file");
        let new_name = file_path.file_stem().unwrap();
        self.change_world_name(new_name.to_str().unwrap());
        f.write(self.tfile_reader.get_file().as_slice())?;
        Ok(0)
    }

    pub fn world_name(&mut self) -> String {
        let pos = self.tfile_reader.get_position();
        self.tfile_reader.move_to_position(self.header.array_of_pointers.header as usize);
        let name = self.tfile_reader.read_string().to_string();
        self.tfile_reader.move_to_position(pos);
        name
    }

    pub fn iterate_tiles(&mut self) -> u32 {
        let mut running_total = 0;
        self.tfile_reader.move_to_position(self.header.array_of_pointers.tiles as usize);
        while self.tfile_reader.get_position() < (self.header.array_of_pointers.chests as usize) {
            running_total += self.read_tile();
        }
        running_total
    }

    pub fn read_tile(&mut self) -> u32 {
        //broken - need to ensure that the entire tile is being read. There is potentially 10 bytes
        //not being read, this function currently stops reading the tile after the 1 or 2 byte tile ID
        let tile_to_check = 23; //23 is for corrupt grass - this is a test value
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
                    let new_tile: u16 = 2;
                    let bytes:[u8; 2] = new_tile.to_le_bytes();
                    self.tfile_reader.replace_bytes(&bytes);
                    return 1
                }
            }
            else {
                let tile = self.tfile_reader.read_byte();
                if tile as u16 == tile_to_check {
                    self.tfile_reader.replace_byte(2);
                    return 1
                }
            }
        }
        0
    }
}
