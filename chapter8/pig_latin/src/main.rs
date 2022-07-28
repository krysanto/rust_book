fn main() {
    let latin = String::from("The quick brown fox jumps over the lazy dog");
    let mut pig_latin = String::new();
    let mut in_word = false;
    let mut has_vowel = false;
    let mut first_char = '\0';
    for letter in latin.chars(){
        if letter == ' '{
            in_word = false;
            if has_vowel{
                pig_latin.push_str("-hay ");
            }else{
                pig_latin.push(first_char);
                pig_latin.push_str("ay ");
            }
            continue
        }
        if in_word{
            pig_latin.push(letter);
        }else{
            in_word = true;
            match letter {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    has_vowel = true;
                    pig_latin.push(letter);
                }
                _ => {
                    has_vowel = false;
                    first_char = letter.to_ascii_lowercase();
                }
            }
        }
    }
    if in_word{
        if has_vowel{
            pig_latin.push_str("-hay ");
        }else{
            pig_latin.push(first_char);
            pig_latin.push_str("ay ");
        }
    }

    println!("{}", pig_latin);

}