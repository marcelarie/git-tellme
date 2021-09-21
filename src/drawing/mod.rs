use colored::*;
pub fn draw_box(content: &[String], subject_type: String) {
    // get max size
    let mut max_width = 0;
    for item in content.iter() {
        if item.chars().count() > max_width {
            max_width = item.chars().count()
        };
    }

    //============>   0    1    2    3    4    5
    let box_lines = ["┏", "━", "┓", "┃", "┗", "┛"];
    //  let box_lines = ["🭽", "▔", "🭾", "▕", "🭿", "▁", "🭼", "▏"];

    // generate box_width
    let mut box_width = box_lines[1].to_owned();
    for _ in 0..max_width {
        box_width.push_str(box_lines[1])
    }

    let upper_box = [
        box_lines[0],
        box_lines[1],
        &box_width,
        box_lines[1],
        box_lines[2],
    ]
    .join("");
    let lower_box = [
        box_lines[4],
        box_lines[1],
        &box_width,
        box_lines[1],
        box_lines[5],
    ]
    .join("");

    // print full box
    // color by type
    if subject_type == "Issue" {
        println!("{}", upper_box.truecolor(211, 160, 77))
    } else {
        println!("{}", upper_box.truecolor(123, 146, 70))
    };
    for item in content.iter() {
        let count = item.chars().count();
        let mut space_size = " ".to_owned();
        if count < max_width {
            for _ in 0..max_width - count {
                space_size.push_str(" ")
            }
        };

        if subject_type == "Issue" {
            println!(
                "{} {}{} {}",
                box_lines[3].truecolor(211, 160, 77),
                item,
                space_size,
                box_lines[3].truecolor(211, 160, 77)
            )
        } else {
            println!(
                "{} {}{} {}",
                box_lines[3].truecolor(123, 146, 70),
                item,
                space_size,
                box_lines[3].truecolor(123, 146, 70)
            )
        };
    }
    if subject_type == "Issue" {
        println!("{}", lower_box.truecolor(211, 160, 77));
    } else {
        println!("{}", lower_box.truecolor(123, 146, 70));
    };
}
