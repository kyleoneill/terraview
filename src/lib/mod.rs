use std::error::Error;
use std::time::Instant;
use std::path::PathBuf;

mod tfilereader;
use tfilereader::TFileReader;

mod header;
use header::Header;

mod world;
use world::World;

extern crate dirs;

/**
should decompress data. Data is currently stored with RLE
If I want to make changes, I should read the tile section and
build a full representation in program memory, make changes,
and then re-encode it back to the stored format and
overwrite the file
**/
pub fn run(world_name: String) -> Result<i32, Box<dyn Error>> {
    let start = Instant::now();

    let (original_file, modified_file) = get_file_paths(&world_name);

    //make a loading bar or something here to get the status. Make it re-usable for world.iterate_tiles()
    let mut world = World::new(&original_file);

    let name = world.world_name();

    //Asserting that the specified and discovered world names are equal verifies that the file was read as expected
    assert_eq!(world_name, name, "File world name and found world name do not match, world file was read incorrectly.");

    println!("Successfully read data for world '{}' with version {}.\nFound world tile section located at offset: {:#X}", name, world.header.release, world.header.array_of_pointers.tiles);
    let modified_count = world.iterate_tiles();
    println!("Replaced {} evil and hallow tile(s)", modified_count);

    match world.save_world(&modified_file.as_ref()) {
        Ok(_i32) => println!("Successively saved world file.\nRuntime: {}ms", start.elapsed().as_millis()),
        Err(_) => println!("Failed to save world file")
    };

    Ok(0)
}

fn get_file_paths(world_name: &String) -> (PathBuf, PathBuf) {
    let mut world_name = world_name.to_owned();

    let mut world_path: PathBuf = dirs::home_dir().unwrap();
    world_path.push("Documents/My Games/Terraria/Worlds/");

    let mut original_world_name = world_name.clone();
    original_world_name.push_str(".wld");
    let original_file = world_path.join(&original_world_name);
    assert!(original_file.is_file(), "Could not locate Terraria world folder.");

    world_name.push_str("_modified.wld");
    let modified_file = world_path.join(&world_name);

    (original_file, modified_file)
}
