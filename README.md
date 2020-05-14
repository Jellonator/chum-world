# Chum World

A program for inspecting files in the Gamecube game Spongebob: Revenge of the Flying Dutchman. 

This project is divided into three parts: libchum, gdchum, and the Godot GUI.

## libchum

This is the library for reading/writing NGC/DGC archives. It is written in Rust.

## gdchum

This is an intermediary library for interaction between Godot and libchum. It is also written in Rust.

## Godot GUI

This is the GUI for Chum World, which can inspect TotemTech archives (NGC/DGC file pairs).
