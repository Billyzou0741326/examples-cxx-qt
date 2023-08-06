import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12

Window {
    id: root

    width: 640
    height: 480
    visible: true
    title: qsTr("Voltage Encryptor")

    Item {
        id: appbar

        height: 48
        anchors {
            top: parent.top
            left: parent.left
            right: parent.right
        }

        Rectangle {
            anchors { fill: parent }
            color: "blue"
        }
    }
}
