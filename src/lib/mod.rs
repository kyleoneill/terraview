use std::error::Error;

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
    let mut world = World::new(&world_path.as_ref());

    let name = world.world_name();

    //Asserting that the specified and discovered world names are equal verifies that the file was read as expected
    assert_eq!(world_name, name, "File world name and found world name do not match, world file was read incorrectly.");

    println!("Successfully read data for world '{}' with version {}.\nFound world tile section located at offset: {:#X}", name, world.header.release, world.header.array_of_pointers.tiles);
    let modified_count = world.iterate_tiles();
    println!("Replaced {} tile(s)", modified_count);

    // let modified_world_name = format!("worlds/{}_modified.wld", world_name);
    // match world.save_world(&modified_world_name.as_ref()) {
    //     Ok(_i32) => println!("Successively saved world file"),
    //     Err(_) => println!("Failed to save world file")
    // };

    Ok(0)
}
