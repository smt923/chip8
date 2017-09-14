# chip8 emulator

Emulating the [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) using Rust

![Tetris](https://i.imgur.com/eEb0CYg.png)

## problems
currently, the emulator will panic often when not in release mode due to overflows in the arithmetic opcodes

there also seems to be a bug with flickering moving sprites and possibly related; these sprites having broken collisions

## todo
- [X] cleanup opcode matching
- [X] add missing opcodes
- [X] setup display (SDL2)
- [X] iron out issue with screen not clearing
- [ ] figure out weird issue with flickering moving objects (seems to cause collisions issues too)
- [ ] probably a lot more