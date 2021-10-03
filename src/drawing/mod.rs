// use colored::*;

pub struct Box<'content> {
    sides: BoxSides,
    width: usize,
    subject_type: String,
    content: &'content [String],
    color: (u32, u32, u32),
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

    fn get_box_color(&mut self) {
        let color = match self.subject_type.as_str() {
            "Issue" => (211, 160, 77),
            _ => (123, 146, 70),
            // None => panic!("No subject type found on Box."),
        };
        self.color = color;
    }

    pub fn draw_box<'c>(&mut self) {
        let mut upper_box = String::new();
        let mut lower_box = String::new();
        let horizontal_side = &self.get_box_width().clone();

        let _ = [
            &self.sides.top_left_corner,
            &self.sides.horizontal_side,
            horizontal_side,
            &self.sides.horizontal_side,
            &self.sides.top_right_corner,
        ]
        .map(|s| upper_box.push_str(s));

        let _ = [
            &self.sides.bottom_left_corner,
            &self.sides.horizontal_side,
            horizontal_side,
            &self.sides.horizontal_side,
            &self.sides.bottom_right_corner,
        ]
        .map(|s| lower_box.push_str(s));

        let _ = self.get_box_color();

        // TODO: BOX COLOR
        println!("{}", upper_box);
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
                self.sides.vertical_side, box_word, box_empty_space, self.sides.vertical_side,
            )
        }
        println!("{}", lower_box);
    }
}

pub fn draw_box<'c>(content: &'c [String], subject_type: String) {
    let box_sides = BoxSides {
        horizontal_side: String::from("━"),
        vertical_side: String::from("┃"),
        top_left_corner: String::from("┏"),
        bottom_left_corner: String::from("┗"),
        top_right_corner: String::from("┓"),
        bottom_right_corner: String::from("┛"),
    };

    let mut the_box = Box {
        sides: box_sides,
        width: 0,
        color: (0, 0, 0),
        subject_type,
        content,
    };

    the_box.draw_box();
}
