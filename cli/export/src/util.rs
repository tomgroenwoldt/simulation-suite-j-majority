use pgfplots::axis::plot::{MarkShape, PlotKey};

pub fn map_value_to_color(value: u64, lowest_value: u64, highest_value: u64) -> (u8, u8, u8) {
    // Define the number of color categories
    let num_categories: u64 = 25;

    // Calculate the value range per category
    let value_range = (highest_value - lowest_value) / num_categories;

    // Calculate the category index based on the value
    let category_index = (value - lowest_value) / value_range;

    // Calculate the RGB components based on the category index
    let red = (category_index * 255) / num_categories;
    let green = 255 - ((category_index * 255) / num_categories);

    (red as u8, green as u8, 0)
}

pub fn map_sample_size_to_markshape(value: u8) -> MarkShape {
    match value {
        3 => MarkShape::Plus,
        4 => MarkShape::X,
        5 => MarkShape::Asterisk,
        6 => MarkShape::Square,
        7 => MarkShape::SquareFilled,
        8 => MarkShape::O,
        9 => MarkShape::OFilled,
        10 => MarkShape::Triangle,
        11 => MarkShape::TriangleFilled,
        12 => MarkShape::Diamond,
        _ => MarkShape::DiamondFilled,
    }
}

pub fn map_sample_size_to_color(value: u8) -> PlotKey {
    match value {
        3 => PlotKey::Custom(String::from("color=red")),
        4 => PlotKey::Custom(String::from("color=green")),
        5 => PlotKey::Custom(String::from("color=blue")),
        6 => PlotKey::Custom(String::from("color=cyan")),
        7 => PlotKey::Custom(String::from("color=magenta")),
        8 => PlotKey::Custom(String::from("color=brown")),
        9 => PlotKey::Custom(String::from("color=violet")),
        10 => PlotKey::Custom(String::from("color=orange")),
        11 => PlotKey::Custom(String::from("color=darkgray")),
        12 => PlotKey::Custom(String::from("color=teal")),
        _ => PlotKey::Custom(String::from("color=black")),
    }
}
