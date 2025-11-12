use crate::models::customer::{self, Entity as Customer};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;

pub struct CustomerService {
    db: DatabaseConnection,
}

impl CustomerService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_all_customers(&self) -> Result<Vec<customer::Model>, Box<dyn std::error::Error>> {
        let customers = Customer::find().all(&self.db).await?;
        Ok(customers)
    }

    pub async fn get_customer_by_id(&self, id: Uuid) -> Result<Option<customer::Model>, Box<dyn std::error::Error>> {
        let customer = Customer::find_by_id(id).one(&self.db).await?;
        Ok(customer)
    }

    pub async fn create_customer(&self, name: String, email: Option<String>, phone: Option<String>, address: Option<String>) -> Result<customer::Model, Box<dyn std::error::Error>> {
        let new_customer = customer::ActiveModel {
            name: sea_orm::ActiveValue::Set(name),
            email: sea_orm::ActiveValue::Set(email),
            phone: sea_orm::ActiveValue::Set(phone),
            address: sea_orm::ActiveValue::Set(address),
            ..Default::default()
        };
        let customer = new_customer.insert(&self.db).await?;
        Ok(customer)
    }

    pub async fn update_customer(&self, id: Uuid, name: Option<String>, email: Option<String>, phone: Option<String>, address: Option<String>) -> Result<customer::Model, Box<dyn std::error::Error>> {
        let mut customer: customer::ActiveModel = Customer::find_by_id(id).one(&self.db).await?
            .ok_or("Customer not found")?.into();

        if let Some(name) = name {
            customer.name = sea_orm::ActiveValue::Set(name);
        }
        if let Some(email) = email {
            customer.email = sea_orm::ActiveValue::Set(email);
        }
        if let Some(phone) = phone {
            customer.phone = sea_orm::ActiveValue::Set(phone);
        }
        if let Some(address) = address {
            customer.address = sea_orm::ActiveValue::Set(address);
        }

        let updated_customer = customer.update(&self.db).await?;
        Ok(updated_customer)
    }

    pub async fn delete_customer(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        Customer::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    pub async fn search_customers(&self, query: &str) -> Result<Vec<customer::Model>, Box<dyn std::error::Error>> {
        let customers = Customer::find()
            .filter(customer::Column::Name.contains(query))
            .all(&self.db).await?;
        Ok(customers)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<customer::Model>, Box<dyn std::error::Error>> {
        let customer = Customer::find()
            .filter(customer::Column::Email.eq(email))
            .one(&self.db).await?;
        Ok(customer)
    }
}
