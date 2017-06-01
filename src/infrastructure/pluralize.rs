pub fn plural_of<S: Into<String>>(word: S, count: i32) -> String {

    if count == 1 {
        return word.into();
    }
    
    return format!("{}s", word.into());
}
