use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sales")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    pub customer_id: Option<Uuid>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub total: BigDecimal,
    pub payment_method: Option<String>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::customer::Entity",
        from = "Column::CustomerId",
        to = "super::customer::Column::Id"
    )]
    Customer,
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CartItem {
    pub item_id: Uuid,
    pub name: String,
    pub price: BigDecimal,
    pub quantity: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cart {
    pub items: Vec<CartItem>,
    pub total: BigDecimal,
}
