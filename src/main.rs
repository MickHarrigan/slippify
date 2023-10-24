use anyhow::Result;
use iced::{Sandbox, Settings};
#[cfg(feature = "show_image")]
use image::{io::Reader, GenericImageView};
#[cfg(feature = "show_image")]
use show_image::{create_window, event, Color, WindowOptions};

// This is a constant 2d array of the character portraits
// u8 is a placeholder type for the later type found from reading and parsing that will be done with the actual app

mod app;
mod characters;
use app::*;
use characters::*;

#[cfg(not(feature = "show_image"))]
fn main() -> Result<()> {
    Slippify::run(Settings::default()).map_err(|e| e.into())
}

#[cfg(feature = "show_image")]
#[show_image::main]
fn main() -> Result<()> {
    let dir = std::env!("CARGO_MANIFEST_DIR");
    // sprite widths are 138 (136 with a 1 pixel border)
    // 1 - 136, 138 - 273, 275 - 410
    // sprite heights are 190 (188 with a 1 pixel border)
    let sprites = Reader::open(format!("{}/assets/melee_chars.png", dir))?.decode()?;
    let width = 136;
    let height = 188;

    let mut row = 0usize;
    let mut col = 0usize;
    let mut img = sprites.view(
        1 + (col + 1) as u32 + (width * col) as u32,
        1 + (row + 1) as u32 + (height * row) as u32,
        (width - 1) as u32,
        (height - 1) as u32,
    );

    let window = create_window(
        "sprite",
        WindowOptions {
            background_color: Color::rgba(1., 1., 1., 0.),
            ..Default::default()
        },
    )?;
    window.set_image("char_sprite", img.to_image())?;

    // next make it that it only shows the skins that exist, no None's should be shown.
    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            // println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }

            if event.input.key_code == Some(event::VirtualKeyCode::J)
                && event.input.state.is_pressed()
            {
                if row < 24 && CHARACTER_PORTRAITS[row + 1][col].is_some() {
                    row += 1;
                } else if row < 24 {
                    row += 1;
                    while CHARACTER_PORTRAITS[row][col].is_none() {
                        col -= 1;
                    }
                } else {
                    row = 0;
                }
                img.change_bounds(
                    1 + (col + 1) as u32 + (width * col) as u32,
                    1 + (row + 1) as u32 + (height * row) as u32,
                    (width - 1) as u32,
                    (height - 1) as u32,
                );
                window.set_image("char_sprite", img.to_image())?;
            }
            if event.input.key_code == Some(event::VirtualKeyCode::K)
                && event.input.state.is_pressed()
            {
                if row > 0 && CHARACTER_PORTRAITS[row - 1][col].is_some() {
                    row -= 1;
                } else if row > 0 {
                    row -= 1;
                    while CHARACTER_PORTRAITS[row][col].is_none() {
                        col -= 1;
                    }
                } else {
                    row = 24;
                }
                img.change_bounds(
                    1 + (col + 1) as u32 + (width * col) as u32,
                    1 + (row + 1) as u32 + (height * row) as u32,
                    (width - 1) as u32,
                    (height - 1) as u32,
                );
                window.set_image("char_sprite", img.to_image())?;
            }
            if event.input.key_code == Some(event::VirtualKeyCode::H)
                && event.input.state.is_pressed()
            {
                if col > 0 && CHARACTER_PORTRAITS[row][col - 1].is_some() {
                    col -= 1;
                } else {
                    // set it to the max non-None value
                    col = match CHARACTER_PORTRAITS[row].partition_point(|a| a.is_some()) {
                        0 => 5,
                        n => n - 1,
                    };
                }
                img.change_bounds(
                    1 + (col + 1) as u32 + (width * col) as u32,
                    1 + (row + 1) as u32 + (height * row) as u32,
                    (width - 1) as u32,
                    (height - 1) as u32,
                );
                window.set_image("char_sprite", img.to_image())?;
            }
            if event.input.key_code == Some(event::VirtualKeyCode::L)
                && event.input.state.is_pressed()
            {
                if col < 5 && CHARACTER_PORTRAITS[row][col + 1].is_some() {
                    col += 1;
                } else {
                    col = 0;
                }
                img.change_bounds(
                    1 + (col + 1) as u32 + (width * col) as u32,
                    1 + (row + 1) as u32 + (height * row) as u32,
                    (width - 1) as u32,
                    (height - 1) as u32,
                );
                window.set_image("char_sprite", img.to_image())?;
            }
        }
    }
    Ok(())
}
