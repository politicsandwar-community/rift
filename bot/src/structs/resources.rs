use bigdecimal::BigDecimal;

#[derive (Clone)]
pub struct Resources {
    pub money: Option<BigDecimal>,
    pub coal: Option<BigDecimal>,
    pub oil: Option<BigDecimal>,
    pub uranium: Option<BigDecimal>,
    pub iron: Option<BigDecimal>,
    pub bauxite: Option<BigDecimal>,
    pub lead: Option<BigDecimal>,
    pub gasoline: Option<BigDecimal>,
    pub munitions: Option<BigDecimal>,
    pub steel: Option<BigDecimal>,
    pub aluminum: Option<BigDecimal>,
    pub food: Option<BigDecimal>
}
