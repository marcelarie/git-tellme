use colored::Colorize;

// use colored::*;

pub struct Box<'content> {
    sides: BoxSides,
    width: usize,
    subject_type: String,
    content: &'content [String],
}

struct BoxSides {
    horizontal_side: String,
    vertical_side: String,
    top_left_corner: String,
    bottom_left_corner: String,
    top_right_corner: String,
    bottom_right_corner: String,
}

impl Box<'_> {
    fn calculate_box_width(&mut self) {
        for box_item in self.content.iter() {
            if box_item.chars().count() > self.width {
                self.width = box_item.chars().count()
            };
        }
    }

    fn get_box_width(&mut self) -> String {
        let _ = &self.calculate_box_width();
        let mut box_width = String::new();
        for _ in 0..self.width {
            box_width.push_str(&self.sides.horizontal_side)
        }
        box_width
    }

    fn get_box_color(&mut self) -> (u8, u8, u8) {
        let color = match self.subject_type.as_str() {
            "Issue" => (211, 160, 77),
            "Discussion" => (251, 241, 199),
            _ => (123, 146, 70),
        };
        color
    }

    pub fn draw_box<'c>(&mut self) {
        let horizontal_side = &self.get_box_width().clone();

        let mut upper_box = String::new();
        let _ = [
            &self.sides.top_left_corner,
            &self.sides.horizontal_side,
            horizontal_side,
            &self.sides.horizontal_side,
            &self.sides.top_right_corner,
        ]
        .map(|s| upper_box.push_str(s));

        let mut lower_box = String::new();
        let _ = [
            &self.sides.bottom_left_corner,
            &self.sides.horizontal_side,
            horizontal_side,
            &self.sides.horizontal_side,
            &self.sides.bottom_right_corner,
        ]
        .map(|s| lower_box.push_str(s));

        let (r, g, b) = self.get_box_color();

        println!("{}", upper_box.truecolor(r, g, b));
        for box_word in self.content.iter() {
            let word_count = box_word.chars().count();
            let width = self.get_box_width().chars().count();
            let mut box_empty_space = String::from("");
            if word_count < width {
                for _ in 0..width - word_count {
                    box_empty_space.push_str(" ")
                }
            };
            println!(
                "{} {}{} {}",
                self.sides.vertical_side.truecolor(r, g, b),
                box_word,
                box_empty_space,
                self.sides.vertical_side.truecolor(r, g, b),
            )
        }
        println!("{}", lower_box.truecolor(r, g, b));
    }
}

pub fn draw_box<'c>(content: &'c [String], subject_type: String) {
    let mut the_box = Box {
        sides: BoxSides {
            horizontal_side: String::from("━"),
            vertical_side: String::from("┃"),
            top_left_corner: String::from("┏"),
            bottom_left_corner: String::from("┗"),
            top_right_corner: String::from("┓"),
            bottom_right_corner: String::from("┛"),
        },
        width: 0,
        subject_type,
        content,
    };

    the_box.draw_box();
}
