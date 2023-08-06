#[cxx_qt::bridge]
pub mod qobject {

    unsafe extern "RustQt" {
        // cxx_qt::bridge requires at least one qobject to be defined - or else build fails
        #[qobject]
        #[qml_element]
        type NilObjectQml = super::NilObject;
    }
}

// All structs are declared under qobject:: namespace
// The qobject must implement the Default trait
#[derive(Default)]
pub struct NilObject {}

// Implement the cxx_qt::Constructor<()> trait
pub struct Application {}
