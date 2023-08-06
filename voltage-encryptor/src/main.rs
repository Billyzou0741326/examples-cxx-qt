pub mod cxxqt_object;

use std::process::exit;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

const APP_NAME: &str = "Application Gease";
const ORG_NAME: &str = "Organization LLC";
const ORG_DOMAIN: &str = "example.com";

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/main.qml"));
    }

    configure_app(&mut app, |app| app.set_application_name(&APP_NAME.into()));
    configure_app(&mut app, |app| app.set_organization_name(&ORG_NAME.into()));
    configure_app(&mut app, |app| {
        app.qguiapplication_set_organization_domain(&ORG_DOMAIN.into())
    });
    if let Some(app) = app.as_mut() {
        exit(app.exec());
    }
}

fn configure_app<F>(app: &mut cxx::UniquePtr<QGuiApplication>, mut f: F)
where
    F: FnMut(core::pin::Pin<&mut QGuiApplication>),
{
    if let Some(app) = app.as_mut() {
        f(app);
    }
}
