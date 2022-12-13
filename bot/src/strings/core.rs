pub const SUPPORT_SERVER: &str = "https://rift.mrvillage.dev/discord";

pub fn user_mention(user_id: &i64) -> String {
    format!("<@{}>", user_id)
}

pub fn expression(expression: &str) -> String {
    format!("```rs\n{}```", expression)
}

pub fn boolean(boolean: bool) -> String {
    if boolean {
        TRUE.to_string()
    } else {
        FALSE.to_string()
    }
}

pub const TRUE: &str = "True";
pub const FALSE: &str = "False";
pub const NONE: &str = "None";
