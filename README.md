# Chum World

A program for inspecting files in the Gamecube games Spongebob: Revenge of the Flying Dutchman and Jimmy Neutron: Boy Genius.

To download the latest Chum World release, check out the [Chum World Releases](https://github.com/Jellonator/chum-world/releases) page.

For information on each of the file formats, check out the [Chum World Wiki](https://github.com/Jellonator/chum-world/wiki).

This project is divided into four parts: libchum, gdchum, the Chum GUI, and chumcli.

## libchum

This is the library for reading/writing NGC/DGC archives. It is written in Rust.

## gdchum

This is an intermediary library for interaction between Godot and libchum. It is also written in Rust.

## Chum GUI

This is the GUI for Chum World, which can inspect TotemTech archives (NGC/DGC file pairs). This part is made with the Godot game engine.

## chumcli

This is the CLI for Chum World, written in Rust.
