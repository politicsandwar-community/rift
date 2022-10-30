use poise::serenity_prelude::CreateEmbedAuthor;

pub fn embed_author(
    name: String,
    icon_url: String,
) -> Box<dyn FnOnce(&mut CreateEmbedAuthor) -> &mut CreateEmbedAuthor> {
    Box::new(move |a: &mut CreateEmbedAuthor| a.name(name).icon_url(icon_url))
}
