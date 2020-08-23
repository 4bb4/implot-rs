# implot-rs

Rust bindings for [ImPlot](https://github.com/epezent/implot), built by running
[bindgen](https://github.com/rust-lang/rust-bindgen) on [cimplot](https://github.com/cimgui/cimplot).

![demo](demo.png)

## Requirements
imgui-rs requires minimum Rust version 1.40, so this project does as well by extension.
Currently the master branch of imgui-rs is used as a dependency until 
https://github.com/Gekkio/imgui-rs/pull/339 makes it into a release.

The sys crate compiles implot, so a C++ compiler will also be required.

## Status
Currently a work in progress. The author is open to collaboration, if you'd like to 
help, feel free to reach out via a Github issue.

At this point, raw bindings are working in implot-sys, and more idiomatic interfaces
for plot creation as well as adding lines to plots are implemented. Everything else 
is still being built.

- [x] "BeginPlot"
  - [x] Basic hello world
  - [x] Plot flags
- [ ] Plotting functionality
  - [ ] Line plot
    - [x] Single y axis
    - [ ] Multiple y axes
  - [ ] Scatter plot
  - [ ] Bar plot
    - [ ] Vertical
    - [ ] Horizontal
  - [ ] Error bar plot
    - [ ] Vertical
    - [ ] Horizontal
  - [ ] Text
  - [ ] Heatmap
  - [ ] Pie chart
  - [ ] Digital data
- [ ] Plot customization
  - [ ] Enums
    - [x] Axis flags
    - [ ] Markers
    - [ ] Styling colors
    - [ ] Styling variables
    - [ ] Colormaps
- [ ] Plot querying 
  - [ ] is hovered
  - [ ] mouse position
  - [ ] mouse limits
  - [ ] is queried
  - [ ] GetPlotQuery
- [ ] Utils
  - [x] Plot limit setting
  - [ ] Plot tick setting
  - [ ] Plot y axis setting for subsequent elements
  - [ ] plot position and size reading
  - [ ] Pixel to plot position
  - [ ] Plot to pixel position
  - [ ] Push/pop plotclip rect (?)
