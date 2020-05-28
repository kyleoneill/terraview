use super::TFileReader;
use super::Header;

pub struct World {
    pub tfile_reader: TFileReader,
    pub header: Header
}

impl World {
    pub fn new(tfile_reader: TFileReader, header: Header) -> World {
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

    pub fn iterate_world_tiles(&mut self) -> i32 {
        self.tfile_reader.move_to_position(self.header.array_of_pointers.tiles);
        for tile in self.header.array_of_pointers.tiles..self.header.array_of_pointers.chests {
            self.read_tile();
        }
        0
    }

    pub fn read_tile(&mut self) -> u32 {
        let byte = self.tfile_reader.read_byte();
        return 0
    }
}
