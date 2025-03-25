# Distributed Systems Challenges by [Fly.io](https://fly.io)

- https://fly.io/dist-sys/

## Setup Maelstrom

```shell
sudo apt install -y openjdk graphviz gnuplot

xh --download https://github.com/jepsen-io/maelstrom/releases/download/v0.2.4/maelstrom.tar.bz2
tar xvjf maelstrom.tar.bz2
chmod +x maelstrom/maelstrom
```

## Run Challenges

```shell
cd challenge-x
cargo build --release

# 1. Echo
../maelstrom/maelstrom test -w echo --bin ./target/release/challenge-1-echo --node-count 1 --time-limit 10 --log-stderr
# 2. Unique-id
../maelstrom/maelstrom test -w unique-ids --bin ./target/release/challenge-2-unique-id --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
# 3a. Broadcast
../maelstrom/maelstrom test -w broadcast --bin ./target/release/challenge-3-broadcast --node-count 1 --time-limit 20 --rate 10
# 3b. Multi-Node Broadcast
../maelstrom/maelstrom test -w broadcast --bin ./target/release/challenge-3-broadcast --node-count 5 --time-limit 20 --rate 10
# 3c. FIXME: Fault Tolerant Broadcast
../maelstrom/maelstrom test -w broadcast --bin ./target/release/challenge-3-broadcast --node-count 5 --time-limit 20 --rate 10 --nemesis partition
# 3d. Efficient Broadcast, Part I & Part II
../maelstrom/maelstrom test -w broadcast --bin ./target/release/challenge-3-broadcast --node-count 25 --time-limit 20 --rate 100 --latency 100
```

## WaitGroup in Rust

- [Waitgroup in Tokio Discussion](https://github.com/tokio-rs/tokio/discussions/5319)
- [TaskTracker](https://docs.rs/tokio-util/latest/tokio_util/task/task_tracker/struct.TaskTracker.html)