use std::sync::Arc;

#[cxx_qt::bridge(cxx_file_stem = "app")]
pub mod qobject {
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qml_singleton]
        #[qproperty(i32, counter)]
        type App = super::AppRs;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        fn no_op(self: Pin<&mut App>);
    }
}

use cxx_qt::CxxQtType;

// Implement the cxx_qt::Constructor<()> trait
pub struct AppRs {
    async_runtime: Arc<tokio::runtime::Runtime>,
    counter: i32,
}

impl Default for AppRs {
    fn default() -> Self {
        Self {
            async_runtime: Arc::new(tokio::runtime::Runtime::new().unwrap()),
            counter: 0,
        }
    }
}

impl qobject::App {
    pub fn no_op(self: core::pin::Pin<&mut Self>) {
        let _ = &self.rust_mut().async_runtime.clone().spawn(async {});
    }
}
