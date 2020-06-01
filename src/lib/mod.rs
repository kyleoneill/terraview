use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead};

mod tfilereader;
use tfilereader::TFileReader;

mod header;
use header::Header;

mod world;
use world::World;

/**
should decompress data. Data is currently stored with RLE
If I want to make changes, I should read the tile section and
build a full representation in program memory, make changes,
and then re-encode it back to the stored format and
overwrite the file
**/
pub fn run() -> Result<i32, Box<dyn Error>> {
    let world_name = "Corruption Farm".to_string();
    let world_path = format!("worlds/{}.wld", world_name);

    //make a loading bar or something here to get the status. Make it re-usable for world.iterate_tiles()
    let file = File::open(world_path).expect("Unable to open world file");
    let bytes = file.bytes().collect::<Result<Vec<_>, _>>().unwrap();

    let mut tfile_reader = TFileReader::new(bytes);
    let header = Header::new(&mut tfile_reader);
    let mut world = World::new(tfile_reader, header);

    let name = world.world_name();

    //Asserting that the specified and discovered world names are equal verifies that the file was read as expected
    assert_eq!(world_name, name, "File world name and found world name do not match, world file was read incorrectly.");

    println!("Successfully read data for world '{}' with version {}.\nFound world tile section located at offset: {:#X}", name, world.header.release, world.header.array_of_pointers.tiles);

    let crystal_heart_count = world.iterate_tiles();
    print!("World has {} crystal heart tiles", crystal_heart_count);

    Ok(0)
}
