mod app;

use color_eyre::Result;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    match App::new() {
        Ok(mut app) => {
            let mut terminal = ratatui::init();
            let app_result = app.run(&mut terminal);

            ratatui::restore();

            app_result
        }
        Err(e) => Err(e),
    }
}
