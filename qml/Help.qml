import QtQuick
import QtQuick.Controls

ScrollView {
    width: parent.width
    height: parent.height

    Column {
        spacing: 20
        padding: 20

        Text {
            text: "OompaSale POS Help"
            font.pixelSize: 24
            font.bold: true
        }

        Text {
            text: "Inventory Management"
            font.pixelSize: 18
            font.bold: true
        }

        Text {
            text: "Use the Inventory tab to manage your product catalog. You can add new items, edit existing ones, and search through your inventory."
            wrapMode: Text.Wrap
            width: parent.width - 40
        }

        Text {
            text: "Sales Processing"
            font.pixelSize: 18
            font.bold: true
        }

        Text {
            text: "The Sales tab allows you to process customer transactions. Search for products, add them to the cart, and complete payments."
            wrapMode: Text.Wrap
            width: parent.width - 40
        }

        Text {
            text: "Customer Management"
            font.pixelSize: 18
            font.bold: true
        }

        Text {
            text: "Manage your customer database in the Customers tab. Add new customers and associate them with sales for loyalty tracking."
            wrapMode: Text.Wrap
            width: parent.width - 40
        }

        Text {
            text: "Reports"
            font.pixelSize: 18
            font.bold: true
        }

        Text {
            text: "Generate business reports and analytics from the Reports tab. View sales data, inventory levels, and export reports."
            wrapMode: Text.Wrap
            width: parent.width - 40
        }

        Text {
            text: "For technical support, contact your system administrator."
            font.italic: true
            wrapMode: Text.Wrap
            width: parent.width - 40
        }
    }
}
