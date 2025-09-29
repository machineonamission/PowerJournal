import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12
import QtQuick.Layouts


Rectangle {
    id: card
    property int padding: 8

    color: "#f0f0f0"
    radius: 8
    width: parent.width - 2 * parent.padding
    height: contentColumn.implicitHeight

    Column {
        id: contentColumn
        width: parent.width
        padding: 5
        spacing: 5
        Label {
            text: "This is an entry. it could contain text"
            wrapMode: Text.WordWrap
            width: parent.width
        }
    }
}