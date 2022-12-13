use crate::traits::ToEmbed;

macro_rules! not_found {
    ($($variant:ident, $name:literal, $infer:ident)+) => {
        #[derive(Debug)]
        pub enum NotFoundError {
            $($variant(Option<String>)),*
        }

        impl ToEmbed for NotFoundError {
            fn to_embed<'a>(&'a self, ctx: &'a $crate::types::Context<'a>) -> Box<dyn Fn(&mut poise::serenity_prelude::CreateEmbed) -> &mut poise::serenity_prelude::CreateEmbed + '_> {
                Box::new(match self {
                    $(Self::$variant(s) => {
                        $crate::embeds::errors::not_found_error(ctx, $name, s, $infer)
                    }),*
                })
            }
        }

        impl std::fmt::Display for NotFoundError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant(_) => write!(f, "{} not found", $name)
                    ),*
                }
            }
        }
    };
}

not_found!(
    Nation, "nation", true
    Alliance, "alliance", true
    TargetRater, "target rater", false
    TargetConfig, "target config", false
);

impl From<NotFoundError> for Box<dyn ToEmbed> {
    fn from(e: NotFoundError) -> Self {
        Box::new(e)
    }
}
