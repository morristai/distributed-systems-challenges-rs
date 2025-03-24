# Distributed Systems Challenges by [Fly.io](https://fly.io)

- https://fly.io/dist-sys/

## Setup Maelstrom

```shell
sudo apt install -y openjdk graphviz gnuplot

xh --download https://github.com/jepsen-io/maelstrom/releases/download/v0.2.4/maelstrom.tar.bz2
tar xvjf maelstrom.tar.bz2
chmod +x maelstrom/maelstrom
```

## 1. Echo

```shell
cd challenge-1-echo
cargo build --release
../maelstrom/maelstrom test -w echo --bin ./target/release/challenge-1-echo --node-count 1 --time-limit 10 --log-stderr
```