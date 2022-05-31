use crate::{app::Apps, runner::InputMode};
use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans, Text},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, Paragraph, Sparkline, Tabs,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, apps: &mut Apps) {
    let app = apps.app_map.get_mut(&apps.rules[apps.index]).unwrap();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match apps.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(apps.input.as_ref())
        .style(match apps.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);

    let titles = apps
        .rules
        .iter()
        .map(|t| Spans::from(vec![Span::styled(t, Style::default().fg(Color::Green))]))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Rules"))
        .select(apps.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );
    f.render_widget(tabs, chunks[2]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[3]);

    let lower_left = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let events: Vec<ListItem> = app
        .totals
        .iter()
        .rev()
        .map(|total| {
            ListItem::new(vec![
                Spans::from(vec![
                    Span::styled(total.clone().0, Style::default().fg(Color::Yellow)),
                    Span::raw(" ".repeat(lower_left[0].width as usize - 20)),
                    Span::styled(
                        Apps::format_speed(total.1 as f64, false),
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::ITALIC)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Spans::from("-".repeat(lower_left[0].width as usize - 1)),
            ])
        })
        .collect();
    let events_list = List::new(events)
        .block(Block::default().borders(Borders::ALL).title("Total"))
        .start_corner(Corner::BottomLeft);
    f.render_widget(events_list, lower_left[0]);

    let sparkline = Sparkline::default()
        .block(Block::default().title("Packet").borders(Borders::ALL))
        .data(&app.chart)
        .max(1800)
        .style(Style::default().fg(Color::Red));
    f.render_widget(sparkline, lower_left[1]);

    let x_labels = vec![
        Span::styled(
            format!("{}s", app.window[0]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("{}s", (app.window[0] + app.window[1]) / 2.0)),
        Span::styled(
            format!("{}s", app.window[1]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];
    let datasets = vec![Dataset::default()
        .name(format!("speed: {}", &app.current_speed))
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Green))
        .graph_type(GraphType::Line)
        .data(&app.net_speed)];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Speed",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .labels(x_labels)
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .labels(vec![
                    Span::styled(
                        format!("{:.1}MB/s", app.y_bounds[0]),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(
                        "{:.1}MB/s",
                        (app.y_bounds[0] + app.y_bounds[1]) / 2.0
                    )),
                    Span::styled(
                        format!("{:.1}MB/s", app.y_bounds[1]),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                ])
                .bounds(app.y_bounds),
        )
        .hidden_legend_constraints((Constraint::Percentage(90), Constraint::Percentage(90)));
    f.render_widget(chart, chunks[1]);
}
