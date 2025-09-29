import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12
import QtQuick.Layouts

// This must match the uri and version
// specified in the qml_module in the build.rs script.
import me.machineonamission.powerjournal 1.0


ApplicationWindow {
    height: 480
    title: qsTr("PowerJournal")
    visible: true
    width: 640
    color: "#ffffff"
    Entries{}
}