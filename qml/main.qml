import QtQuick 2.15
import QtQuick.Controls 2.15

ApplicationWindow {
    visible: true
    width: 1024
    height: 768
    title: "OompaSale POS"

    menuBar: MenuBar {
        Menu {
            title: "File"
            MenuItem {
                text: "Settings"
                onTriggered: settingsDialog.open()
            }
            MenuItem {
                text: "Exit"
                onTriggered: Qt.quit()
            }
        }
        Menu {
            title: "Help"
            MenuItem {
                text: "Help"
                onTriggered: helpDialog.open()
            }
            MenuItem {
                text: "About"
                onTriggered: aboutDialog.open()
            }
        }
    }

    TabBar {
        id: tabBar
        width: parent.width

        TabButton {
            text: "Inventory"
        }
        TabButton {
            text: "Sales"
        }
        TabButton {
            text: "Customers"
        }
        TabButton {
            text: "Reports"
        }
    }

    StackLayout {
        anchors.top: tabBar.bottom
        anchors.bottom: parent.bottom
        width: parent.width
        currentIndex: tabBar.currentIndex

        Loader {
            source: "InventoryList.qml"
        }

        Loader {
            source: "SalesScreen.qml"
        }

        Loader {
            source: "CustomerList.qml"
        }

        Loader {
            source: "ReportsDashboard.qml"
        }
    }

    Dialog {
        id: settingsDialog
        title: "Settings"
        width: 400
        height: 300

        Loader {
            source: "Settings.qml"
            anchors.fill: parent
        }
    }

    Dialog {
        id: helpDialog
        title: "Help"
        width: 600
        height: 400

        Loader {
            source: "Help.qml"
            anchors.fill: parent
        }
    }

    Dialog {
        id: aboutDialog
        title: "About OompaSale"
        standardButtons: Dialog.Ok

        Text {
            text: "OompaSale POS\nVersion 0.1.0\nBuilt with Qt 6 and Rust"
        }
    }
}
