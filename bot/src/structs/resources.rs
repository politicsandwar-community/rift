use std::ops::Add;

use bigdecimal::BigDecimal;
use pnwkit::Object;
use rift_lang::Expose;

#[derive(Clone, Debug, sqlx::Type, Expose)]
#[sqlx(type_name = "resources")]
pub struct Resources {
    #[expose]
    pub money: BigDecimal,
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

impl Add<Resources> for Resources {
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            money: self.money + rhs.money,
            coal: self.coal + rhs.coal,
            oil: self.oil + rhs.oil,
            uranium: self.uranium + rhs.uranium,
            iron: self.iron + rhs.iron,
            bauxite: self.bauxite + rhs.bauxite,
            lead: self.lead + rhs.lead,
            gasoline: self.gasoline + rhs.gasoline,
            munitions: self.munitions + rhs.munitions,
            steel: self.steel + rhs.steel,
            aluminum: self.aluminum + rhs.aluminum,
            food: self.food + rhs.food,
        }
    }

    type Output = Self;
}

#[macro_export]
macro_rules! resource_field {
    ($resources:ident $($r:ident)*) => {
      [
          $(
              (stringify!($r), format!("{:.2}", $resources.$r), true),
          )*
      ]
    };
  }
