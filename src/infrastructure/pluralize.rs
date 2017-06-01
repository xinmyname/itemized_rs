pub fn plural_of(word: &str, count: i32) -> String {

    if count == 1 {
        return format!("{}s", word);
    }
    return "items".to_string();
}
