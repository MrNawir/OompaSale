import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    title: "Customer Form"
    standardButtons: Dialog.Ok | Dialog.Cancel

    ColumnLayout {
        TextField {
            id: nameField
            placeholderText: "Customer Name"
            Layout.fillWidth: true
        }

        TextField {
            id: emailField
            placeholderText: "Email Address"
            inputMethodHints: Qt.ImhEmailCharactersOnly
            Layout.fillWidth: true
        }

        TextField {
            id: phoneField
            placeholderText: "Phone Number"
            inputMethodHints: Qt.ImhDialableCharactersOnly
            Layout.fillWidth: true
        }

        TextArea {
            id: addressField
            placeholderText: "Address"
            Layout.fillWidth: true
            Layout.minimumHeight: 80
        }
    }

    onAccepted: {
        // TODO: Save customer via Rust
        console.log("Saving customer:", nameField.text, emailField.text)
    }
}
