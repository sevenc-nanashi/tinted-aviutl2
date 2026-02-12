static START_MARKER: &str = "; -- tinted-aviutl2-style-start --";
static END_MARKER: &str = "; -- tinted-aviutl2-style-end --";

pub fn merge_style(original: &str, new_style: &str) -> String {
    let mut result = String::new();
    let mut in_inserted_style_section = false;
    let mut is_existing_color_section = false;

    for line in original.lines() {
        if line.trim() == START_MARKER {
            in_inserted_style_section = true;
            result.push_str(START_MARKER);
            result.push('\n');
            result.push_str(new_style);
            result.push('\n');
        } else if line.trim() == END_MARKER {
            in_inserted_style_section = false;
            result.push_str(END_MARKER);
            result.push('\n');
        } else if in_inserted_style_section {
            // Skip
        } else if line.trim().to_lowercase() == "[color]" {
            result.push_str("; [Color]\n");
            is_existing_color_section = true;
        } else if line.trim().starts_with('[') && line.trim().ends_with(']') {
            result.push_str(line);
            result.push('\n');
            is_existing_color_section = false;
        } else if is_existing_color_section {
            if line.trim().is_empty() {
                result.push_str(";\n");
            } else {
                result.push_str("; ");
                result.push_str(line);
                result.push('\n');
            }
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    // If the original did not contain the markers, add the new style at the end
    if !result.contains(START_MARKER) && !result.contains(END_MARKER) {
        result.push('\n');
        result.push_str(START_MARKER);
        result.push('\n');
        result.push_str(new_style);
        result.push('\n');
        result.push_str(END_MARKER);
        result.push('\n');
    }

    result
}
