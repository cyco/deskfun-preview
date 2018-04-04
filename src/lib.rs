use std::ffi::CStr;
use std::fs;
use std::os::raw::c_char;
use std::path;

extern crate byteorder;
extern crate image;

pub enum OSStatus {
    NoError = 0,
}

mod point;

mod game_type;
use game_type::*;

mod palette;
use palette::*;

mod my_byte_order;

mod game_data;
use game_data::ReadGameDataExt;

mod save_game;
use save_game::ReadSaveGameExt;

mod tile_renderer;

mod zone_renderer;
use zone_renderer::ZoneRenderer;

use image::png::PNGEncoder;

extern crate elapsed;
use elapsed::measure_time;

#[no_mangle]
pub extern "C" fn generate_thumbnail(
    raw_bundle_path: *const c_char,
    raw_game_path: *const c_char,
    out_length: *mut usize,
    out_buffer: *mut *mut u8,
) -> OSStatus {
    let data_path = unsafe { path::Path::new(CStr::from_ptr(raw_bundle_path).to_str().unwrap()) };
    let game_path = unsafe { path::Path::new(CStr::from_ptr(raw_game_path).to_str().unwrap()) };
    let game_type = identify_save_game_type(game_path).expect("Save game is not valid!");

    let (elapsed, mut game_data) = measure_time(|| {
        let path = build_game_data_path(&data_path, &game_type);
        let mut buffer = fs::File::open(&path).expect("Unable to open game data file!");
        buffer
            .read_game_data()
            .expect("Unable to read game data file!")
    });
    println!("reading game file: {}", elapsed);

    let (elapsed, palette) = measure_time(|| {
        let path = build_palette_path(&data_path, &game_type);
        let mut buffer = fs::File::open(&path).expect("Unable to open palette file!");
        Palette::new(&mut buffer).expect("Unable to read palette!")
    });
    println!("reading palette: {}", elapsed);

    let (elapsed, game) = measure_time(|| {
        let mut buffer = fs::File::open(&game_path).expect("Unable to save game file!");
        buffer
            .read_save_game(&mut game_data.zones)
            .expect("Unable to read game file!")
    });
    println!("reading save game: {}", elapsed);

    let mut buffer = Vec::new();
    let (elapsed, _) = measure_time(|| {
        let renderer = ZoneRenderer::new(game_data, palette);
        let result = renderer.render(game.current_zone_id, game.position_on_zone);

        {
            let encoder = PNGEncoder::new(&mut buffer);
            encoder
                .encode(&result, 288, 288, image::ColorType::RGBA(8))
                .expect("Unable to write output file");
        }
        buffer.shrink_to_fit();
    });
    println!("rendered scene: {}", elapsed);

    unsafe {
        *out_length = buffer.len();
        *out_buffer = buffer.as_mut_ptr();
        std::mem::forget(buffer);
    };

    OSStatus::NoError
}
