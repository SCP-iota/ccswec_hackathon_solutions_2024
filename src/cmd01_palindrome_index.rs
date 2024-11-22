#[derive(Debug)]
pub enum PalindromeIndexError {}

impl std::fmt::Display for PalindromeIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unable to find palindrome index!")
    }
}

impl std::error::Error for PalindromeIndexError {}

pub fn palindrome_index(input: &str) -> Result<i32, PalindromeIndexError> {
    if is_palindrome(input) {
        return Ok(-1);
    }

    for i in 0..input.len() {
        let s = input.chars().enumerate().filter(|(j, _)| *j != i).map(|(_, c)| c).collect::<String>();

        if is_palindrome(&s) {
            return Ok(i as i32);
        }
    }

    Ok(-1)
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}