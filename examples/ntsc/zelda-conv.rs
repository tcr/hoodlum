// cargo-deps: gif="*" maplit="*"
// You can also leave off the version number, in which case, it's assumed
// to be "*".  Also, the `cargo-deps` comment *must* be a single-line
// comment, and it *must* be the first thing in the file, after the
// hashbang.

#[macro_use] extern crate maplit;
extern crate gif;

use std::io::prelude::*;

fn rle(l: usize, data: Vec<u8>) -> Vec<(u16, u8)> {
    let mut rle = vec![];
    let mut last = None;
    let mut run: u16 = 0;
    for item in data {
        if Some(item) != last || (16 - run.leading_zeros()) >= l as u32 {
            if last.is_some() {
                rle.push((run, last.unwrap()))
            }
            run = 1;
            last  = Some(item);
        } else {
            run += 1;
        }
    }
    if last.is_some() {
        rle.push((run, last.unwrap()))
    }
    rle
}

fn main() {
    // Open the file
    use std::fs::File;
    use gif::SetParameter;

    let mut decoder = gif::Decoder::new(File::open("zelda-bw.gif").unwrap());
    // Configure the decoder such that it will expand the image to RGBA.
    decoder.set(gif::ColorOutput::Indexed);
    // Read the file header
    let mut decoder = decoder.read_info().unwrap();

    let mut out = File::create("zelda.hex").unwrap();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        let a = 4;
        println!("total size: {:?}", frame.buffer.len());
        //println!("rle size: {:?}", rle(a, frame.buffer.iter().cloned().collect()).len() * (a + 2));

        for c in frame.buffer[0..32768].iter() {
            out.write_all(format!("{}\n", c).as_bytes());
        }

        //println!("max {:?}", rle.iter().map(|x| x.0).max());
        //println!("rle encoded: {:?}", rle.len() * 11 / 8);

        // Tile set.
        //let mut tileset = btreeset![];
        //let buffer = frame.buffer.iter().cloned().collect::<Vec<_>>();
        //
        //const TILE_SIZE: usize = 8;
        //
        //fn get_tile(input: &[u8], x: usize, y: usize, width: usize) -> Vec<u8> {
        //    let mut tile = vec![];
        //    for offset in 0..TILE_SIZE {
        //        let start = ((x * TILE_SIZE) + (y * TILE_SIZE * width) + (offset * width)) as usize;
        //        tile.extend(&input[start..start + TILE_SIZE]);
        //    }
        //    tile
        //}
        //
        //// Seek out individual tiles.
        //let mut map = vec![];
        //for i in 0..frame.width as usize / TILE_SIZE {
        //    for j in 0..frame.height as usize / TILE_SIZE {
        //        let tile = get_tile(&buffer, i as usize, j as usize, frame.width as usize);
        //        tileset.insert(tile.clone());
        //        map.push(tileset.iter().position(|x| *x == tile).unwrap() as u8);
        //    }
        //}
        //
        //out.write_all(format!("def tiles: bit[8][{}] = {{\n", tileset.len() * 8).as_bytes());
        //for tile in &tileset {
        //    for i in 0..8 {
        //        out.write_all(format!("    0b{},\n", tile[i*8..(i+1)*8].iter().map(|x| x.to_string()).collect::<Vec<_>>().join("")).as_bytes());
        //    }
        //}
        //out.write_all(b"};\n\n");
        //out.write_all(format!("def tilemap: bit[8][{}] = {{\n", map.len()).as_bytes());
        //for tile in &map {
        //    out.write_all(format!("    0b{:08b},\n", tile).as_bytes());
        //}
        //out.write_all(b"};\n\n");
        //
        //println!("tiles: {:?}", tileset.len());
        //println!("map bytes: {:?}", (frame.width as usize / TILE_SIZE) * (frame.height as usize / TILE_SIZE) * 8);
        //println!("tile size: {:?}", tileset.len() * TILE_SIZE * TILE_SIZE * 2);
        //println!("map size: {:?}", rle(4, buffer).len() * 5);
        //let l = 4;
        //println!("tile size: {:?}", tileset.iter().map(|x| rle(l, x.clone()).len() * (l + 1)).sum::<usize>());

        // Process every frame
        //for chunk in frame.iter().chunks(4) {
        //    println!("type {:?}", chunk);
        //}
        //for chunk in frame.buffer.iter() {
            //out.write_all(format!("    0b{:03b},\n", chunk+1).as_bytes());
        //}

    }
}
