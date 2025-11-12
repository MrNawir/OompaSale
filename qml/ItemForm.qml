import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    title: "Item Form"
    standardButtons: Dialog.Ok | Dialog.Cancel

    ColumnLayout {
        TextField {
            id: nameField
            placeholderText: "Item Name"
        }

        TextField {
            id: descriptionField
            placeholderText: "Description"
        }

        TextField {
            id: priceField
            placeholderText: "Price"
            inputMethodHints: Qt.ImhFormattedNumbersOnly
        }

        TextField {
            id: quantityField
            placeholderText: "Quantity"
            inputMethodHints: Qt.ImhDigitsOnly
        }

        TextField {
            id: categoryField
            placeholderText: "Category"
        }
    }

    onAccepted: {
        // TODO: Save item via Rust
        console.log("Saving item:", nameField.text, priceField.text)
    }
}
