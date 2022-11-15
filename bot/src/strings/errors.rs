#[inline(always)]
pub fn not_found_error(name: &str, value: &Option<String>, infer: bool) -> String {
    if value.is_none() && infer {
        format!("Your Discord account is not linked to a nation so I couldn't infer your {name}.")
    } else if let Some(value) = value {
        format!("No {name} found with argument `{value}`.")
    } else {
        format!("No {name} found.")
    }
}

pub const FATAL_ERROR: &str = const_format::formatcp!("Uh oh! Something went wrong! Please try again later. If you need further assistance or the problem persists please join the support server [here]({}) or context <@258298021266063360>.", crate::strings::core::SUPPORT_SERVER);
pub const GUILD_ONLY_ERROR: &str = "This command can only be used in a server.";
pub const DM_ONLY_ERROR: &str = "This command can only be used in a DM.";
