//! This is where all the UI components go.
//!
//! This module should only contain non-blocking code, since it runs in the main UI thread, which is a 60 fps loop.
pub mod agenda;
pub mod billing;
pub mod management;
pub mod patient_files;

pub use agenda::Agenda;
pub use billing::Billing;
pub use management::Management;
pub use patient_files::PatientFiles;
