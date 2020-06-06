use super::TFileReader;
use std::str;

pub struct Header {
    pub release: i32,
    pub magic_number: [u8; 7],
    pub file_type: u8,
    pub revision: u32,
    pub is_favorite: u64,
    pub num_of_pointers: u16,
    pub array_start_address: usize,
    pub array_of_pointers: FilePointers
}

impl Header {
    pub fn new(world: &mut TFileReader) -> Header {
        let release = world.read_int_32(); //0..4
        let mut magic_number: [u8; 7] = [0; 7];
        world.read_multiple_bytes(&mut magic_number); //4..11
        assert_eq!(str::from_utf8(&magic_number).unwrap(), "relogic", "Magic number verification failed");
        let file_type = world.read_byte(); //1 for .MAP, 2 for .WLD, 3 for .PLR
        let revision = world.read_int_32() as u32; //Incremented every time the file is saved
        let is_favorite = world.read_int_64() as u64;
        let num_of_pointers = world.read_int_16() as u16; //24..26
        let array_start_address = world.get_position();
        let array_of_pointers = FilePointers::new(world, num_of_pointers);
        Header {
            release,
            magic_number,
            file_type,
            revision,
            is_favorite,
            num_of_pointers,
            array_start_address,
            array_of_pointers
        }
    }
}

pub struct FilePointers {
    pub header: i32,
    pub tiles: i32,
    pub chests: i32,
    pub signs: i32,
    pub npc: i32,
    pub entities: i32,
    pub footer: i32,
    pub unknown_1: i32,
    pub unknown_2: i32,
    pub unknown_3: i32,
    pub unknown_4: i32
}

impl FilePointers {
    fn new(world: &mut TFileReader, _amount: u16) -> FilePointers {
        let header = world.read_int_32(); //26..30
        let tiles = world.read_int_32(); //30..34
        let chests = world.read_int_32(); //34..38
        let signs = world.read_int_32(); //38..42
        let npc = world.read_int_32(); //42..46
        let entities = world.read_int_32(); //46..50
        let footer = world.read_int_32(); //50..54
        let unknown_1 = world.read_int_32();
        let unknown_2 = world.read_int_32();
        let unknown_3 = world.read_int_32();
        let unknown_4 = world.read_int_32();
        FilePointers {
            header,
            tiles,
            chests,
            signs,
            npc,
            entities,
            footer,
            unknown_1,
            unknown_2,
            unknown_3,
            unknown_4
        }
    }
}