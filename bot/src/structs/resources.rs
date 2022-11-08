use bigdecimal::BigDecimal;
use pnwkit::Object;

#[derive(Clone, Debug, sqlx::Type)]
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
    pub food: BigDecimal,
}

impl From<Object> for Resources {
    fn from(o: Object) -> Self {
        Self {
            money: o.get("money").unwrap().value().into(),
            coal: o.get("coal").unwrap().value().into(),
            oil: o.get("oil").unwrap().value().into(),
            uranium: o.get("uranium").unwrap().value().into(),
            iron: o.get("iron").unwrap().value().into(),
            bauxite: o.get("bauxite").unwrap().value().into(),
            lead: o.get("lead").unwrap().value().into(),
            gasoline: o.get("gasoline").unwrap().value().into(),
            munitions: o.get("munitions").unwrap().value().into(),
            steel: o.get("steel").unwrap().value().into(),
            aluminum: o.get("aluminum").unwrap().value().into(),
            food: o.get("food").unwrap().value().into(),
        }
    }
}
