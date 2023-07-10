use crate::{structs::Resources, types::Context};
use bigdecimal::ToPrimitive;
use time::macros::time;
use time::OffsetDateTime;

pub fn get_monetary(ctx: &Context<'_>, res: &Resources) -> f64 {
    let tradeprice_data = ctx
        .data()
        .cache
        .find_many_tradeprices(|data| {
            data.date == OffsetDateTime::now_utc().replace_time(time!(00:00))
        })
        .into_iter()
        .find(|_| true)
        .unwrap();
    println!("{:?}", tradeprice_data);

    let total: f64 = (&res.money
        + (&res.coal * &tradeprice_data.coal)
        + (&res.oil * &tradeprice_data.oil)
        + (&res.uranium * &tradeprice_data.uranium)
        + (&res.iron * &tradeprice_data.iron)
        + (&res.bauxite * &tradeprice_data.bauxite)
        + (&res.lead * &tradeprice_data.lead)
        + (&res.gasoline * &tradeprice_data.gasoline)
        + (&res.munitions * &tradeprice_data.munitions)
        + (&res.steel * &tradeprice_data.steel)
        + (&res.aluminum * &tradeprice_data.aluminum)
        + (&res.food * &tradeprice_data.food))
        .to_f64()
        .unwrap();

    // UnImplimented - Waiting for trade prices
    total
}
