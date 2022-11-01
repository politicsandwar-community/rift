use url::Url;

pub fn link(text: String, link: String) -> String {
    match Url::parse(&link) {
        Ok(val) => format!("[{}]({})", text, val),
        Err(_) => "None".to_owned(),
    }
}
