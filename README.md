# 🎲 Dice Challenge

<img src="LOGO.png" alt="Dice Challenge" width="150" height="150">

A command-line tool that rolls multiple dice repeatedly until all dice show the same face (matching dice).

## Overview

This tool rolls dice at specified intervals and stops when all dice show the same number. Each trial displays the dice faces, trial count, elapsed time, and the probability of getting matching dice for the first time at that trial.

## Installation

```bash
git clone https://github.com/your-username/rs-dice-challenge.git
cd rs-dice-challenge
cargo build --release
```

The binary will be created at `./target/release/dice-challenge`.

## Usage

```bash
# Default (2 dice, 1 second interval)
./target/release/dice-challenge

# With options
./target/release/dice-challenge -n 3 -i 0.5  # 3 dice, 0.5 second interval

# Show help
./target/release/dice-challenge --help
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `-n, --num-dice` | Number of dice | 2 |
| `-i, --interval` | Interval between trials (seconds) | 1.0 |

## Output Example

```
🎲 サイコロチャレンジ開始！
サイコロ数: 2 / 間隔: 1秒
---
⚀ ⚁
number of trials: 000000000000005 / ETA: 00:05 / Prob: 11.56893004%

⚃ ⚃ ★
number of trials: 000000000000006 / ETA: 00:06 / Prob: 9.64077503%

🎉 ゾロ目達成！ 6 回目の試行で成功しました！
```

## Probability

- **Probability of matching dice**: $p = \frac{1}{6^{n-1}}$ (n = number of dice)
  - 2 dice: 16.67%
  - 3 dice: 2.78%
  - 4 dice: 0.46%

- **Probability of first match at k-th trial (Geometric Distribution)**: $P(X = k) = (1 - p)^{k-1} \times p$

## License

Apache-2.0
