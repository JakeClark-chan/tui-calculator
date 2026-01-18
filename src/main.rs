use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{error::Error, io};
use tui_calculator::Operation;

// Nhập logic máy tính từ file lib.rs
use tui_calculator::Calculator;

const CALC_WIDTH: u16 = 30;
const CALC_HEIGHT: u16 = 17;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Khởi tạo terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Tạo đối tượng máy tính
    let mut calc = Calculator::new();
    let res = run_app(&mut terminal, &mut calc);

    // 3. Dọn dẹp terminal khi thoát (Cực kỳ quan trọng)
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

// Hàm hỗ trợ căn giữa một vùng có kích thước cố định
fn centered_rect(width: u16, height: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let padding_vertical = r.height.saturating_sub(height) / 2;
    let padding_horizontal = r.width.saturating_sub(width) / 2;
    ratatui::layout::Rect {
        x: r.x + padding_horizontal,
        y: r.y + padding_vertical,
        width: width.min(r.width),
        height: height.min(r.height),
    }
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    calc: &mut Calculator,
) -> Result<(), Box<dyn Error>>
where
    B::Error: 'static,
{
    loop {
        // Vẽ giao diện
        terminal.draw(|f| {
            // 1. Tạo một vùng ở giữa màn hình (Ví dụ rộng 30, cao 15)
            let area = centered_rect(CALC_WIDTH, CALC_HEIGHT, f.area());
            // 2. Vẽ một cái khối (Block) có viền xung quanh vùng đó
            let outer_block = ratatui::widgets::Block::default()
                .title(" Rust Calculator ")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded);

            f.render_widget(outer_block, area);
            // 3. Chia vùng area thành 2 phần: Màn hình hiển thị (3 dòng) và Bàn phím (còn lại)
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .margin(1) // Cách viền 1 ô cho đẹp
                .constraints([
                    ratatui::layout::Constraint::Length(3), // Màn hình cao 3 dòng
                    ratatui::layout::Constraint::Min(0),    // Bàn phím chiếm phần còn lại
                ])
                .split(area);

            // // Tạm thời vẽ một hộp chữ nhật vào phần Màn hình để kiểm tra
            // let display_block = ratatui::widgets::Block::default()
            //     .borders(ratatui::widgets::Borders::ALL)
            //     .title(" [ Display ] ");

            // Xác định tiêu đề dựa trên phép tính đang chờ
            let title = match calc.get_operation_status() {
                Some(Operation::Add) => " [ Adding... ] ",
                Some(Operation::Subtract) => " [ Subtracting... ] ",
                Some(Operation::Multiply) => " [ Multiplying... ] ",
                Some(Operation::Divide) => " [ Dividing... ] ",
                None => " [ Result ] ",
            };

            // Lấy con số hiện tại từ máy tính
            let current_display = calc.display();
            let display_paragraph = ratatui::widgets::Paragraph::new(current_display)
                .block(
                    ratatui::widgets::Block::default()
                        .borders(ratatui::widgets::Borders::ALL)
                        .title(title), // <--- Dùng biến title ở đây
                )
                .alignment(ratatui::layout::Alignment::Right);

            // 4. Định nghĩa các nút bấm
            let keys = [
                ["7", "8", "9", "/"],
                ["4", "5", "6", "*"],
                ["1", "2", "3", "-"],
                ["C", "0", "=", "+"],
            ];
            // 5. Chia bàn phím thành các hàng
            let rows = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Percentage(25),
                    ratatui::layout::Constraint::Percentage(25),
                    ratatui::layout::Constraint::Percentage(25),
                    ratatui::layout::Constraint::Percentage(25),
                ])
                .split(chunks[1]);
            // 6. Vẽ từng nút
            for (i, row_rect) in rows.iter().enumerate() {
                let cols = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints([
                        ratatui::layout::Constraint::Percentage(25),
                        ratatui::layout::Constraint::Percentage(25),
                        ratatui::layout::Constraint::Percentage(25),
                        ratatui::layout::Constraint::Percentage(25),
                    ])
                    .split(*row_rect);
                for (j, col_rect) in cols.iter().enumerate() {
                    let key_text = keys[i][j];
                    let button = ratatui::widgets::Paragraph::new(key_text)
                        .block(
                            ratatui::widgets::Block::default()
                                .borders(ratatui::widgets::Borders::ALL),
                        )
                        .alignment(ratatui::layout::Alignment::Center);
                    f.render_widget(button, *col_rect);
                }
            }

            f.render_widget(display_paragraph, chunks[0]);

            // Render Help Text
            let help_text = "q: Quit | c: Clear | 0-9: Input | +-*/=: Calc";
            let help_area = ratatui::layout::Rect {
                x: area.x,
                y: area.y + area.height + 1,
                width: area.width,
                height: 2,
            };
            let help_paragraph = ratatui::widgets::Paragraph::new(help_text)
                .alignment(ratatui::layout::Alignment::Center)
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray))
                .wrap(ratatui::widgets::Wrap { trim: true });
            f.render_widget(help_paragraph, help_area);
        })?;

        // Xử lý sự kiện (Phím và Chuột)
        match event::read()? {
            // Thoát bằng phím 'q'
            Event::Key(key) if key.code == KeyCode::Char('q') => return Ok(()),

            // Xử lý click chuột
            Event::Mouse(mouse)
                if mouse.kind == event::MouseEventKind::Down(event::MouseButton::Left) =>
            {
                // Tọa độ click: mouse.column (x) và mouse.row (y)
                // Chúng ta sẽ cần tính toán xem (x, y) này thuộc về nút nào.

                // Gợi ý logic:
                // 1. Tính toán lại layout (area, chunks, rows, cols) y hệt như lúc vẽ.
                // 2. Dùng hàm `.contains(point)` của Rect để kiểm tra.
                handle_mouse_click(mouse.column, mouse.row, calc, terminal.size()?.into());
            }
            // Giữ nguyên phần xử lý KeyCode hiện tại của bạn ở đây...
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    // Nếu là phím ký tự từ '0' đến '9'
                    KeyCode::Char(c) if c.is_digit(10) => {
                        // Chuyền số vào máy tính (c.to_digit(10) trả về Option<u32>)
                        if let Some(digit) = c.to_digit(10) {
                            calc.input_digit(digit as u8);
                        }
                    }
                    // Nếu là phím +, -, *, /
                    KeyCode::Char('+') => {
                        calc.set_operation(Operation::Add);
                    }
                    KeyCode::Char('-') => {
                        calc.set_operation(Operation::Subtract);
                    }
                    KeyCode::Char('*') => {
                        calc.set_operation(Operation::Multiply);
                    }
                    KeyCode::Char('/') => {
                        calc.set_operation(Operation::Divide);
                    }
                    KeyCode::Char('=') | KeyCode::Enter => {
                        calc.calculate();
                    }
                    KeyCode::Char('c') => {
                        calc.clear();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn handle_mouse_click(x: u16, y: u16, calc: &mut Calculator, screen_area: ratatui::layout::Rect) {
    let area = centered_rect(CALC_WIDTH, CALC_HEIGHT, screen_area);
    // ... thực hiện split area thành chunks, rồi rows, rồi cols ...
    // Nếu col_rect.contains(ratatui::layout::Position::new(x, y)) {
    //     Lấy nhãn nút đó và kích hoạt hàm tương ứng của calc
    // }

    // Divide into 2 parts
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .margin(1)
        .constraints([
            ratatui::layout::Constraint::Length(3),
            ratatui::layout::Constraint::Min(0),
        ])
        .split(area);

    // Bàn phím nằm trên chunks[1]
    let keys = [
        ["7", "8", "9", "/"],
        ["4", "5", "6", "*"],
        ["1", "2", "3", "-"],
        ["C", "0", "=", "+"],
    ];
    let rows = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([ratatui::layout::Constraint::Percentage(25); 4])
        .split(chunks[1]);
    // B. Duyệt qua từng ô để "Hit-test"
    for (i, row_rect) in rows.iter().enumerate() {
        let cols = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([ratatui::layout::Constraint::Percentage(25); 4])
            .split(*row_rect);
        for (j, col_rect) in cols.iter().enumerate() {
            // Kiểm tra xem (x, y) có nằm trong ô này không
            if x >= col_rect.left()
                && x < col_rect.right()
                && y >= col_rect.top()
                && y < col_rect.bottom()
            {
                let key = keys[i][j];
                // C. Kích hoạt logic dựa trên nhãn nút
                match key {
                    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                        if let Ok(digit) = key.parse::<u8>() {
                            calc.input_digit(digit);
                        }
                    }
                    "+" => calc.set_operation(Operation::Add),
                    "-" => calc.set_operation(Operation::Subtract),
                    "*" => calc.set_operation(Operation::Multiply),
                    "/" => calc.set_operation(Operation::Divide),
                    "=" => calc.calculate(),
                    "C" => calc.clear(),
                    _ => {}
                }
                return; // Đã tìm thấy thì thoát luôn
            }
        }
    }
}
