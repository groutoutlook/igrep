use igrep::{
    ig::{file_entry::FileEntry, grep_match::GrepMatch},
    ui::{
        context_viewer::{ContextViewer, ContextViewerPosition},
        result_list::ResultList,
        theme::dark::Dark,
    },
};
use ratatui::{backend::TestBackend, buffer::Buffer, style::Color, Frame, Terminal};

const SAMPLE_DATA: &str = include_str!("data/sample.ansi");
const SAMPLE_PATH: &str = "tests/data/sample.ansi";

fn text_background(buffer: &Buffer, needle: &str) -> Option<Color> {
    for y in 0..buffer.area.height {
        let mut row = String::with_capacity(buffer.area.width as usize);
        for x in 0..buffer.area.width {
            row.push_str(buffer.get(x, y).symbol());
        }

        if let Some(index) = row.find(needle) {
            return buffer.get(index as u16, y).style().bg;
        }
    }

    None
}

fn draw_buffer(draw: impl FnOnce(&mut Frame)) -> Buffer {
    let backend = TestBackend::new(220, 6);
    let mut terminal = Terminal::new(backend).expect("failed to create test terminal");
    terminal.draw(draw).expect("failed to draw test UI");
    terminal.backend().buffer().clone()
}

#[test]
fn preserves_ansi_backgrounds_for_sample_data_in_results_and_preview() {
    let (line_index, line_text) = SAMPLE_DATA
        .lines()
        .enumerate()
        .find(|(_, line)| line.contains("Red BG"))
        .expect("sample data does not contain Red BG");
    let start = line_text.find("Red BG").expect("missing Red BG text");
    let end = start + "Red BG".len();

    let mut result_list = ResultList::new(true);
    result_list.add_entry(FileEntry::new(
        SAMPLE_PATH.into(),
        vec![GrepMatch::new(
            (line_index + 1) as u64,
            line_text.to_owned(),
            vec![(start, end)],
        )],
    ));

    let theme = Dark;

    let result_buffer = draw_buffer(|frame| result_list.draw(frame, frame.size(), &theme));

    assert_eq!(
        text_background(&result_buffer, "Red BG"),
        Some(Color::Indexed(52))
    );
    assert_eq!(
        text_background(&result_buffer, "Green BG"),
        Some(Color::Green)
    );

    let mut context_viewer = ContextViewer::new(ContextViewerPosition::Horizontal, true);
    context_viewer.update_if_needed(SAMPLE_PATH, &theme);

    let preview_buffer =
        draw_buffer(|frame| context_viewer.draw(frame, frame.size(), &result_list, &theme));

    assert_eq!(
        text_background(&preview_buffer, "Red BG"),
        Some(Color::Indexed(52))
    );
    assert_eq!(
        text_background(&preview_buffer, "Blue BG"),
        Some(Color::Blue)
    );
}
