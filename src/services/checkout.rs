use bigdecimal::BigDecimal;
use crate::models::{item, sale::{self, Cart, CartItem}, customer};
use crate::services::inventory::InventoryService;
use sea_orm::{DatabaseConnection, ActiveModelTrait};
use uuid::Uuid;

pub struct CheckoutService {
    db: DatabaseConnection,
    inventory_service: InventoryService,
}

impl CheckoutService {
    pub fn new(db: DatabaseConnection) -> Self {
        let inventory_service = InventoryService::new(db.clone());
        Self { db, inventory_service }
    }

    pub async fn add_to_cart(&self, cart: &mut Cart, item_id: Uuid, quantity: i32) -> Result<(), Box<dyn std::error::Error>> {
        let item = self.inventory_service.get_item_by_id(item_id).await?
            .ok_or("Item not found")?;

        if item.quantity < quantity {
            return Err("Insufficient stock".into());
        }

        // Check if item already in cart
        if let Some(existing) = cart.items.iter_mut().find(|i| i.item_id == item_id) {
            existing.quantity += quantity;
        } else {
            cart.items.push(CartItem {
                item_id,
                name: item.name.clone(),
                price: item.price.clone(),
                quantity,
            });
        }

        self.recalculate_total(cart);
        Ok(())
    }

    pub async fn remove_from_cart(&self, cart: &mut Cart, item_id: Uuid, quantity: i32) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(item) = cart.items.iter_mut().find(|i| i.item_id == item_id) {
            if item.quantity <= quantity {
                cart.items.retain(|i| i.item_id != item_id);
            } else {
                item.quantity -= quantity;
            }
            self.recalculate_total(cart);
        }
        Ok(())
    }

    pub async fn checkout(&self, cart: &Cart, customer_id: Option<Uuid>, payment_method: String) -> Result<sale::Model, Box<dyn std::error::Error>> {
        // Validate stock
        for cart_item in &cart.items {
            let item = self.inventory_service.get_item_by_id(cart_item.item_id).await?
                .ok_or("Item not found")?;
            if item.quantity < cart_item.quantity {
                return Err("Insufficient stock for checkout".into());
            }
        }

        // Create sale record
        let new_sale = sale::ActiveModel {
            customer_id: sea_orm::ActiveValue::Set(customer_id),
            total: sea_orm::ActiveValue::Set(cart.total.clone()),
            payment_method: sea_orm::ActiveValue::Set(Some(payment_method)),
            ..Default::default()
        };

        let sale = new_sale.insert(&self.db).await?;

        // Update inventory
        for cart_item in &cart.items {
            let mut item: item::ActiveModel = self.inventory_service.get_item_by_id(cart_item.item_id).await?
                .ok_or("Item not found")?.into();
            let current_qty = item.quantity.as_ref();
            item.quantity = sea_orm::ActiveValue::Set(current_qty - cart_item.quantity);
            item.update(&self.db).await?;
        }

        Ok(sale)
    }

    pub fn search_items(&self, query: &str) -> Result<Vec<item::Model>, Box<dyn std::error::Error>> {
        // This would be async in real implementation
        // For now, delegate to inventory service
        // But since it's sync, we'll assume it's handled elsewhere
        Err("Not implemented".into())
    }

    fn recalculate_total(&self, cart: &mut Cart) {
        cart.total = cart.items.iter()
            .map(|item| &item.price * BigDecimal::from(item.quantity))
            .sum();
    }
}
