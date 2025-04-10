/*
this file shows the use of OOP approach to building a gui application in rust
*/

pub trait Draw {
    fn draw(&self) {}
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing Button height: {}, width: {}, label: {}",
            self.height, self.width, self.label
        );
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Close"),
            }),
        ],
    };

    screen.run();
}
