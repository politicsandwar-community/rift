mod help;
mod main;
mod odds;
mod revenue;
mod toot;
mod who;

use crate::types::Command;

pub fn commands() -> Vec<Command> {
    main::commands()
        .into_iter()
        .chain(help::commands())
        .chain(revenue::commands())
        .chain(who::commands())
        .chain(toot::commands())
        .chain(odds::commands())
        .collect()
}
