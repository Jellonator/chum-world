# Chum World

A program for inspecting files in the Gamecube games Spongebob: Revenge of the Flying Dutchman and Jimmy Neutron: Boy Genius.

## Downloads

To download the latest Chum World release, check out the [Chum World Releases](https://github.com/Jellonator/chum-world/releases) page.

## Wiki

For information on each of the file formats, check out the [Chum World Wiki](https://github.com/Jellonator/chum-world/wiki).

## Project structure

This project is divided into four parts: libchum, gdchum, the Chum GUI, and chumcli.

### libchum

This is the library for reading/writing NGC/DGC archives. It is written in Rust.

### gdchum

This is an intermediary library for interaction between Godot and libchum. It is also written in Rust.

### Chum GUI

This is the GUI for Chum World, which can inspect TotemTech archives (NGC/DGC file pairs). This part is made with the Godot game engine.

### chumcli

This is the CLI for Chum World, written in Rust.

## Compiling

To compile Chum World, you will need to download the Rust compiler: [https://www.rust-lang.org/](https://www.rust-lang.org/). For the GUI, you will also need to download the Godot game engine: [https://godotengine.org/](https://godotengine.org/).

### Compiling the GUI

To compile the GUI, first navigate to the `gdchum` folder, then enter the following:
```
cargo build --release
```
You can now open the project in Godot. Open Godot, then press the `Import` button on the right. Open the `project.gd` file. Now click on the project in the project list. Finally, click either the `Edit` button or the `Run` botton on the right to edit or run the project, respectively.

### Compiling the CLI

To compile the CLI, first navigate to the `chumcli` folder, then enter the following:
```
cargo build --release
```
The resulting binary should be in the `chumcli/target/release` folder. On Windows, this file is `chumcli\target\release\chumcli.exe`. On Linux, this file is `chumcli/target/release/chumcli`.
