#!/bin/bash

trap ctrl_c INT

function ctrl_c() {
  kill $(jobs -p)
}

RUST_LOG=raft ./target/debug/raftd --bind 127.0.0.1:5000 --connect 127.0.0.1:5001 127.0.0.1:5002 &
RUST_LOG=raft ./target/debug/raftd --bind 127.0.0.1:5001 --connect 127.0.0.1:5000 127.0.0.1:5002 &
RUST_LOG=raft ./target/debug/raftd --bind 127.0.0.1:5002 --connect 127.0.0.1:5000 127.0.0.1:5001 &

wait $(jobs -p)
