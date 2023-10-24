use iced::{
    widget::{button, column, container, image as iced_image, image::Handle, row, text},
    Element, Length, Sandbox,
};
use image::{io::Reader, DynamicImage, GenericImageView};

pub struct Slippify {
    skin: usize,
    char: usize,
    img: CharacterPortrait,
}

#[derive(Debug, Clone)]
pub enum Message {
    NextCharacter,
    PreviousCharacter,
    NextSkin,
    PreviousSkin,
}

pub struct CharacterPortrait {
    parent: DynamicImage,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl CharacterPortrait {
    fn as_vec(&self) -> Vec<u8> {
        self.parent
            .view(
                self.x as u32,
                self.y as u32,
                self.width as u32,
                self.height as u32,
            )
            .to_image()
            .to_vec()
    }
}

impl Sandbox for Slippify {
    type Message = Message;
    fn new() -> Self {
        let sprites = Reader::open(format!(
            "{}/assets/melee_chars.png",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap()
        .decode()
        .unwrap();
        Slippify {
            skin: 0,
            char: 0,
            img: CharacterPortrait {
                parent: sprites,
                width: 136,
                height: 188,
                x: 1,
                y: 1,
            },
        }
    }
    fn title(&self) -> String {
        String::from("Slippify")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NextSkin => {
                if self.skin < 5 {
                    self.skin += 1;
                } else {
                    self.skin = 0;
                }
            }
            Message::NextCharacter => {
                if self.char < 24 {
                    self.char += 1;
                } else {
                    self.char = 0;
                }
            }
            Message::PreviousCharacter => {
                if self.char > 0 {
                    self.char -= 1;
                } else {
                    self.char = 24;
                }
            }
            Message::PreviousSkin => {
                if self.skin > 0 {
                    self.skin -= 1;
                } else {
                    self.skin = 5;
                }
            }
        }
        self.img.x = (self.skin + 1) + (self.img.width * self.skin);
        self.img.y = (self.char + 1) + (self.img.height * self.char);
    }
    fn view(&self) -> Element<'_, Self::Message> {
        // put some buttons to change the char and skin
        let mut controls = row![];
        controls = controls.push(
            button("Previous Skin")
                .on_press(Message::PreviousSkin)
                .padding(12),
        );
        controls = controls.push(button("Next Skin").on_press(Message::NextSkin).padding(12));
        controls = controls.push(
            button("Previous Character")
                .on_press(Message::PreviousCharacter)
                .padding(12),
        );
        controls = controls.push(
            button("Next Character")
                .on_press(Message::NextCharacter)
                .padding(12),
        );

        let content: Element<_> = column![
            container(iced_image(Handle::from_pixels(136, 188, self.img.as_vec()))).center_x(),
            container(controls).center_x(),
            text(format!("Current Skin: {}", self.skin)),
            text(format!("Current Char: {}", self.char))
        ]
        .max_width(680)
        .spacing(20)
        .padding(20)
        .into();
        container(content).width(Length::Fill).center_x().into()
    }
    // Below are to be implemented later

    // fn theme(&self) -> iced::Theme {}
    // fn style(&self) -> iced::theme::Application {}
    // fn scale_factor(&self) -> f64 {}
    // fn run(settings: Settings<()>) -> Result<(), iced::Error>
    // where
    //     Self: 'static + Sized,
    // {
    // }
}
