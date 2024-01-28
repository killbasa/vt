# VT

## Installation

### From script

The install script can be found [here](/install.sh).

```sh
curl -L -sSf https://raw.githubusercontent.com/killbasa/vt/main/install.sh | sh
```

### From source

Since the program needs to be compiled, [Rust](https://www.rust-lang.org/) is a requirement.

```sh
git clone https://github.com/killbasa/vt.git
cargo install --locked --path vt
```

## Example

```sh
vt config set apikey

vt channel set iori UCN5bD1YYapThOeadG7YkBOA # or aliased: vt ch set iori UCN5bD1YYapThOeadG7YkBOA
# Channel alias added: iori -> UCN5bD1YYapThOeadG7YkBOA

vt channel ls
# iori -> UCN5bD1YYapThOeadG7YkBOA

vt get iori
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
