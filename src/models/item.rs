use bigdecimal::BigDecimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Uuid")]
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub price: BigDecimal,
    pub quantity: i32,
    pub category: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
