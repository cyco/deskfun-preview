extern crate byteorder;
extern crate elapsed;
extern crate encoding;
extern crate image;

#[repr(u64)]
pub enum OSStatus {
    NoError = 0,
    SomeError = 1,
}

use elapsed::measure_time;
use image::png::PNGEncoder;
use std::ffi::CStr;
use std::fs;
use std::os::raw::c_char;
use std::panic;
use std::path;

mod game_data;
mod game_type;
mod io;
mod palette;
mod point;
mod save_game;
mod tile_renderer;
mod zone_renderer;

use game_data::ReadGameDataExt;
use game_type::*;
use palette::*;
use save_game::ReadSaveGameExt;
use zone_renderer::render_zone;

fn build_game_data_path(base_path: &path::Path, game_type: GameType) -> path::PathBuf {
    match game_type {
        GameType::Yoda => base_path.join(path::Path::new("Contents/Resources/yoda.data")),
        GameType::Indy => base_path.join(path::Path::new("Contents/Resources/indy.data")),
    }
}

fn build_palette_path(base_path: &path::Path, game_type: GameType) -> path::PathBuf {
    match game_type {
        GameType::Yoda => base_path.join(path::Path::new("Contents/Resources/yoda.pal")),
        GameType::Indy => base_path.join(path::Path::new("Contents/Resources/indy.pal")),
    }
}

#[no_mangle]
pub extern "C" fn generate_thumbnail(
    raw_bundle_path: *const c_char,
    raw_game_path: *const c_char,
    out_length: *mut usize,
    out_buffer: *mut *mut u8,
) -> OSStatus {
    match panic::catch_unwind(|| {
        let data_path =
            unsafe { path::Path::new(CStr::from_ptr(raw_bundle_path).to_str().unwrap()) };
        let game_path = unsafe { path::Path::new(CStr::from_ptr(raw_game_path).to_str().unwrap()) };

        let mut buffer = fs::File::open(&game_path).expect("Unable to save game file!");
        let (elapsed, (game_type, save_game_reader)) =
            measure_time(|| buffer.read_save_game().expect("Unable to read game file!"));
        println!("determining save type: {}", elapsed);

        let (elapsed, mut game_data) = measure_time(|| {
            let path = build_game_data_path(&data_path, game_type);
            let mut buffer = fs::File::open(&path).expect("Unable to open game data file!");
            buffer
                .read_game_data(game_type)
                .expect("Unable to read game data file!")
        });
        println!("reading game file: {}", elapsed);

        let (elapsed, palette) = measure_time(|| {
            let path = build_palette_path(&data_path, game_type);
            let mut buffer = fs::File::open(&path).expect("Unable to open palette file!");
            Palette::new(&mut buffer).expect("Unable to read palette!")
        });
        println!("reading palette: {}", elapsed);

        let (elapsed, game) = measure_time(|| {
            save_game_reader(&mut buffer, &mut game_data.zones)
                .expect("Unable to read game data file!")
        });
        println!("reading save game: {}", elapsed);

        let mut buffer = Vec::new();
        let (elapsed, _) = measure_time(|| {
            let result = render_zone(
                &game_data,
                &palette,
                game.current_zone_id,
                game.position_on_zone,
            );
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
    }) {
        Err(why) => {
            println!("error: {:?}", why);
            OSStatus::SomeError
        }
        _ => OSStatus::NoError,
    }
}
