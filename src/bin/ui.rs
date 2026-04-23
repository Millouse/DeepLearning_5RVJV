use DeepLearning::ui::app::run_ui;

#[macroquad::main("Pond UI")]
async fn main() {
    run_ui().await;
}
