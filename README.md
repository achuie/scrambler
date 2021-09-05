# Scrambler

A scramble generator for 3x3 Rubik's Cubes.

## Install

### With Nix

Initialize `Cargo.lock`:
```sh
$ nix-shell --command 'cargo update'
```
or use the convenience script `generate_lockfile.sh`.

Install:
```sh
$ nix-env -i -f default.nix
```

Enter a development environment providing the same rust version:
```sh
$ nix-shell
```

### Manually

Just use `cargo` like normal.

## Usage

### Random Move Generator

Choose sequence of random moves, with a heuristic to avoid immediate repeat
moves.

### IDA*

Coming soon!
