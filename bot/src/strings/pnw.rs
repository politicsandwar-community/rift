use url::Url;

pub fn link(text: String, link: String) -> String {
    format!("[{}]({})", text, Url::parse(&link).expect("somtinbroke"))
}
