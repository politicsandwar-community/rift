use poise::serenity_prelude::CreateEmbedAuthor;

pub fn embed_author<'a>(
    name: &'a str,
    icon_url: &'a str,
) -> Box<dyn FnOnce(&mut CreateEmbedAuthor) -> &mut CreateEmbedAuthor + 'a> {
    Box::new(move |a: &mut CreateEmbedAuthor| a.name(name).icon_url(icon_url))
}
