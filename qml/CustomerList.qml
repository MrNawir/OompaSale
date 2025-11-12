import QtQuick
import QtQuick.Controls

Item {
    ListView {
        anchors.fill: parent
        model: ListModel {
            ListElement { name: "John Doe"; email: "john@example.com"; phone: "555-0123" }
            // TODO: Connect to Rust model
        }
        delegate: Item {
            width: parent.width
            height: 60

            Column {
                Text { text: name; font.bold: true }
                Text { text: email }
                Text { text: phone }
            }

            MouseArea {
                anchors.fill: parent
                onClicked: {
                    // TODO: Open customer details
                }
            }
        }
    }

    Button {
        text: "Add Customer"
        anchors.bottom: parent.bottom
        anchors.right: parent.right
        onClicked: {
            // TODO: Open add customer dialog
        }
    }
}
