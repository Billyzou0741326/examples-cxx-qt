use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature (default).
        // - Qt Qml is linked by enabling the qt_qml Cargo feature (default).
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        .qt_module("Quick")
        // Generate C++ from the `#[cxx_qt::bridge]` module
        // At least one qobject must be defined
        .qml_module(
            "com.example.voltage_encryptor",
            1,
            0,
            &["src/app.rs"],
        )
        // Generate C++ code from the .qrc file with the rcc tool
        // https://doc.qt.io/qt-6/resources.html
        .cc_builder(|cc| {
            cc.include("../cpp");
        })
        .build();
}
