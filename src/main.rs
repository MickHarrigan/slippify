use anyhow::Result;
use image::{io::Reader, GenericImageView};
use show_image::{create_window, event, Color, WindowOptions};

// This is a constant 2d array of the character portraits
// u8 is a placeholder type for the later type found from reading and parsing that will be done with the actual app
type CharacterPortraits = [[Option<u8>; 6]; 25];
const CHARACTER_PORTRAITS: CharacterPortraits = [
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Dr. Mario
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Mario
    [Some(0), Some(1), Some(2), Some(3), None, None],    // Luigi
    [Some(0), Some(1), Some(2), Some(3), None, None],    // Bowser
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Peach
    [Some(0), Some(1), Some(2), Some(3), Some(4), Some(5)], // Yoshi
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Donkey Kong
    [Some(0), Some(1), Some(2), Some(3), Some(4), Some(5)], // Captain Falcon
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Ganondorf
    [Some(0), Some(1), Some(2), Some(3), None, None],    // Falco
    [Some(0), Some(1), Some(2), Some(3), None, None],    // Fox
    [Some(0), Some(1), Some(2), Some(3), None, None],    // Ness
    [Some(0), Some(1), Some(2), Some(3), None, None],    // Ice Climbers
    [Some(0), Some(1), Some(2), Some(3), Some(4), Some(5)], // Kirby
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Samus
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Zelda
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Link
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Young Link
    [Some(0), Some(1), Some(2), Some(3), None, None],    // Pichu
    [Some(0), Some(1), Some(2), Some(3), None, None],    // Pikachu
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Jigglypuff
    [Some(0), Some(1), Some(2), Some(3), None, None],    // Mewtwo
    [Some(0), Some(1), Some(2), Some(3), None, None],    // Mr. Game and Watch
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Marth
    [Some(0), Some(1), Some(2), Some(3), Some(4), None], // Roy
];

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
