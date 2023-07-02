use async_trait::async_trait;
use bigdecimal::BigDecimal;
use model_derive::Model;
use rift_lang::Expose;
use time::OffsetDateTime;

#[derive(Clone, Debug, Expose, Model)]
#[table = "tradeprices"]
#[cache_name = "tradeprice"]
#[subscriptions = "Tradeprice"]
#[has_pnwkit]
pub struct Tradeprice {
    #[expose]
    pub id: i32,
    #[expose]
    pub date: OffsetDateTime,
    #[expose]
    pub coal: BigDecimal,
    #[expose]
    pub oil: BigDecimal,
    #[expose]
    pub uranium: BigDecimal,
    #[expose]
    pub iron: BigDecimal,
    #[expose]
    pub bauxite: BigDecimal,
    #[expose]
    pub lead: BigDecimal,
    #[expose]
    pub gasoline: BigDecimal,
    #[expose]
    pub munitions: BigDecimal,
    #[expose]
    pub steel: BigDecimal,
    #[expose]
    pub aluminum: BigDecimal,
    #[expose]
    pub food: BigDecimal,
    #[expose]
    pub credits: BigDecimal,
}
