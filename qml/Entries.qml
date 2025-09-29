import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12
import QtQuick.Layouts

ScrollView {
    id: scroller
    anchors.fill: parent
    clip: true
    Column {
        id: rootColumn
        width: scroller.contentItem.width
        spacing: 10
        padding: 10

        Repeater {
            model: 50
            delegate: Entry {

            }
        }
    }

}