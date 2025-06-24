use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType},
};
// use super::GitApp;

pub fn ui(frame: &mut Frame) {
    let main_block = Block::bordered()
        .title(
            Line::from(" ReposiTUI (GIT) ")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Blue)),
        )
        .style(Style::default().fg(Color::Green))
        .border_type(BorderType::Rounded);

    frame.render_widget(main_block, frame.area());
}
