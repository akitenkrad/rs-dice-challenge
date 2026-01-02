use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::errors::{AppError, AppResult};

/// キャッシュファイル名
pub const STATE_FILE: &str = ".dice-challenge-state.json";

/// ゲームの状態を保存する構造体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    /// サイコロの数
    pub num_dice: usize,
    /// 試行間隔（秒）
    pub interval: f64,
    /// 完了した試行回数
    pub trials_completed: u64,
    /// 経過時間（秒）
    pub elapsed_secs: u64,
    /// 経過時間（ナノ秒部分）
    pub elapsed_nanos: u32,
    /// 最後のサイコロの目
    pub last_dice: Vec<u8>,
}

impl GameState {
    /// 新しいGameStateを作成
    pub fn new(num_dice: usize, interval: f64) -> Self {
        Self {
            num_dice,
            interval,
            trials_completed: 0,
            elapsed_secs: 0,
            elapsed_nanos: 0,
            last_dice: Vec::new(),
        }
    }

    /// 状態を更新
    pub fn update(&mut self, trials: u64, elapsed_secs: u64, elapsed_nanos: u32, dice: Vec<u8>) {
        self.trials_completed = trials;
        self.elapsed_secs = elapsed_secs;
        self.elapsed_nanos = elapsed_nanos;
        self.last_dice = dice;
    }
}

/// 状態をファイルに保存
pub fn save_state(state: &GameState) -> AppResult<()> {
    let json = serde_json::to_string_pretty(state)
        .map_err(|e| AppError::InternalAppError(format!("JSON serialization failed: {}", e)))?;
    fs::write(STATE_FILE, json)
        .map_err(|e| AppError::InternalAppError(format!("Failed to write state file: {}", e)))?;
    Ok(())
}

/// 状態をファイルから読み込み
pub fn load_state() -> AppResult<Option<GameState>> {
    let path = Path::new(STATE_FILE);
    if !path.exists() {
        return Ok(None);
    }

    let json = fs::read_to_string(path)
        .map_err(|e| AppError::InternalAppError(format!("Failed to read state file: {}", e)))?;
    let state: GameState = serde_json::from_str(&json)
        .map_err(|e| AppError::InternalAppError(format!("JSON parse failed: {}", e)))?;
    Ok(Some(state))
}

/// 状態ファイルを削除
pub fn delete_state() -> AppResult<()> {
    let path = Path::new(STATE_FILE);
    if path.exists() {
        fs::remove_file(path)
            .map_err(|e| AppError::InternalAppError(format!("Failed to delete state file: {}", e)))?;
    }
    Ok(())
}

/// 状態ファイルが存在するか確認
pub fn state_exists() -> bool {
    Path::new(STATE_FILE).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_new() {
        let state = GameState::new(3, 0.5);
        assert_eq!(state.num_dice, 3);
        assert_eq!(state.interval, 0.5);
        assert_eq!(state.trials_completed, 0);
    }

    #[test]
    fn test_game_state_update() {
        let mut state = GameState::new(2, 1.0);
        state.update(10, 15, 500_000_000, vec![3, 3]);
        assert_eq!(state.trials_completed, 10);
        assert_eq!(state.elapsed_secs, 15);
        assert_eq!(state.elapsed_nanos, 500_000_000);
        assert_eq!(state.last_dice, vec![3, 3]);
    }
}
