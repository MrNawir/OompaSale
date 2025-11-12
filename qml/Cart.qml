import QtQuick
import QtQuick.Controls

GroupBox {
    title: "Shopping Cart"
    width: parent.width
    height: 200

    ListView {
        anchors.fill: parent
        id: cartView
        model: ListModel {
            // Cart items will be populated here
        }
        delegate: Row {
            spacing: 10
            Text { text: model.name }
            Text { text: "x" + model.quantity }
            Text { text: "$" + (model.price * model.quantity).toFixed(2) }
            Button {
                text: "-"
                onClicked: {
                    // TODO: Decrease quantity
                }
            }
            Button {
                text: "+"
                onClicked: {
                    // TODO: Increase quantity
                }
            }
            Button {
                text: "Remove"
                onClicked: {
                    // TODO: Remove from cart
                }
            }
        }
    }

    Text {
        anchors.bottom: parent.bottom
        anchors.right: parent.right
        id: cartTotal
        text: "Total: $0.00"
        font.bold: true
    }
}
