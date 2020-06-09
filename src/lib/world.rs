use super::TFileReader;
use super::Header;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

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

pub struct Tile {
    pub flag_1: u8,
    pub flag_2: Option<u8>,
    pub flag_3: Option<u8>,
    pub tile_type: Option<u16>
}

impl Tile {
    pub fn new(world: &mut World) -> Tile {
        let flag_1 = world.tfile_reader.read_byte();
        assert!(flag_1 != 255, "Flag1 should not have every bit set");
        let mut flag_2 = None;
        let mut flag_3 = None;
        let mut tile_type = None;
        let second_flag_present = (flag_1 & 1) != 0;
        if second_flag_present {
            flag_2 = Some(world.tfile_reader.read_byte());
            let third_flag_present = (flag_2.unwrap() & 1) != 0;
            if third_flag_present {
                flag_3 = Some(world.tfile_reader.read_byte()); //Flag3 may or may not exist
            }
        }
        let block_has_tile_type = (flag_1 & 2) != 0; //Bit 1 must be set if the block has a tile type
        if block_has_tile_type {
            let block_is_two_bytes = (flag_1 & 32) != 0; //If bit 5 is set, the block is stored over 2 bytes rather than 1
            if block_is_two_bytes {
                let mut tile_bytes: [u8; 2] = [0; 2];
                world.tfile_reader.read_multiple_bytes(&mut tile_bytes);
                let tt = u16::from_le_bytes(tile_bytes);
                assert!(tt < 10000, "Tile type is too high");
                tile_type = Some(tt);
            }
            else {
                let convert_tt = world.tfile_reader.read_byte();
                let convert_tt = u16::from(convert_tt);//u16::from_le_bytes(convert_tt);
                assert!(convert_tt < 10000, "Tile type is too high");
                tile_type = Some(convert_tt);
            }
        }
        Tile {
            flag_1,
            flag_2,
            flag_3,
            tile_type
        }
    }
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
        
        let tile_replacements: HashMap<u16, u16> = [
            (112 as u16, 53 as u16), //ebonstone
            (23 as u16, 2 as u16), //corrupt grass
            (163 as u16, 161 as u16), //purple ice
            (25 as u16, 1 as u16), //ebonstone
            (116 as u16, 53 as u16), //pearlsand
            (109 as u16, 2 as u16), //hallow grass
            (164 as u16, 161 as u16), //pink ice
            (117 as u16, 1 as u16), //pearlstone
            (234 as u16, 53 as u16), //crimsand
            (199 as u16, 2 as u16), //crimson grass
            (200 as u16, 161 as u16), //red ice
            (203 as u16, 1 as u16), //crimstone
            (402 as u16, 397 as u16), //hardened pearlsand block
            (399 as u16, 397 as u16), //hardened crimsand block
            (398 as u16, 397 as u16), //hardened ebonsand block
            (403 as u16, 396 as u16), //pearlsandstone
            (401 as u16, 396 as u16), //crimsandstone
            (400 as u16, 396 as u16), //ebonsandstone
        ].iter().cloned().collect();

        while self.tfile_reader.get_position() < (self.header.array_of_pointers.chests as usize) {
            running_total += self.read_tile(&tile_replacements);
        }
        running_total
    }

    pub fn read_tile(&mut self, replacement_map: &HashMap<u16, u16>) -> u32 {
        // TODO - is there a way to clean this up? Try to match things once, but keep the byte order intact?
        let tile = Tile::new(self);
        let mut count = 0;
        match tile.tile_type {
            Some(tile_type) => {
                if tile_type < 256 {
                    // TODO - delete corrupt weeds, crimson weeds. Will change the byte-structure of the tile
                    if replacement_map.contains_key(&tile_type) {
                        self.tfile_reader.replace_byte(replacement_map.get(&tile_type).unwrap().to_owned() as u8);
                        count += 1;
                    }
                }
                else {
                    //I'm assuming that the tile ID is two byte here? The current method works but it should be tested further
                    if replacement_map.contains_key(&tile_type) {
                        let replacement = replacement_map.get(&tile_type).unwrap().to_le_bytes();
                        self.tfile_reader.replace_bytes(&replacement);
                        count += 1;
                    }
                }
                let byte_position = tile_type / 8;
                let bit_position = tile_type % 8;
                let is_tile_frame_important = (self.header.tile_frame_important[byte_position as usize] & (1 << bit_position)) != 0;
                if is_tile_frame_important {
                    let _frame_x = self.tfile_reader.read_int_16();
                    let _frame_y = self.tfile_reader.read_int_16();
                }
                match tile.flag_3 {
                    Some(flag_3) => {
                        if (flag_3 & 8) != 0 { //paint of tile set if flag3 bit 3 is 1
                            let _paint_of_tile = self.tfile_reader.read_byte();
                        }
                    },
                    None => ()
                }
            }
            None => ()
        }
        if (tile.flag_1 & 4) != 0 { //type of wall set if flag1 bit 2 is 1
            let _type_of_wall = self.tfile_reader.read_byte();
            match tile.flag_3 {
                Some(flag_3) => {
                    if (flag_3 & 16) != 0 { //paint of wall set if flag3 bit 4 is 1
                        let _paint_of_wall = self.tfile_reader.read_byte();
                    }
                }
                None => ()
            }
        }
        if (tile.flag_1 & 24) != 0 { //volume of liquid set if flag1 bit 3 OR flag1 bit 4 is 1
            let _volume_of_liquid = self.tfile_reader.read_byte();
        }
        let rle = (tile.flag_1 & 192) >> 6;
        if rle == 0 {
            //No RLE
        }
        else if rle == 1 {
            let _rle_lsb = self.tfile_reader.read_byte();
        }
        else if rle == 2 {
            let _rle_msb = self.tfile_reader.read_int_16();
        }
        else {
            panic!("RLE compression error");
        }
        count
    }
}
