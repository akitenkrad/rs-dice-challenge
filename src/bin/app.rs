use clap::Parser;
use crossterm::{cursor, execute, terminal};
use shared::dice::{dice_to_display, first_zoromi_probability, is_all_same, roll_multiple_dice};
use std::io::{Write, stdout};
use std::thread;
use std::time::{Duration, Instant};

/// サイコロチャレンジ - ゾロ目が出るまで試行を繰り返すCLIツール
#[derive(Parser, Debug)]
#[command(name = "dice-challenge")]
#[command(about = "複数のサイコロを振って、ゾロ目が出るまで試行を繰り返す")]
struct Args {
    /// サイコロの数
    #[arg(short = 'n', long, default_value_t = 2)]
    num_dice: usize,

    /// 試行を繰り返す間隔（秒）
    #[arg(short = 'i', long, default_value_t = 1.0)]
    interval: f64,
}

/// 経過時間をMM:ss形式でフォーマット
fn format_elapsed(elapsed: Duration) -> String {
    let total_secs = elapsed.as_secs();
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}", mins, secs)
}

/// 画面を上書きして出力を表示
fn display_output(dice: &[u8], trial: u64, elapsed: Duration, num_dice: usize) {
    let is_zoromi = is_all_same(dice);
    let dice_display = dice_to_display(dice, is_zoromi);
    let prob = first_zoromi_probability(num_dice, trial);
    let elapsed_str = format_elapsed(elapsed);

    let mut stdout = stdout();

    // カーソルを行頭に移動し、行をクリア
    execute!(
        stdout,
        cursor::MoveToColumn(0),
        terminal::Clear(terminal::ClearType::CurrentLine)
    )
    .unwrap();

    // 1行目: サイコロの出目
    print!("{}", dice_display);

    // 改行して2行目
    execute!(stdout, cursor::MoveToColumn(0)).unwrap();
    println!();
    execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();

    // 2行目: 統計情報
    print!(
        "number of trials: {:015} / ETA: {} / Prob: {:.8}%",
        trial,
        elapsed_str,
        prob * 100.0
    );

    stdout.flush().unwrap();

    // カーソルを1行上に戻す（次の試行で上書きするため）
    if !is_zoromi {
        execute!(stdout, cursor::MoveUp(1)).unwrap();
    }
}

fn main() {
    let args = Args::parse();

    if args.num_dice < 2 {
        eprintln!("エラー: サイコロの数は2以上を指定してください");
        std::process::exit(1);
    }

    println!("🎲 サイコロチャレンジ開始！");
    println!("サイコロ数: {} / 間隔: {}秒", args.num_dice, args.interval);
    println!("---");

    let interval = Duration::from_secs_f64(args.interval);
    let start = Instant::now();
    let mut trial: u64 = 0;

    loop {
        trial += 1;
        let dice = roll_multiple_dice(args.num_dice);
        let elapsed = start.elapsed();

        display_output(&dice, trial, elapsed, args.num_dice);

        if is_all_same(&dice) {
            println!();
            println!();
            println!("🎉 ゾロ目達成！ {} 回目の試行で成功しました！", trial);
            break;
        }

        thread::sleep(interval);
    }
}
