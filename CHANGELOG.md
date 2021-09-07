# Changelog

## Unreleased
### General notes
* Changed to using imgui types from `imgui-rs`. This improves compatibility
  with other code that uses `imgui-rs` directly (help by: @a1ien)
* Reduced bindings size considerably by include-listing types (help by: @a1ien)
* Removed direct dependency on `lazy_static` (credit: @a1ien)
* Changed `i8` to `char` where appropriate (credit: @birktj)
* Updated bindgen dependency to 0.57

### API changes
* `Plot::size` now takes a `[f32; 2]` argument instead of two separate `f32` values
* Added conversions from `Range<f64>`, `[f64; 2]`, `(f64, f64)` and `ImVec2` to `ImPlotRange`
  (credit: @a1ien)
* `Plot::x_limits` and `Plot::y_limits` now take an `Into<ImPlotRange>` argument for the 
  limits. Combined with the above conversions, there are now more flexible ways to specify 
  limits.
* There are now convenience functions for setting the Y axis limits for individual axes, called
  `Plot::y1_limits`, `Plot::y2_limits` and `Plot::y3_limits`.
* There is now a feature to set linked Y limits - `Plot::linked_y_limits`, along with 
  convenience functions for the individual axes.

## v0.4.0
* Setting axis ratio
* Other minor additions
* Pinned imgui versions more tightly

## v0.3.0
* Heatmap support
* Doc aliases (for Rust 1.48 and newer)
* Simple style setters

## v0.2.0
* More API coverage and minor improvements
* Minor API breaks from v0.1.0.

## v0.1.0
* Added metadata to Cargo.toml
