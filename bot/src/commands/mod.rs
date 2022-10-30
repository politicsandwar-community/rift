mod help;
mod main;
mod toot;
mod who;

use crate::types::Command;

pub fn commands() -> Vec<Command> {
    main::commands()
        .into_iter()
        .chain(help::commands())
        .chain(who::commands())
        .chain(toot::commands())
        .collect()
}
