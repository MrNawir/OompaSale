use crate::models::{sale, item, customer, report::{Report, ReportItem}};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use chrono::{DateTime, Utc, Duration};
use bigdecimal::BigDecimal;

pub struct AnalyticsService {
    db: DatabaseConnection,
}

impl AnalyticsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn generate_sales_report(&self, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<Report, Box<dyn std::error::Error>> {
        let sales = sale::Entity::find()
            .filter(sale::Column::CreatedAt.gte(start_date))
            .filter(sale::Column::CreatedAt.lt(end_date))
            .all(&self.db)
            .await?;

        let total_sales = sales.len() as f64;
        let total_revenue: BigDecimal = sales.iter()
            .map(|s| &s.total)
            .sum();

        let items = vec![
            ReportItem {
                label: "Total Sales".to_string(),
                value: total_sales,
            },
            ReportItem {
                label: "Total Revenue".to_string(),
                value: total_revenue.to_string().parse().unwrap_or(0.0),
            },
        ];

        Ok(Report {
            period: format!("{} to {}", start_date.date(), end_date.date()),
            data: items,
        })
    }

    pub async fn generate_inventory_report(&self) -> Result<Report, Box<dyn std::error::Error>> {
        let items = item::Entity::find().all(&self.db).await?;

        let total_items = items.len() as f64;
        let low_stock_items = items.iter().filter(|i| i.quantity < 10).count() as f64;
        let total_value: BigDecimal = items.iter()
            .map(|i| &i.price * BigDecimal::from(i.quantity))
            .sum();

        let data = vec![
            ReportItem {
                label: "Total Items".to_string(),
                value: total_items,
            },
            ReportItem {
                label: "Low Stock Items (< 10)".to_string(),
                value: low_stock_items,
            },
            ReportItem {
                label: "Total Inventory Value".to_string(),
                value: total_value.to_string().parse().unwrap_or(0.0),
            },
        ];

        Ok(Report {
            period: "Current".to_string(),
            data,
        })
    }

    pub async fn generate_customer_report(&self) -> Result<Report, Box<dyn std::error::Error>> {
        let customers = customer::Entity::find().all(&self.db).await?;
        let sales = sale::Entity::find().all(&self.db).await?;

        let total_customers = customers.len() as f64;
        let customers_with_sales = sales.iter()
            .filter_map(|s| s.customer_id)
            .collect::<std::collections::HashSet<_>>()
            .len() as f64;

        let data = vec![
            ReportItem {
                label: "Total Customers".to_string(),
                value: total_customers,
            },
            ReportItem {
                label: "Active Customers".to_string(),
                value: customers_with_sales,
            },
        ];

        Ok(Report {
            period: "All Time".to_string(),
            data,
        })
    }

    pub async fn get_daily_sales(&self, days: i32) -> Result<Vec<(String, f64)>, Box<dyn std::error::Error>> {
        let end_date = Utc::now();
        let start_date = end_date - Duration::days(days as i64);

        let mut result = Vec::new();
        let mut current_date = start_date;

        while current_date < end_date {
            let next_date = current_date + Duration::days(1);
            let day_sales = sale::Entity::find()
                .filter(sale::Column::CreatedAt.gte(current_date))
                .filter(sale::Column::CreatedAt.lt(next_date))
                .all(&self.db)
                .await?;

            let day_total: BigDecimal = day_sales.iter().map(|s| &s.total).sum();
            result.push((current_date.date().to_string(), day_total.to_string().parse().unwrap_or(0.0)));
            current_date = next_date;
        }

        Ok(result)
    }
}
