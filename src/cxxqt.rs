use std::pin::Pin;
use cxx_qt::cxx_qt_lib;

#[cxx_qt::bridge(namespace = "Inventory")]
pub mod inventory_qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt.h");
        type QGuiApplication;
        include!("cxx-qt-lib/include/core/qstring.h");
        type QString;
    }

    #[cxx_qt::qobject(
        base = "QObject",
        qml_uri = "OompaSale.Inventory",
        qml_version = "1.0"
    )]
    #[derive(Default)]
    pub struct InventoryModel {
        #[qproperty]
        items: Vec<String>,
    }

    impl cxx_qt::cxx_qt_lib::QObject<InventoryModel> {
        #[qinvokable]
        pub fn add_item(self: Pin<&mut Self>, name: QString, price: f64, quantity: i32) {
            let item_str = format!("{{\"name\":\"{name}\",\"price\":{price},\"quantity\":{quantity}}}");
            let mut this = self.as_mut();
            this.items_mut().push(item_str);
        }

        #[qinvokable]
        pub fn get_items(self: Pin<&mut Self>) -> QString {
            let items = &self.items;
            let json = format!("[{}]", items.join(","));
            QString::from(&json)
        }
    }
}
