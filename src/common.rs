const VOWELS: &[char] = &['A', 'E', 'I', 'O', 'U'];

const ALPHABET: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const MONTH_CODES: &[char] = &[
    'A', // January
    'B', // Febraury
    'C', // March
    'D', // April
    'E', // May
    'H', // June
    'L', // July
    'M', // August
    'P', // September
    'R', // October
    'S', // November
    'T', // December
];

pub fn is_vowel(letter: &char) -> bool {
    VOWELS.contains(letter)
}

pub fn is_consonant(letter: &char) -> bool {
    !VOWELS.contains(letter)
}

pub fn to_alphabet(index: usize) -> char {
    ALPHABET[index]
}

pub fn to_month_codes(index: usize) -> char {
    MONTH_CODES[index]
}

pub fn is_month_code(letter: &char) -> bool {
    MONTH_CODES.contains(letter)
}
