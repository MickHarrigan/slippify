type CharacterPortraits = [[Option<u8>; 6]; 25];
pub const CHARACTER_PORTRAITS: CharacterPortraits = [
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

pub struct CharacterImage {
    /// The top left corner pixel of the character's icon in the CSS
    icon: (usize, usize),
    /// The index of the skin used in the `CHARACTER_PORTRAITS`
    skin: usize,
}
