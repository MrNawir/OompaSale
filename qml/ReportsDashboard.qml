import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    ColumnLayout {
        anchors.fill: parent
        spacing: 20

        // Report type selection
        RowLayout {
            ComboBox {
                id: reportType
                model: ["Sales Report", "Inventory Report", "Customer Report"]
            }
            Button {
                text: "Generate Report"
                onClicked: {
                    // TODO: Generate selected report
                }
            }
        }

        // Date range selection
        RowLayout {
            visible: reportType.currentText === "Sales Report"
            Text { text: "From:" }
            TextField {
                id: fromDate
                placeholderText: "YYYY-MM-DD"
            }
            Text { text: "To:" }
            TextField {
                id: toDate
                placeholderText: "YYYY-MM-DD"
            }
        }

        // Report display area
        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true

            TextArea {
                id: reportContent
                readOnly: true
                text: "Select a report type and generate..."
                wrapMode: TextEdit.Wrap
            }
        }

        // Export button
        Button {
            text: "Export Report"
            Layout.alignment: Qt.AlignRight
            onClicked: {
                // TODO: Export functionality
            }
        }
    }
}
