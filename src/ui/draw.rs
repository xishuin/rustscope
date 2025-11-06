// src/ui/draw.rs
use std::collections::VecDeque;
use crate::app::App;
use crate::data::net::NetInfo;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};


fn draw_net_chart(
    f: &mut Frame,
    area: Rect,
    title: &str,
    download_data: &VecDeque<f64>,
    upload_data: &VecDeque<f64>,
) {
    // 1. 准备下载数据集（绿色）
    let download_points: Vec<(f64, f64)> = download_data
        .iter()
        .enumerate()
        .map(|(i, &rate)| (i as f64, rate))
        .collect();
    let download_dataset = Dataset::default()
        .name("Download")
        .graph_type(GraphType::Line)
        .style(Color::Green)
        .marker(ratatui::symbols::Marker::Braille)
        .data(&download_points);

    // 2. 准备上传数据集（红色）
    let upload_points: Vec<(f64, f64)> = upload_data
        .iter()
        .enumerate()
        .map(|(i, &rate)| (i as f64, rate))
        .collect();
    let upload_dataset = Dataset::default()
        .name("Upload")
        .graph_type(GraphType::Line)
        .style(Color::Red)
        .marker(ratatui::symbols::Marker::Braille)
        .data(&upload_points);

    // 3. 动态计算 Y 轴边界，让图表自适应
    let max_download = download_data.iter().fold(0.0_f64, |a, &b| a.max(b));
    let max_upload = upload_data.iter().fold(0.0_f64, |a, &b| a.max(b));
    let max_rate = max_download.max(max_upload);
    // 如果没有流量，给一个最小值，避免Y轴从0到0
    let y_max = if max_rate > 0.0 { max_rate * 1.1 } else { 10.0 };

    let chart = Chart::new(vec![download_dataset, upload_dataset])
        .block(Block::default().title(title).borders(Borders::ALL))
        .x_axis(
            Axis::default()
                .title("Time (ticks)")
                .style(Color::Gray)
                .bounds([0.0, download_data.len() as f64]),
        )
        .y_axis(
            Axis::default()
                .style(Color::Gray)
                .bounds([0.0, y_max]),
        );

    // --- 关键改动：先渲染图表框架 ---
    f.render_widget(chart, area);

    // --- 关键改动：只有在有数据时才渲染标签 ---
    if let (Some(&latest_down), Some(&latest_up)) = (download_data.back(), upload_data.back()) {
        // 只有当最新速率不为0时才显示标签，避免一直显示 0.0 KB/s
        if latest_down > 0.0 || latest_up > 0.0 {
            let down_text = NetInfo::format_rate(latest_down);
            let up_text = NetInfo::format_rate(latest_up);
            let text = format!("↓ {} | ↑ {}", down_text, up_text);

            let label_style = Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD);

            let text_widget = Paragraph::new(text).style(label_style);
            let text_area = Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: 30,
                height: 1,
            };
            f.render_widget(text_widget, text_area);
        }
    }
}


    // ... draw_chart 函数保持不变 ...
fn draw_chart(
        f: &mut Frame,
        area: Rect,
        title: &str,
        data: &VecDeque<f64>,
        y_label: &str,
        color: Color,
    ) {
        if data.is_empty() {
            return;
        }

        // 1. 准备图表数据
        let data_points: Vec<(f64, f64)> = data
            .iter()
            .enumerate()
            .map(|(i, &usage)| (i as f64, usage))
            .collect();

        // 将 name 设为空，以隐藏默认的图例
        let dataset = Dataset::default()
            .name("") // <--- 关键改动：隐藏默认图例
            .graph_type(GraphType::Line)
            .style(color)
            .marker(ratatui::symbols::Marker::Braille)
            .data(&data_points);

        let chart = Chart::new(vec![dataset])
            .block(Block::default().title(title).borders(Borders::ALL))
            .x_axis(
                Axis::default()
                    .title("Time (ticks)")
                    .style(Color::Gray)
                    .bounds([0.0, data.len() as f64]),
            )
            .y_axis(
                Axis::default()
                    .title("") // <--- 关键改动：隐藏Y轴标题，因为我们会在标签里显示
                    .style(Color::Gray)
                    .bounds([0.0, 100.0]),
            );

        // 2. 首先渲染图表
        f.render_widget(chart, area);

        // 3. 在图表内部渲染自定义的、更大的标签
        if let Some(&latest_value) = data.back() {
            // 将标签文字和当前值组合在一起
            let text = format!("{}: {:.1}%", y_label, latest_value);
            
            // 创建一个带加粗和背景色的样式，让它看起来更大更突出
            let label_style = Style::default()
                .fg(color) // 前景色使用图表的颜色
                .bg(Color::DarkGray) // 深灰色背景，像一个标签
                .add_modifier(Modifier::BOLD); // 加粗

            let text_widget = Paragraph::new(text).style(label_style);

            // 计算文本放置的位置：图表左上角，边框内侧
            let text_area = Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: (y_label.len() + 10) as u16, // 根据标签长度调整宽度
                height: 1,
            };

            // 将文本渲染到计算好的位置
            f.render_widget(text_widget, text_area);
        }
    }
/// 主绘制函数，现在根据 App 的标志动态绘制
pub fn draw(f: &mut Frame, app: &App) {
    // 1. 创建主布局：上(图表区) -> 下(提示区)
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // 图表区域
            Constraint::Length(3), // 提示区
        ])
        .split(f.area());

    let chart_area = vertical_chunks[0];

    // 2. 根据布尔标志，决定要绘制哪些模块
    let mut modules_to_draw = Vec::new();
    if app.show_cpu {
        modules_to_draw.push("cpu");
    }
    if app.show_memory {
        modules_to_draw.push("memory");
    }
    // 预留位置，未来启用
    // if app.show_gpu { modules_to_draw.push("gpu"); }
    if app.show_net { modules_to_draw.push("net"); }

    // 3. 动态生成布局约束
    // 如果没有选择任何模块（理论上不应该发生），则默认显示CPU
    if modules_to_draw.is_empty() {
        modules_to_draw.push("cpu");
    }
    
    let constraints: Vec<Constraint> = modules_to_draw
        .iter()
        .map(|_| Constraint::Percentage(100 / modules_to_draw.len() as u16))
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Horizontal) // 水平排列
        .constraints(constraints)
        .split(chart_area);

    // 4. 按顺序绘制选中的模块
    for (i, module) in modules_to_draw.iter().enumerate() {
        match *module {
            "cpu" => {
                draw_chart(
                    f,
                    chunks[i],
                    "CPU Usage History",
                    &app.system_info.cores.usages,
                    "Usage ",
                    Color::Yellow,
                );
            }
            "memory" => {
                let total_mem_gb = app.system_info.memory.info.get_total_memory_gb();
                let memory_title = format!("Memory Usage (Total: {:.2} GB)", total_mem_gb);
                draw_chart(
                    f,
                    chunks[i],
                    &memory_title,
                    &app.system_info.memory.usages,
                    "Usage ",
                    Color::Cyan,
                );
            }
            // 预留未来模块的绘制逻辑
            // "gpu" => { /* ... 调用 draw_chart ... */ }
            "net" => {
                let title = format!("Network Usage ({})", app.system_info.net.get_interface_name());
                draw_net_chart(
                    f,
                    chunks[i],
                    &title,
                    &app.system_info.net.download_rates,
                    &app.system_info.net.upload_rates,
                );
            }
            _ => {} // 忽略未知模块
        }
    }

    // 5. 渲染底部的提示文字
    let paragraph = Paragraph::new("Press 'q' to quit.")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, vertical_chunks[1]);
}