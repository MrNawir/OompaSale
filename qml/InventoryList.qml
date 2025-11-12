import QtQuick
import QtQuick.Controls

Item {
    ListView {
        anchors.fill: parent
        model: ListModel {
            ListElement { name: "Sample Item"; price: 10.99; quantity: 5 }
            // TODO: Connect to Rust model
        }

        delegate: Item {
            width: parent.width
            height: 50

            Row {
                spacing: 10
                Text { text: name }
                Text { text: "$" + price }
                Text { text: "Qty: " + quantity }
            }
        }
    }

    Button {
        text: "Add Item"
        anchors.bottom: parent.bottom
        anchors.right: parent.right
        onClicked: {
            // TODO: Open add item dialog
        }
    }
}
