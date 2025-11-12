use crate::models::sale::Cart;
use bigdecimal::BigDecimal;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Receipt {
    pub sale_id: String,
    pub items: Vec<ReceiptItem>,
    pub subtotal: BigDecimal,
    pub tax: BigDecimal,
    pub total: BigDecimal,
    pub payment_method: String,
    pub timestamp: String,
}

#[derive(Clone, Debug)]
pub struct ReceiptItem {
    pub name: String,
    pub quantity: i32,
    pub price: BigDecimal,
    pub total: BigDecimal,
}

impl Receipt {
    pub fn from_cart(cart: &Cart, sale_id: String, payment_method: String) -> Self {
        let subtotal = cart.total.clone();
        let tax_rate = BigDecimal::from(0.08); // 8% tax
        let tax = &subtotal * &tax_rate;
        let total = &subtotal + &tax;

        let items = cart.items.iter().map(|item| ReceiptItem {
            name: item.name.clone(),
            quantity: item.quantity,
            price: item.price.clone(),
            total: &item.price * BigDecimal::from(item.quantity),
        }).collect();

        Self {
            sale_id,
            items,
            subtotal,
            tax,
            total,
            payment_method,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn print(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for Receipt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "RECEIPT")?;
        writeln!(f, "Sale ID: {}", self.sale_id)?;
        writeln!(f, "Time: {}", self.timestamp)?;
        writeln!(f)?;
        writeln!(f, "{:<20} {:>5} {:>8} {:>8}", "Item", "Qty", "Price", "Total")?;
        writeln!(f, "{}", "-".repeat(45))?;
        for item in &self.items {
            writeln!(f, "{:<20} {:>5} ${:>7.2} ${:>7.2}",
                item.name.chars().take(20).collect::<String>(),
                item.quantity,
                item.price,
                item.total)?;
        }
        writeln!(f)?;
        writeln!(f, "{:<35} ${:>7.2}", "Subtotal:", self.subtotal)?;
        writeln!(f, "{:<35} ${:>7.2}", "Tax:", self.tax)?;
        writeln!(f, "{:<35} ${:>7.2}", "Total:", self.total)?;
        writeln!(f, "Payment: {}", self.payment_method)?;
        writeln!(f)?;
        writeln!(f, "Thank you for your business!")?;
        Ok(())
    }
}
