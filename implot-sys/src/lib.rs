#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// just for linking for tests
#[cfg(test)]
use imgui_sys as _;

use std::ops::Range;
include!("bindings.rs");

impl From<Range<f64>> for ImPlotRange {
    fn from(from: Range<f64>) -> Self {
        ImPlotRange {
            Min: from.start,
            Max: from.end,
        }
    }
}

impl From<[f64; 2]> for ImPlotRange {
    fn from(from: [f64; 2]) -> Self {
        ImPlotRange {
            Min: from[0],
            Max: from[1],
        }
    }
}

impl From<(f64, f64)> for ImPlotRange {
    fn from(from: (f64, f64)) -> Self {
        ImPlotRange {
            Min: from.0,
            Max: from.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_range_from_range() {
        let r = 5.0..7.0;
        let im_range: ImPlotRange = r.clone().into();
        assert_eq!(im_range.Min, r.start);
        assert_eq!(im_range.Max, r.end);

        let arr = [7.0, 8.0];
        let im_range: ImPlotRange = arr.clone().into();
        assert_eq!(im_range.Min, arr[0]);
        assert_eq!(im_range.Max, arr[1]);

        let tuple = (12.0, 19.0);
        let im_range: ImPlotRange = tuple.clone().into();
        assert_eq!(im_range.Min, tuple.0);
        assert_eq!(im_range.Max, tuple.1);
    }
}
