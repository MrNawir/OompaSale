import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    ColumnLayout {
        anchors.fill: parent
        spacing: 20

        GroupBox {
            title: "Database Settings"
            Layout.fillWidth: true

            ColumnLayout {
                TextField {
                    placeholderText: "Database URL"
                    Layout.fillWidth: true
                }
            }
        }

        GroupBox {
            title: "UI Settings"
            Layout.fillWidth: true

            ColumnLayout {
                CheckBox {
                    text: "Enable offline mode"
                    checked: true
                }
                CheckBox {
                    text: "Show notifications"
                    checked: true
                }
            }
        }

        GroupBox {
            title: "Security Settings"
            Layout.fillWidth: true

            ColumnLayout {
                TextField {
                    placeholderText: "Admin password"
                    echoMode: TextInput.Password
                    Layout.fillWidth: true
                }
            }
        }

        Button {
            text: "Save Settings"
            Layout.alignment: Qt.AlignRight
            onClicked: {
                // TODO: Save settings
            }
        }
    }
}
