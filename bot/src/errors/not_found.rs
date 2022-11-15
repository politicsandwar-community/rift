use crate::traits::ToEmbed;

macro_rules! not_found {
    ($($variant:ident, $name:ident, $infer:ident)+) => {
        #[derive(Debug)]
        pub enum NotFoundError {
            $($variant(Option<String>)),*
        }

        impl ToEmbed for NotFoundError {
            fn to_embed<'a>(&'a self, ctx: &'a $crate::types::Context<'a>) -> Box<dyn Fn(&mut poise::serenity_prelude::CreateEmbed) -> &mut poise::serenity_prelude::CreateEmbed + '_> {
                Box::new(match self {
                    $(Self::$variant(s) => {
                        $crate::embeds::not_found_error(ctx, stringify!($name), s, $infer)
                    }),*
                })
            }
        }

        impl std::fmt::Display for NotFoundError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$variant(_) => write!(f, "{} not found", stringify!($name))
                    ),*
                }
            }
        }
    };
}

not_found!(
    Nation, nation, true
    Alliance, alliance, true
);

impl From<NotFoundError> for Box<dyn ToEmbed> {
    fn from(e: NotFoundError) -> Self {
        Box::new(e)
    }
}
