# Icon Viewer Design Docs

These docs should describe the basic structure and functionality of the app, as well as why certain decisions were made. They should be helpful to both humans and LLMs to ensure that the code works as intended and has a coherent and sane design.

## App Overview

A Rust app using the `egui` and `eframe` libraries:

- On launch it should create a window containing a list of all program icons from the current machine.
- The window should be resizable, but should start taking the full space available on the current monitor.
- The list should be vertically scrollable.
- Pressing `Esc`, `Ctrl+D`, or `Ctrl+C` should exit the app.
- All standard icon types should be rendered, including `SVG` and `PNG`.
