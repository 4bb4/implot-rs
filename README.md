# implot-rs

Rust bindings for [ImPlot](https://github.com/epezent/implot), built by running
[bindgen](https://github.com/rust-lang/rust-bindgen) on [cimplot](https://github.com/cimgui/cimplot).

![demo](demo.png)

## Requirements
imgui-rs requires minimum Rust version 1.40, so this project does as well by extension.
Currently the master branch of imgui-rs is used as a dependency until 
https://github.com/Gekkio/imgui-rs/pull/339 makes it into a release.

The sys crate compiles implot, so a C++ compiler will also be required.

## Design approach
This repo tries to follow the approaches and style used in `imgui-rs` somewhat closely,
because implot is to be used within imgui programs, and hence keeping the interfaces
and design philosophies close should make it easier to do that.

If you spot any design inconsistencies or papercuts, feel free to open an issue.

## Status
Currently a work in progress. The author is open to collaboration, if you'd like to 
help, feel free to reach out via a Github issue.

Note that the API is not stabilized yet and expected to change as development progresses.
Once there are actual releases on crates.io, semantic versioning will be followed.

At this point, raw bindings are working in implot-sys, and more idiomatic interfaces
for plot creation as well a subset of the functionality for plots are implemented. 
See below for an overview of the progress.

- [x] "BeginPlot"
  - [x] Basic hello world
  - [x] Plot flags
- [ ] Plotting functionality
  - [x] Line plot
  - [x] Text plot
  - [ ] Scatter plot
  - [ ] Bar plot
    - [ ] Vertical
    - [ ] Horizontal
  - [ ] Error bar plot
    - [ ] Vertical
    - [ ] Horizontal
  - [ ] Heatmap
  - [ ] Pie chart
  - [ ] Digital data
- [ ] Plot customization
  - [x] Axis flags
  - [x] Styling colors
  - [x] Styling variables
  - [ ] Colormaps
- [ ] Plot querying 
  - [x] is hovered
  - [x] mouse position in plot
  - [x] plot limits
  - [x] is queried
  - [x] get plot query
  - [ ] Choice of y axis
- [ ] Utils
  - [x] Plot limit setting
  - [x] imgui-rs style safe push/pop stacks
  - [ ] Plot tick setting
  - [ ] Set Y axis setting for subsequent elements
  - [ ] Plot position and size reading
  - [ ] Pixel to plot position
  - [ ] Plot to pixel position
  - [ ] Push/pop plotclip rect (?)
