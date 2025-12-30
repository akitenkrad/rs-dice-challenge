use rand::Rng;

/// サイコロの目を表すUnicode文字
const DICE_FACES: [char; 6] = ['⚀', '⚁', '⚂', '⚃', '⚄', '⚅'];

/// サイコロを振り、1〜6の値を返す
pub fn roll_dice() -> u8 {
    rand::thread_rng().gen_range(1..=6)
}

/// サイコロの目をUnicode文字に変換
pub fn dice_to_char(value: u8) -> char {
    DICE_FACES[(value - 1) as usize]
}

/// 複数のサイコロを振り、すべての出目を返す
pub fn roll_multiple_dice(count: usize) -> Vec<u8> {
    (0..count).map(|_| roll_dice()).collect()
}

/// ゾロ目かどうかを判定
pub fn is_all_same(dice: &[u8]) -> bool {
    if dice.is_empty() {
        return false;
    }
    let first = dice[0];
    dice.iter().all(|&d| d == first)
}

/// サイコロの出目を表示用文字列に変換
pub fn dice_to_display(dice: &[u8], is_zoromi: bool) -> String {
    let dice_str: String = dice
        .iter()
        .map(|&d| dice_to_char(d).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    if is_zoromi {
        format!("{} ★", dice_str)
    } else {
        dice_str
    }
}

/// n個のサイコロでゾロ目が出る確率を計算
/// p = 6 / 6^n = 1 / 6^(n-1)
pub fn zoromi_probability(num_dice: usize) -> f64 {
    if num_dice <= 1 {
        return 1.0;
    }
    1.0 / 6_f64.powi((num_dice - 1) as i32)
}

/// 幾何分布: k回目の試行で初めてゾロ目が出る確率
/// P(X = k) = (1 - p)^(k-1) * p
pub fn first_zoromi_probability(num_dice: usize, trial: u64) -> f64 {
    let p = zoromi_probability(num_dice);
    (1.0 - p).powi((trial - 1) as i32) * p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_dice_range() {
        for _ in 0..100 {
            let value = roll_dice();
            assert!(value >= 1 && value <= 6);
        }
    }

    #[test]
    fn test_dice_to_char() {
        assert_eq!(dice_to_char(1), '⚀');
        assert_eq!(dice_to_char(6), '⚅');
    }

    #[test]
    fn test_is_all_same() {
        assert!(is_all_same(&[1, 1, 1]));
        assert!(!is_all_same(&[1, 2, 1]));
        assert!(!is_all_same(&[]));
    }

    #[test]
    fn test_zoromi_probability() {
        // 2個のサイコロ: 1/6
        assert!((zoromi_probability(2) - 1.0 / 6.0).abs() < 1e-10);
        // 3個のサイコロ: 1/36
        assert!((zoromi_probability(3) - 1.0 / 36.0).abs() < 1e-10);
    }
}
