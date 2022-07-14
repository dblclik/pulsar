# Pulsar

## Overview
Pulsar is a simple, multi-threaded metrics emitter for testing UDP listeners.  When paired with [Neith](https://www.github.com/dblclik/neith) it represents a valuable tool for load and integration testing.

## Developing
To re-use for your own purposes, change the `emit()` function to behave as you would like.

## TODOs
- Add Clap integration for CLI args
- Support mutliple emit functions
- Persistent (until SIGTERM) emissions