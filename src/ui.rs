use crate::{app::Apps, runner::InputMode};
use chrono::Local;
use ratatui::{
    layout::{Alignment, Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Text},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, Paragraph, Sparkline,
        Tabs, Wrap,
    },
    Frame,
};
use ratatui::text::Line;

pub fn draw(f: &mut Frame, apps: &mut Apps) {
    let app = apps.app_map.get_mut(&apps.rules[apps.index]).unwrap();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Max(1),
                Constraint::Max(3),
                Constraint::Max(3),
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match apps.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled(
                    "q",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
                Span::raw(" to exit, "),
                Span::styled(
                    "e",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
                Span::raw(" to start editing, "),
                Span::styled(
                    "dd",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
                Span::raw(" to delete rule."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled(
                    "Esc",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
                Span::raw(" to stop editing, "),
                Span::styled(
                    "Enter",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
                Span::raw(" to record the rule."),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(apps.input.as_str())
        .style(match apps.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);

    let tab_items = apps
        .rules
        .iter()
        .map(|t| Line::from(vec![Span::styled(t, Style::default().fg(Color::Green))]))
        .collect();
    let tabs = Tabs::new(tab_items)
        .block(Block::default().borders(Borders::ALL).title("Rules"))
        .select(apps.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow)
                .bg(Color::Black),
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

    let overview_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(lower_left[0]);

    let overview_text = vec![
        Line::from(vec![
            Span::styled(
                "Interface:  ",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM),
            ),
            Span::styled(
                apps.interface_name.to_string(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Start Time: ",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM),
            ),
            Span::styled(
                app.start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Uptime:     ",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM),
            ),
            Span::styled(
                format!(
                    "{}d {}h {}m {}s",
                    Local::now()
                        .signed_duration_since(app.start_time)
                        .num_days(),
                    Local::now()
                        .signed_duration_since(app.start_time)
                        .num_hours()
                        % 24,
                    Local::now()
                        .signed_duration_since(app.start_time)
                        .num_minutes()
                        % 60,
                    Local::now()
                        .signed_duration_since(app.start_time)
                        .num_seconds()
                        % 60
                ),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
    ];

    let overview = Paragraph::new(overview_text)
        .block(Block::default().borders(Borders::ALL).title("Overview"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(overview, overview_chunk[0]);

    let total_items: Vec<ListItem> = app
        .totals
        .iter()
        .rev()
        .map(|total| {
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(total.clone().0, Style::default().fg(Color::Yellow)),
                    Span::raw(" ".repeat(if lower_left[0].width > 20 {
                        lower_left[0].width as usize - 20
                    } else {
                        1
                    })),
                    Span::styled(
                        Apps::format_speed(total.1 as f64, false),
                        Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::ITALIC)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from("-".repeat(lower_left[0].width as usize - 4)),
            ])
        })
        .collect();
    let total_list = List::new(total_items)
        .block(Block::default().borders(Borders::ALL).title("Total"))
        .start_corner(Corner::BottomLeft);
    f.render_widget(total_list, overview_chunk[1]);

    let packet_chart = Sparkline::default()
        .block(Block::default().title("Packet").borders(Borders::ALL))
        .data(&app.chart)
        .max(1800)
        .style(Style::default().fg(Color::Red));
    f.render_widget(packet_chart, lower_left[1]);

    let x_labels = vec![
        Span::styled(
            format!("{}s", app.window[0]),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{}s", (app.window[0] + app.window[1]) / 2.0),
            Style::default().add_modifier(Modifier::BOLD),
        ),
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

    let speed_chart = Chart::new(datasets)
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
                    Span::styled(
                        format!("{:.1}MB/s", (app.y_bounds[0] + app.y_bounds[1]) / 2.0),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{:.1}MB/s", app.y_bounds[1]),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                ])
                .bounds(app.y_bounds),
        )
        .hidden_legend_constraints((Constraint::Percentage(90), Constraint::Percentage(90)));
    f.render_widget(speed_chart, chunks[1]);
}
