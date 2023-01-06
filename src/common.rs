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

pub fn vowels() -> Vec<char> {
    VOWELS.to_vec()
}

pub fn alphabet() -> Vec<char> {
    ALPHABET.to_vec()
}

pub fn month_codes() -> Vec<char> {
    MONTH_CODES.to_vec()
}
