use anyhow::Result;
use image::{io::Reader, GenericImageView};
use show_image::{create_window, event, Color, WindowOptions};

// This is a constant 2d array of the character portraits
type CharacterPortraits = [[Option<u8>; 6]; 25];
const CHARACTER_PORTRAITS: CharacterPortraits = [[None; 6]; 25];

#[show_image::main]
fn main() -> Result<()> {
    let dir = std::env!("CARGO_MANIFEST_DIR");
    // sprite widths are 138 (136 with a 1 pixel border)
    // 1 - 136, 138 - 273, 275 - 410
    // sprite heights are 190 (188 with a 1 pixel border)
    let sprites = Reader::open(format!("{}/assets/melee_chars.png", dir))?.decode()?;
    let width = 136;
    let height = 188;

    let mut row = 0;
    let mut col = 0;
    let mut img = sprites.view(
        1 + (col + 1) + (width * col),
        1 + (row + 1) + (height * row),
        width - 1,
        height - 1,
    );

    let window = create_window(
        "sprite",
        WindowOptions {
            background_color: Color::rgba(1., 1., 1., 1.),
            ..Default::default()
        },
    )?;
    window.set_image("char_sprite", img.to_image())?;

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
                if row < 24 {
                    row += 1;
                } else {
                    row = 0;
                }
                img.change_bounds(
                    1 + (col + 1) + (width * col),
                    1 + (row + 1) + (height * row),
                    width - 1,
                    height - 1,
                );
                window.set_image("char_sprite", img.to_image())?;
            }
            if event.input.key_code == Some(event::VirtualKeyCode::K)
                && event.input.state.is_pressed()
            {
                if row > 0 {
                    row -= 1;
                } else {
                    row = 24;
                }
                img.change_bounds(
                    1 + (col + 1) + (width * col),
                    1 + (row + 1) + (height * row),
                    width - 1,
                    height - 1,
                );
                window.set_image("char_sprite", img.to_image())?;
            }
            if event.input.key_code == Some(event::VirtualKeyCode::H)
                && event.input.state.is_pressed()
            {
                if col > 0 {
                    col -= 1;
                } else {
                    col = 5;
                }
                img.change_bounds(
                    1 + (col + 1) + (width * col),
                    1 + (row + 1) + (height * row),
                    width - 1,
                    height - 1,
                );
                window.set_image("char_sprite", img.to_image())?;
            }
            if event.input.key_code == Some(event::VirtualKeyCode::L)
                && event.input.state.is_pressed()
            {
                if col < 5 {
                    col += 1;
                } else {
                    col = 0;
                }
                img.change_bounds(
                    1 + (col + 1) + (width * col),
                    1 + (row + 1) + (height * row),
                    width - 1,
                    height - 1,
                );
                window.set_image("char_sprite", img.to_image())?;
            }
        }
    }
    Ok(())
}
