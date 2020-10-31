
# implot-rs

Rust bindings for [ImPlot](https://github.com/epezent/implot), built by running
[bindgen](https://github.com/rust-lang/rust-bindgen) on [cimplot](https://github.com/cimgui/cimplot).

The bindings are currently based on ImPlot version 0.8-WIP (see 
[implot-sys/third-party](implot-sys/third-party) for the exact commit currently pointed to). 
The status section below provides detailed information on implementation status. 

[![Docs.rs documentation](https://docs.rs/implot/badge.svg)](https://docs.rs/implot/)
![Tests](https://github.com/4bb4/implot-rs/workflows/Tests/badge.svg)

**Important note:** As long as the code is pre-1.0 release, the API is expected to have 
breaking changes between minor versions. Patch versions should be backwards compatible. 
After 1.0, semver will be followed more properly.

![demo](demo.png)

## Requirements
imgui-rs requires minimum Rust version 1.40, so this project does as well by extension.
The sys crate compiles implot, so a C++ compiler will also be required.

## Examples 
Examples are being built in the `implot-examples` crate in this repo. To try them out, 
clone the repo, change into the `implot-examples` directory and try for example
```
  cargo run --example line_plots
```

## Documentation
For released versions, see 
[![Docs.rs documentation](https://docs.rs/implot/badge.svg)](https://docs.rs/implot/). 
Make sure you are looking at the right release, since the API is still changing. 
For the master branch, can build it yourself however by cloning this repo and then doing 
```
  cargo doc --open
```
An effort is made to document everything as it is being added. Feel free to open an issue
if documentation is unclear or lacking.


## Implementation status
Currently a work in progress, coverage of the C++ API is increased steadily. The author 
is open to collaboration, if you'd like to help, feel free to reach out via a Github issue.

At this point, raw bindings are working in implot-sys, and more idiomatic interfaces
for plot creation as well a subset of the functionality for plots are implemented. 

While the raw bindings have versions of most functions for different data types such as
32-bit or 64-bit floats and various integers, the higher-level bindings are currently only
created for 64-bit floats.

- [x] "BeginPlot"
  - [x] Basic hello world
  - [x] Plot flags
- [ ] Plotting functionality
  - [x] Line plot
  - [x] Text plot
  - [x] Scatter plot
  - [x] Bar plot
    - [x] Vertical
    - [x] Horizontal
  - [ ] Shaded plot
  - [ ] Stem plots
  - [ ] Images
  - [ ] Error bar plot
    - [ ] Vertical
    - [ ] Horizontal
  - [ ] Heatmap
  - [ ] Pie chart
  - [ ] Digital data
  - [ ] Stairs plot
  - [ ] Annotations
  - [ ] Dragline
  - [ ] Dragpoint
- [ ] Plot customization
  - [x] Axis flags
  - [x] Styling colors
  - [x] Styling variables
  - [x] Colormaps
  - [ ] Legend locations
- [x] Plot querying 
  - [x] is hovered
  - [x] mouse position in plot
  - [x] plot limits
  - [x] is queried
  - [x] get plot query
  - [x] are axes hovered
  - [x] Choice of y axis
- [ ] Utils
  - [x] Plot limit setting
  - [x] imgui-rs style safe push/pop stacks
  - [x] Plot tick setting
  - [ ] Input remapping
  - [x] Set Y axis setting for subsequent elements
  - [ ] Set non-default Y axis ticks and labels
  - [ ] Plot position and size reading
  - [ ] Pixel to plot position
  - [ ] Plot to pixel position
  - [ ] Push/pop plotclip rect (?)

# Developer documentation
## Design approach
This repo tries to follow the approaches and style used in `imgui-rs` somewhat closely,
because implot is to be used within imgui programs, and hence keeping the interfaces
and design philosophies close should make it easier to do that.

If you spot any design inconsistencies or paper cuts, feel free to open an issue.
