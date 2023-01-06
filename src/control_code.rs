const ALPHABET: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct ControlCode {}

impl ControlCode {
    pub fn compute(codice_fiscale: &str) -> char {
        let partial_code: Vec<char> = codice_fiscale.to_uppercase().chars().collect();
        let mut control_code = 0;

        for (n, character) in partial_code.iter().enumerate().take(15) {
            if let Some(val) = get_conversion_table_value(character, (n + 1) % 2 == 0) {
                control_code += val;
            }
        }

        let index_alphabet: usize = (&control_code % 26).try_into().unwrap();
        ALPHABET[index_alphabet]
    }
}

fn get_conversion_table_value(character: &char, even: bool) -> Option<i32> {
    let ternary = |even_value: i32, odd_value: i32| {
        if even {
            Some(even_value)
        } else {
            Some(odd_value)
        }
    };

    match character.to_owned() {
        'A' | '0' => ternary(0, 1),
        'B' | '1' => ternary(1, 0),
        'C' | '2' => ternary(2, 5),
        'D' | '3' => ternary(3, 7),
        'E' | '4' => ternary(4, 9),
        'F' | '5' => ternary(5, 13),
        'G' | '6' => ternary(6, 15),
        'H' | '7' => ternary(7, 17),
        'I' | '8' => ternary(8, 19),
        'J' | '9' => ternary(9, 21),
        'K' => ternary(10, 2),
        'L' => ternary(11, 4),
        'M' => ternary(12, 18),
        'N' => ternary(13, 20),
        'O' => ternary(14, 11),
        'P' => ternary(15, 3),
        'Q' => ternary(16, 6),
        'R' => ternary(17, 8),
        'S' => ternary(18, 12),
        'T' => ternary(19, 14),
        'U' => ternary(20, 16),
        'V' => ternary(21, 10),
        'W' => ternary(22, 22),
        'X' => ternary(23, 25),
        'Y' => ternary(24, 24),
        'Z' => ternary(25, 23),
        _ => None,
    }
}
