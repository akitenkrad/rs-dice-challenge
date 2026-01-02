use clap::Parser;
use crossterm::{cursor, execute, terminal};
use shared::dice::{dice_to_display, first_zoromi_probability, is_all_same, roll_multiple_dice};
use shared::state::{delete_state, load_state, save_state, GameState};
use std::io::{Write, stdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// 定期保存の間隔（試行回数）
const SAVE_INTERVAL: u64 = 10;

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

    /// 前回の状態から再開
    #[arg(long)]
    resume: bool,
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

    // 状態の初期化
    let (mut state, previous_elapsed) = if args.resume {
        match load_state() {
            Ok(Some(saved_state)) => {
                if saved_state.num_dice != args.num_dice {
                    eprintln!(
                        "警告: サイコロ数が異なります（保存: {} / 指定: {}）",
                        saved_state.num_dice, args.num_dice
                    );
                    eprintln!("キャッシュを破棄して新規開始します。");
                    let _ = delete_state();
                    (
                        GameState::new(args.num_dice, args.interval),
                        Duration::ZERO,
                    )
                } else {
                    let elapsed =
                        Duration::new(saved_state.elapsed_secs, saved_state.elapsed_nanos);
                    println!(
                        "📂 前回の状態から再開します（試行回数: {}, 経過時間: {}）",
                        saved_state.trials_completed,
                        format_elapsed(elapsed)
                    );
                    (saved_state, elapsed)
                }
            }
            Ok(None) => {
                eprintln!("警告: 保存された状態がありません。新規開始します。");
                (
                    GameState::new(args.num_dice, args.interval),
                    Duration::ZERO,
                )
            }
            Err(e) => {
                eprintln!("警告: 状態の読み込みに失敗しました: {}", e);
                eprintln!("新規開始します。");
                (
                    GameState::new(args.num_dice, args.interval),
                    Duration::ZERO,
                )
            }
        }
    } else {
        // 新規開始時は既存のキャッシュを削除
        let _ = delete_state();
        (
            GameState::new(args.num_dice, args.interval),
            Duration::ZERO,
        )
    };

    // Ctrl+Cシグナルハンドラの設定
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("シグナルハンドラの設定に失敗しました");

    println!("🎲 サイコロチャレンジ開始！");
    println!("サイコロ数: {} / 間隔: {}秒", args.num_dice, args.interval);
    println!("（Ctrl+C で中断・状態を保存）");
    println!("---");

    let interval = Duration::from_secs_f64(args.interval);
    let start = Instant::now();
    let mut trial = state.trials_completed;
    let mut last_dice = state.last_dice.clone();

    while running.load(Ordering::SeqCst) {
        trial += 1;
        let dice = roll_multiple_dice(args.num_dice);
        let elapsed = start.elapsed() + previous_elapsed;

        display_output(&dice, trial, elapsed, args.num_dice);

        // 定期保存
        if trial % SAVE_INTERVAL == 0 {
            state.update(trial, elapsed.as_secs(), elapsed.subsec_nanos(), dice.clone());
            if let Err(e) = save_state(&state) {
                eprintln!("\n警告: 状態の保存に失敗しました: {}", e);
            }
        }

        last_dice = dice.clone();

        if is_all_same(&dice) {
            println!();
            println!();
            println!("🎉 ゾロ目達成！ {} 回目の試行で成功しました！", trial);
            // 成功時はキャッシュを削除
            let _ = delete_state();
            return;
        }

        thread::sleep(interval);
    }

    // Ctrl+Cによる中断時の処理
    let elapsed = start.elapsed() + previous_elapsed;
    state.update(trial, elapsed.as_secs(), elapsed.subsec_nanos(), last_dice);

    println!();
    println!();
    println!("⏸️  中断しました。状態を保存しています...");

    match save_state(&state) {
        Ok(()) => {
            println!(
                "✅ 状態を保存しました（試行回数: {}, 経過時間: {}）",
                trial,
                format_elapsed(elapsed)
            );
            println!("   --resume オプションで再開できます。");
        }
        Err(e) => {
            eprintln!("❌ 状態の保存に失敗しました: {}", e);
        }
    }
}
