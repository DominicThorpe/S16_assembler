use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

use crate::validation::validate_label;


/**
 * Takes a filename as input and generates the label table for that file where the label is the key and the 
 * address of the label is the value.
 */
pub fn get_label_table(input_file:&File) -> HashMap<String, usize> {
    let mut lable_table:HashMap<String, usize> = HashMap::new();

    let mut line_num:usize = 0;

    // filter out all empty lines and trim away whitespace
    let input_lines:Vec<String> = BufReader::new(input_file).lines().filter_map(|line| match line.unwrap().trim() {
        "" => None, 
        l => Some(l.to_string())
    }).collect();

    for line in input_lines {
        if line.ends_with(":") { // if the line is just a label
            let label = line[..line.len() - 1].to_string();

            validate_label(&label).unwrap();
            lable_table.insert(label, line_num);

            continue;
        } else if let Some(index) = line.find(":") { // if the line is a label and an instruction
            let label = line[..index].to_string();
            validate_label(&label).unwrap();

            line[..line.len() - 1].to_string();
            lable_table.insert(label, line_num);
        }

        // add 2 lines for a 16 bit instr and 4 for a 32 bit instr
        match line.to_lowercase().contains("movi") {
            true => line_num += 4,
            false => line_num += 2
        }
    }

    lable_table
}



#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;

    use super::get_label_table;


    #[test]
    fn test_label_table_generation() {
        let input_file = OpenOptions::new().read(true).open("test_files/test_label_table_gen.asm").unwrap();
        let label_table = get_label_table(&input_file);

        assert_eq!(label_table["start"], 0);
        assert_eq!(label_table["label_2"], 4);
        assert_eq!(label_table["label_3"], 6);
        assert_eq!(label_table["label_4"], 12);
    }


    #[test]
    #[should_panic]
    fn test_invalid_label() {
        let input_file = OpenOptions::new().read(true).open("test_files/test_invalid_label.asm").unwrap();
        let _ = get_label_table(&input_file);
    }
}
