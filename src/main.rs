use rust_bert::pipelines::summarization::SummarizationModel;
use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Summarizator", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

//#[derive(Default)]
struct MyEguiApp {
    summ_model: SummarizationModel,
    input: String,
    output: String,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        
        Self {
            summ_model: SummarizationModel::new(Default::default()).unwrap(),
            input: String::new(),
            output: String::new(),
        }
    }

    fn summarize(&mut self) {
        let output = self.summ_model.summarize(&[self.input.clone()]);
        self.output = output[0].clone();
        println!("{}", self.output);
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Summarization");
           ui.label("Input text: ");
           ui.text_edit_multiline(&mut self.input);
           ui.label("Output text: ");
           ui.text_edit_multiline(&mut self.output);
           if ui.button("Run").clicked() {
                self.summarize();
           }
       });
   }
}