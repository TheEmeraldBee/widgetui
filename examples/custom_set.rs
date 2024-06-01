use anyhow::Result;
use ratatui::widgets::Paragraph;
use widgetui::*;

#[derive(State)]
pub struct CoolState {
    pub q_count: i32,
}

impl Default for CoolState {
    fn default() -> Self {
        Self { q_count: 8 }
    }
}

pub fn widget(
    mut frame: ResMut<WidgetFrame>,
    mut events: ResMut<Events>,
    mut state: ResMut<CoolState>,
) -> WidgetResult {
    if events.key(crossterm::event::KeyCode::Char('q')) {
        state.q_count -= 1;
        if state.q_count <= 0 {
            events.register_exit();
            return Ok(());
        }
    }

    let size = frame.size();

    frame.render_widget(
        Paragraph::new(format!("Press `q` {} more times", state.q_count)),
        size,
    );

    Ok(())
}

#[set]
pub fn CoolSet(app: App) -> App {
    app.widgets(widget).states(CoolState::default())
}

fn main() -> Result<()> {
    App::new(100)?.sets(CoolSet).run()
}
