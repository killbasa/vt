<div align="center">

# vt

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Check live and upcoming YouTube streams from your terminal.

</div>

## Installation

### From script

The install script can be found [here](/install.sh).

```sh
curl -L -sSf https://raw.githubusercontent.com/killbasa/vt/main/install.sh | sh
```

### Manually

Precompiled binaries can be found in the [releases](https://github.com/killbasa/vt/releases) page.

### From source

Since the program needs to be compiled, [Rust](https://www.rust-lang.org/) is a requirement.

```sh
git clone https://github.com/killbasa/vt.git
cargo install --locked --path vt
```

## YouTube quota usage

The CLI uses the [list](https://developers.google.com/youtube/v3/docs/videos/list) endpoint which costs 1 out of 10,000 quota.

## Usage

### Setting your YouTube API key

```sh
# Set your YouTube API key from https://console.cloud.google.com
vt config set apikey
```

### Checking individual channels

```sh
# Add a channel for checking streams
vt channel create iori UCN5bD1YYapThOeadG7YkBOA

# Check all your channels
vt channel list

# Get live and upcoming streams
vt check iori
# [live] 【MINECRAFT】地下大冒険開始！無事生き延びれるのかっ！？going underground again:)【白鹿いおり Phase Connect】
#  ├─     url: https://www.youtube.com/watch?v=Z0qmy9eZ5kE
#  └─ started: 2 hours ago
#
# [upcoming] 【One Hand Clapping #1】響かせるぜ美しきビブラァトォ～【白鹿いおり Phase Connect】
#  ├─       url: https://www.youtube.com/watch?v=jmB2yR0R1bI
#  └─ scheduled: in 6 days
#
# [upcoming] 【Mafia Ⅲ: Definitive Edition】part 12! second mob lieutenant! on Twitch【白鹿いおり Phase Connect】
#  ├─       url: https://www.youtube.com/watch?v=zt-2r3R4EDE
#  └─ scheduled: in a week

```

### Checking a group of channels

```sh
# Create a group
vt groups create phase

# Add a channel to the group
vt groups add iori
vt groups add nasa

# Check the channels in a group
vt groups check phase
# [upcoming] 【コラボ対談】貴方はどうして天体が好き？【phase connect】
#  ├─   channel: Nasa Ch. 転寝ナサ 【Phase Connect】
#  ├─       url: https://www.youtube.com/watch?v=-e1OUTo4JVA
#  └─ scheduled: in 14 hours
#
# [upcoming] 【Duolingo】Learning Spanish for my First Time! on Twitch/スペイン語を勉強するよ【白鹿いおり Phase Connect】
#  ├─   channel: Iori Ch. 白鹿いおり【Phase Connect】
#  ├─       url: https://www.youtube.com/watch?v=PlYtKct94HI
#  └─ scheduled: in 3 days
#
# [upcoming] 【AmongUs】ワールドワイドな選手たち！【phase connect】
#  ├─   channel: Nasa Ch. 転寝ナサ 【Phase Connect】
#  ├─       url: https://www.youtube.com/watch?v=zFPMQoQOmhY
#  └─ scheduled: in 3 days
#
# [upcoming] 【Keep Talking and Nobody Explodes】天才二人にかかれば爆弾処理くらいおゆーwwwwな件について。w/@PinaPengin 【白鹿いおり Phase Connect】
#  ├─   channel: Iori Ch. 白鹿いおり【Phase Connect】
#  ├─       url: https://www.youtube.com/watch?v=QbOjOyn4a0M
#  └─ scheduled: in 4 days
```

## Contributing

### Install tooling

```sh
cargo install cargo-binstall
cargo binstall cargo-shear -y
```
