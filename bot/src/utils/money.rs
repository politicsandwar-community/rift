use bigdecimal::ToPrimitive;

use crate::{structs::Resources, types::Context};

pub fn get_monetary(ctx: &Context<'_>, res: &Resources) -> f64 {
    let total: f64 = res.money.to_f64().unwrap();

    // UnImplimented - Waiting for trade prices
    total
}
