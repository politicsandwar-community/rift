mod help;
mod main;

use crate::types::Command;

pub fn commands() -> Vec<Command> {
    main::commands()
        .into_iter()
        .chain(help::commands())
        .collect()
}
