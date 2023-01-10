use std::str::Chars;

pub struct Omocodes {}

impl Omocodes {
    #[allow(dead_code)]
    pub fn generate(codice_fiscale: Chars) -> Vec<Vec<char>> {
        let mut starting_codice_fiscale: Vec<char> = codice_fiscale.collect::<Vec<char>>().to_vec();
        let omocodes_letter_indices = [14, 13, 12, 10, 9, 7, 6];
        let mut omocodes: Vec<Vec<char>> = vec![];

        for index in omocodes_letter_indices {
            starting_codice_fiscale[index] =
                inverse_map_omocodes(starting_codice_fiscale[index]).unwrap();
            omocodes.push(starting_codice_fiscale.to_vec())
        }

        omocodes
    }

    pub fn replace_omocodes_characters(value: &str) -> String {
        let mut purified_value: Vec<char> = value.chars().collect();
        let omocodes_letter_indices = [14, 13, 12, 10, 9, 7, 6];
        for index in omocodes_letter_indices {
            let current_letter = purified_value[index];
            if !current_letter.is_ascii_alphabetic() {
                break;
            }

            let map_current_letter = map_omocodes(current_letter);
            if map_current_letter.is_none() {
                return value.to_string();
            }

            purified_value[index] =
                map_current_letter.expect("mapping shoud be always populated here");
        }

        return purified_value.iter().collect();
    }
}

fn map_omocodes(letter: char) -> Option<char> {
    match letter {
        'L' | 'l' => Some('0'),
        'M' | 'm' => Some('1'),
        'N' | 'n' => Some('2'),
        'P' | 'p' => Some('3'),
        'Q' | 'q' => Some('4'),
        'R' | 'r' => Some('5'),
        'S' | 's' => Some('6'),
        'T' | 't' => Some('7'),
        'U' | 'u' => Some('8'),
        'V' | 'v' => Some('9'),
        _ => None,
    }
}

fn inverse_map_omocodes(letter: char) -> Option<char> {
    match letter {
        '0' => Some('L'),
        '1' => Some('M'),
        '2' => Some('N'),
        '3' => Some('P'),
        '4' => Some('Q'),
        '5' => Some('R'),
        '6' => Some('S'),
        '7' => Some('T'),
        '8' => Some('U'),
        '9' => Some('V'),
        _ => None,
    }
}

// omocodes SRRVCN90B1SK83VS - BRNPRZ72D52F83VC - SCKCKH79A01Z34PP
