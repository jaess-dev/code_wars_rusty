use itertools::Itertools;

/// https://www.codewars.com/kata/5264d2b162488dc400000001
///
/// Write a function that takes in a string of one or more words,
/// and returns the same string, but with all words that have five
/// or more letters reversed (Just like the name of this Kata).
/// Strings passed in will consist of only letters and spaces.
/// Spaces will be included only when more than one word is present.
///
/// Examples:
///
/// ```
/// "Hey fellow warriors"  --> "Hey wollef sroirraw"
/// "This is a test        --> "This is a test"
/// "This is another test" --> "This is rehtona test"
/// ```
///
fn spin_words(words: &str) -> String {
    // for word in words
    // if word.len >= 5
    // then
    // word.revers()
    // else
    // word
    // join
    words.split(" ")
        .map(|word| {
            if word.len() >= 5 {
                word.chars().rev().join("")
            } else {
                word.to_string()
            }
        })
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(
            spin_words("You are almost to the last test"),
            "You are tsomla to the last test"
        );
        assert_eq!(
            spin_words("Just kidding there is still one more"),
            "Just gniddik ereht is llits one more"
        );
        assert_eq!(
            spin_words("Seriously this is the last one"),
            "ylsuoireS this is the last one"
        );
    }
}
