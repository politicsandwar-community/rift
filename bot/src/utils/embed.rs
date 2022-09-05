use poise::serenity_prelude::{CreateEmbedAuthor, EmbedField};

pub fn embed_author(
    name: String,
    icon_url: String,
) -> Box<dyn FnOnce(&mut CreateEmbedAuthor) -> &mut CreateEmbedAuthor> {
    Box::new(move |a: &mut CreateEmbedAuthor| a.name(name).icon_url(icon_url))
}

pub fn embed_field(
    name: String,
    value: String,
) -> Box<dyn FnOnce(&mut EmbedField) -> &mut EmbedField> {
    Box::new(move |a: &mut EmbedField| a.inline(true).name(name).value(value))
}
