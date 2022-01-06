pub fn color_string_splitter(color: &String) -> Vec<&str> {
    if color.contains(",") {
        color.trim().split(",").collect::<Vec<&str>>()
    } else {
        color.trim().split_ascii_whitespace().collect::<Vec<&str>>()
    }
}
