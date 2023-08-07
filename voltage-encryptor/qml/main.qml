import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Window 2.12
import QtQuick.Layouts 1.12

import com.example.voltage_encryptor 1.0

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

    Label {
        anchors {
            top: appbar.bottom
            left: parent.left
            right: parent.right
        }
        height: 48
        horizontalAlignment: Text.AlignHCenter
        text: App.counter
        wrapMode: Text.Wrap
    }
}
