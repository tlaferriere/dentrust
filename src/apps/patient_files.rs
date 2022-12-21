use crate::models::Id;
use tokio::sync::watch;

pub fn app_channels() -> (ChannelsBackend, ChannelsFrontend) {
    let (patient_names_backend, patient_names_frontend) = watch::channel(vec![]);
    (
        ChannelsBackend {
            patient_names: patient_names_backend,
        },
        ChannelsFrontend {
            patient_names: patient_names_frontend,
        },
    )
}

pub struct ChannelsBackend {
    patient_names: watch::Sender<Vec<String>>,
}

pub struct ChannelsFrontend {
    patient_names: watch::Receiver<Vec<String>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
struct PatientName {
    id: usize,
    name: String,
}

impl Id for PatientName {
    fn id(&self) -> usize {
        self.id
    }
}

impl Default for PatientName {
    fn default() -> Self {
        Self {
            id: 1,
            name: "Johnny Boy!".into(),
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PatientFiles {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    selected_patient: usize,
    patient_names: Vec<PatientName>,
}

impl Default for PatientFiles {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            patient_names: vec![
                PatientName {
                    id: 1,
                    name: "Jean".into(),
                },
                PatientName {
                    id: 2,
                    name: "Paul".into(),
                },
            ],
            selected_patient: 1,
        }
    }
}

impl PatientFiles {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn sidebar_contents(&mut self, ui: &mut egui::Ui) {
        let mut selected_patient: usize = self.selected_patient.to_owned();

        for name in &self.patient_names {
            if ui
                .selectable_label(name.id() == selected_patient, &name.name)
                .clicked()
            {
                selected_patient = name.id;
            }
        }
        self.selected_patient = selected_patient;
    }
}

impl eframe::App for PatientFiles {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            // ui.heading("Side Panel");

            ui.vertical(|ui| self.sidebar_contents(ui));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
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
}
