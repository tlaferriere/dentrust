use crate::apps::{patient_files, Agenda, Billing, Management, PatientFiles};
use eframe::App;
use std::sync::Arc;

use strum::EnumIter;
use strum::EnumMessage;
use strum::IntoEnumIterator;
use tokio::runtime::{Handle, Runtime};

pub fn app_channels() -> (ChannelsBackend, ChannelsFrontend) {
    let (patient_files_back, patient_files_front) = patient_files::app_channels();
    (
        ChannelsBackend {
            patient_files: patient_files_back,
        },
        ChannelsFrontend {
            patient_files: patient_files_front,
        },
    )
}

pub struct ChannelsBackend {
    patient_files: patient_files::ChannelsBackend,
}

pub struct ChannelsFrontend {
    patient_files: patient_files::ChannelsFrontend,
}

#[derive(EnumIter, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
enum TabAnchor {
    PatientFiles,
    Agenda,
    Billing,
    Management,
}

impl TabAnchor {
    pub fn name(&self) -> &'static str {
        match self {
            TabAnchor::PatientFiles => "Dossiers de patients",
            TabAnchor::Agenda => "Agenda",
            TabAnchor::Billing => "Facturation et plans de traitement",
            TabAnchor::Management => "Gestion",
        }
    }
    pub fn anchor(&self) -> &'static str {
        match self {
            TabAnchor::PatientFiles => "patients",
            TabAnchor::Agenda => "agenda",
            TabAnchor::Billing => "billing",
            TabAnchor::Management => "management",
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DentrustApp {
    #[serde(skip)]
    spawner: Handle,

    files: PatientFiles,
    agenda: Agenda,
    billing: Billing,
    management: Management,
    open_tab: TabAnchor,
}

impl Default for DentrustApp {
    fn default() -> Self {
        Self {
            spawner: Handle::current(),
            files: PatientFiles::default(),
            agenda: Agenda::default(),
            billing: Billing::default(),
            management: Management::default(),
            open_tab: TabAnchor::Agenda,
        }
    }
}

pub(crate) const APP_KEY: &str = "Dentrust";

impl DentrustApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, spawner: Handle) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.open_tab {
            TabAnchor::PatientFiles => self.files.update(ctx, frame),
            TabAnchor::Agenda => self.agenda.update(ctx, frame),
            TabAnchor::Billing => self.billing.update(ctx, frame),
            TabAnchor::Management => self.management.update(ctx, frame),
        }
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();
        let mut selected_tab: TabAnchor = self.open_tab.to_owned();

        for tab in TabAnchor::iter() {
            if ui
                .selectable_label(self.open_tab == tab, tab.name())
                .clicked()
            {
                selected_tab = tab.to_owned();
                if frame.is_web() {
                    ui.output().open_url(format!("#{}", tab.anchor()));
                }
            }
        }
        self.open_tab = selected_tab;
    }
}

impl eframe::App for DentrustApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            });
        });

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     // The top panel is often a good place for a menu bar:
        //     egui::menu::bar(ui, |ui| {
        //         ui.menu_button("File", |ui| {
        //             if ui.button("Quit").clicked() {
        //                 frame.close();
        //             }
        //         });
        //     });
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            self.show_selected_app(ctx, frame);
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
