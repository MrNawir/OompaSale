import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    ColumnLayout {
        anchors.fill: parent
        spacing: 10

        // Customer selection
        RowLayout {
            Text { text: "Customer:" }
            ComboBox {
                id: customerCombo
                model: ListModel {
                    ListElement { text: "Walk-in Customer"; value: "" }
                    // TODO: Populate with customer list
                }
                textRole: "text"
                Layout.fillWidth: true
            }
            Button {
                text: "New Customer"
                onClicked: {
                    // TODO: Open customer form
                }
            }
        }

        // Product list
        ListView {
            id: productList
            Layout.fillWidth: true
            Layout.fillHeight: true
            model: ListModel {
                ListElement { name: "Sample Product"; price: 15.99 }
                // TODO: Connect to Rust model
            }
            delegate: Item {
                width: parent.width
                height: 50
                Row {
                    Text { text: name }
                    Text { text: "$" + price }
                    Button {
                        text: "Add to Cart"
                        onClicked: {
                            // TODO: Add to cart
                        }
                    }
                }
            }
        }

        // Cart summary
        GroupBox {
            title: "Cart"
            Layout.fillWidth: true
            height: 200

            ListView {
                anchors.fill: parent
                id: cartList
                model: ListModel {
                    // TODO: Cart items
                }
                delegate: Text { text: model.name + " x" + model.quantity + " $" + model.price }
            }
        }

        // Checkout
        RowLayout {
            Button {
                text: "Checkout"
                onClicked: {
                    // TODO: Process payment
                }
            }
            Text {
                id: totalText
                text: "Total: $0.00"
            }
        }
    }
}
