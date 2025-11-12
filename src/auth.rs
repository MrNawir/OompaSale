use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Manager,
    Cashier,
}

impl Role {
    pub fn can_manage_inventory(&self) -> bool {
        matches!(self, Role::Admin | Role::Manager)
    }

    pub fn can_process_sales(&self) -> bool {
        matches!(self, Role::Admin | Role::Manager | Role::Cashier)
    }

    pub fn can_view_reports(&self) -> bool {
        matches!(self, Role::Admin | Role::Manager)
    }
}

pub struct User {
    pub id: String,
    pub username: String,
    pub role: Role,
}

pub fn check_permission(user: &User, action: &str) -> bool {
    match action {
        "manage_inventory" => user.role.can_manage_inventory(),
        "process_sales" => user.role.can_process_sales(),
        "view_reports" => user.role.can_view_reports(),
        _ => false,
    }
}
