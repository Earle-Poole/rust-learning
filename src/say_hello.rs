pub fn func(inner_text: Option<String>) -> String {
    let inner_text = inner_text.unwrap_or("World".to_string());

    format!("{}{}", "Hello ", inner_text)
}
