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

    let mut data_mode = true;
    let mut line_num:usize = 0;

    // filter out all empty lines and trim away whitespace
    let input_lines:Vec<String> = BufReader::new(input_file).lines().filter_map(|line| match line.unwrap().trim() {
        "" => None, 
        l => Some(l.to_string())
    }).collect();

    for line in input_lines {
        // if the data section has ended, move into code mode
        if line.contains("code:") {
            data_mode = false;
            continue
        }

        // if the line is just a label
        if line.ends_with(":") { 
            let label = line[..line.len() - 1].to_string();

            validate_label(&label).unwrap();
            lable_table.insert(label, line_num);

            continue;
        } 
        
        // if the line is a label and an instruction or data
        else if let Some(index) = line.find(":") { 
            let label = line[..index].to_string();
            validate_label(&label).unwrap();

            line[..line.len() - 1].to_string();
            lable_table.insert(label, line_num);
        }

        if data_mode == true {
            let data = match line.find(":") {
                Some(index) => &line[index + 1..],
                None => &line
            };

            let tokens:Vec<&str> = data.split_whitespace().collect();
            match *tokens.get(0).unwrap() {
                ".byte" => line_num += 1,
                ".word" => line_num += 2,
                ".long" => line_num += 4,
                ".array" => line_num += tokens.len() - 1,
                ".asciiz" => line_num += line[line.find("`").unwrap()..line.len() - 1].len() + 1,
                invalid => panic!("{} is not a valid datatype", invalid)
            }
        }

        // add 2 lines for a 16 bit instr and 4 for a 32 bit instr
        else {
            match line.to_lowercase().contains("movi") {
                true => line_num += 4,
                false => line_num += 2
            }
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

        println!("{:#?}", label_table);

        assert_eq!(label_table["my_byte"], 0);
        assert_eq!(label_table["my_word"], 1);
        assert_eq!(label_table["my_long"], 3);
        assert_eq!(label_table["my_array"], 7);
        assert_eq!(label_table["my_ascii"], 12);

        assert_eq!(label_table["start"], 26);
        assert_eq!(label_table["label_2"], 30);
        assert_eq!(label_table["label_3"], 32);
        assert_eq!(label_table["label_4"], 38);
    }


    #[test]
    #[should_panic]
    fn test_invalid_label() {
        let input_file = OpenOptions::new().read(true).open("test_files/test_invalid_label.asm").unwrap();
        let _ = get_label_table(&input_file);
    }
}
