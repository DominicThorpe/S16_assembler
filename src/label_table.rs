use crate::validation::validate_label;

pub fn get_label_table_entry_from_line(line_num:usize, line:&str) -> Option<(String, usize)> {
    match line.find(":") {
        Some(index) => {
            validate_label(&line[..index]).unwrap();
            Some((line[..index].to_string(), line_num))
        },

        None => None
    }
}