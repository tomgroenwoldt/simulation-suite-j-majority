pub fn map_value_to_color(value: u64, lowest_value: u64, highest_value: u64) -> (u8, u8, u8) {
    // Define the number of color categories
    let num_categories: u64 = 100;

    // Calculate the value range per category
    let value_range = (highest_value - lowest_value) / num_categories;

    // Calculate the category index based on the value
    let category_index = (value - lowest_value) / value_range;

    // Calculate the RGB components based on the category index
    let red = (category_index * 255) / num_categories;
    let green = 255 - ((category_index * 255) / num_categories);

    (red as u8, green as u8, 0)
}
