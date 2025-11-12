import QtQuick
import QtQuick.Controls

Dialog {
    id: errorDialog
    title: "Error"
    standardButtons: Dialog.Ok

    property string errorMessage: "An error occurred"

    Text {
        text: errorMessage
        wrapMode: Text.Wrap
    }
}
