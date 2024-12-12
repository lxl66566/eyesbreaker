# Eyesbreaker

A small toy project to prevent you from staring at the screen for too long. It will create a system tray icon, where the icon will light up (turn green) after a given time, prompting you to stand up, stretch, and relax your eyes.

一个玩具小项目，让你不要长时间盯着屏幕。它会创建一个系统托盘图标，在给定时间后图标会亮起（变为绿色），此时请站起来活动活动，放松一下眼睛。

Only tested on windows. If you are using Linux, please try [safeeyes](https://github.com/slgobinath/SafeEyes).

该应用主要支持 windows 平台。在 Linux 下，你可以使用 [safeeyes](https://github.com/slgobinath/SafeEyes)。

## Usage

Moving the mouse over the tray icon will reset the timer when it's green. At any time, left click the tray icon will reset the timer, right click will exit the program. Hover the mouse over the icon to view the remaining time.

计时结束后将鼠标移动到托盘上，以重置计时器；在任意时刻左击重置计时器，右击图标以退出程序；将鼠标悬停在图标上以查看剩余时间。

## Installation

- use [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):
  ```sh
  cargo binstall eyesbreaker
  ```
- use [bpm](https://github.com/lxl66566/bpm):
  ```sh
  bpm i https://github.com/lxl66566/eyesbreaker
  ```
- manual: download binary from [release](https://github.com/lxl66566/eyesbreaker/releases)
- compile from source:
  ```sh
  cargo install eyesbreaker
  ```

## Usage

```
Usage: eyesbreaker.exe [OPTIONS]

Options:
  -t, --time <TIME>  Count down time (in seconds) [default: 1800]
  -h, --help         Print help
  -V, --version      Print version
```

## Additional

If you need start this program automatically on boot, please use [user-startup-py](https://github.com/lxl66566/user-startup-py).

如果你需要开机自启，可以看看 [user-startup-rs](https://github.com/lxl66566/user-startup-rs)。
