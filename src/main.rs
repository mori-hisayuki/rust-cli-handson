use clap::Parser;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};

const TRAIN: [&str; 10] = [
    "      ====        ________                ___________ ",
    "  _D _|  |_______/        \\__I_I_____===__|_________| ",
    "   |(_)---  |   H\\________/ |   |        =|___ ___| ",
    "   /     |  |   H  |  |     |   |         ||_| |_|| ",
    "  |      |  |   H  |__--------------------| [___] | ",
    "  | ________|___H__/__|_____/[][]~\\_______|       | ",
    "  |/ |   |-----------I_____I [][] []  D   |=======|_ ",
    "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ",
    " |/-=|___|=   O=====O=====O=====O|_____/~\\___/           ",
    "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/          ",
];

const TRAIN_LENGTH: u16 = TRAIN[0].len() as u16;

fn draw_train(potision: u16) {
    let mut stdout = io::stdout();

    for (i, line) in TRAIN.iter().enumerate() {
        let train_x = potision;
        let train_y = (i + 4) as u16;
        execute!(
            stdout,
            cursor::MoveTo(train_x, train_y),
            crossterm::style::Print(line)
        )
        .unwrap();
    }
}

const SMOKE: [&str; 4] = ["           (@)", "      (@@)", "  (@@@)", "(@@@@)"];

fn draw_smoke(position: u16) {
    let mut stdout = io::stdout();
    for (i, smoke) in SMOKE.iter().enumerate() {
        let smoke_x = position + 6;
        let smoke_y = i as u16;
        execute!(
            stdout,
            cursor::MoveTo(smoke_x, smoke_y),
            crossterm::style::Print(smoke)
        )
        .unwrap();
    }
}

/// コマンドライン引数
#[derive(Parser)]
#[command(version)]
struct CommandArgs {
    /// アニメーションの描画ウェイト(ミリ秒)
    #[arg(short, long, default_value_t = 100)]
    wait: u64,
}

fn main() {
    let args: CommandArgs = CommandArgs::parse();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut stdout = io::stdout();
    let terminal_width = crossterm::terminal::size().unwrap().0;

    let max_position = if terminal_width > TRAIN_LENGTH {
        terminal_width - TRAIN_LENGTH
    } else {
        0
    };

    execute!(stdout, Clear(ClearType::All), cursor::Hide).unwrap();
    // Main animation loop
    for pos in (0..=max_position).rev() {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        execute!(stdout, Clear(ClearType::All)).unwrap();
        draw_smoke(pos);
        draw_train(pos);

        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(args.wait)); //★変更
    }

    execute!(stdout, Clear(ClearType::All), cursor::Show).unwrap();
}
