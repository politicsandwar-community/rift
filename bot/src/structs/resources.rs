use bigdecimal::BigDecimal;

#[derive(sqlx::Type, Clone)]
#[sqlx(type_name = "resources")]
pub struct Resources {
    pub money: BigDecimal,
    pub coal: BigDecimal,
    pub oil: BigDecimal,
    pub uranium: BigDecimal,
    pub iron: BigDecimal,
    pub bauxite: BigDecimal,
    pub lead: BigDecimal,
    pub gasoline: BigDecimal,
    pub munitions: BigDecimal,
    pub steel: BigDecimal,
    pub aluminum: BigDecimal,
    pub food: BigDecimal
}
