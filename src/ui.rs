use ratatui::{
    style::{Modifier, Style},
    widgets::{Block, Borders, HighlightSpacing, List},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(
        List::new(
            app.stateful_list
                .items
                .iter()
                .enumerate()
                .map(|(_, item)| item.name.clone())
                .collect::<Vec<String>>(),
        )
        .block(Block::default().title("List").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED),
        )
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always),
        frame.size(),
    )
}
