use bigdecimal::BigDecimal;
use crate::models::item::{self, Entity as Item};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
use uuid::Uuid;

impl InventoryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_all_items(&self) -> Result<Vec<item::Model>, Box<dyn std::error::Error>> {
        let items = Item::find().all(&self.db).await?;
        Ok(items)
    }

    pub async fn get_item_by_id(&self, id: Uuid) -> Result<Option<item::Model>, Box<dyn std::error::Error>> {
        let item = Item::find_by_id(id).one(&self.db).await?;
        Ok(item)
    }

    pub async fn create_item(&self, name: String, description: Option<String>, price: f64, quantity: i32, category: Option<String>) -> Result<item::Model, Box<dyn std::error::Error>> {
        let new_item = item::ActiveModel {
            name: sea_orm::ActiveValue::Set(name),
            description: sea_orm::ActiveValue::Set(description),
            price: sea_orm::ActiveValue::Set(BigDecimal::from(price)),
            quantity: sea_orm::ActiveValue::Set(quantity),
            category: sea_orm::ActiveValue::Set(category),
            ..Default::default()
        };
        let item = new_item.insert(&self.db).await?;
        Ok(item)
    }

    pub async fn update_item(&self, id: Uuid, name: Option<String>, description: Option<String>, price: Option<f64>, quantity: Option<i32>, category: Option<String>) -> Result<item::Model, Box<dyn std::error::Error>> {
        let mut item: item::ActiveModel = Item::find_by_id(id).one(&self.db).await?
            .ok_or("Item not found")?.into();

        if let Some(name) = name {
            item.name = sea_orm::ActiveValue::Set(name);
        }
        if let Some(description) = description {
            item.description = sea_orm::ActiveValue::Set(description);
        }
        if let Some(price) = price {
            item.price = sea_orm::ActiveValue::Set(BigDecimal::from(price));
        }
        if let Some(quantity) = quantity {
            item.quantity = sea_orm::ActiveValue::Set(quantity);
        }
        if let Some(category) = category {
            item.category = sea_orm::ActiveValue::Set(category);
        }

        let updated_item = item.update(&self.db).await?;
        Ok(updated_item)
    }

    pub async fn delete_item(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        Item::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    pub async fn search_items(&self, query: &str) -> Result<Vec<item::Model>, Box<dyn std::error::Error>> {
        let items = Item::find()
            .filter(item::Column::Name.contains(query))
            .all(&self.db).await?;
        Ok(items)
    }
}
