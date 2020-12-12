//! This example demonstrates how line plots are to be used, along with some querying features
//! that will be applicable to all kinds of plots.

use imgui::{im_str, CollapsingHeader, Condition, Ui};
use implot::{
    get_plot_limits, get_plot_mouse_position, get_plot_query, is_plot_hovered, is_plot_queried,
    pixels_to_plot_vec2, plot_to_pixels_vec2, push_style_color, push_style_var_f32,
    push_style_var_i32, set_colormap_from_preset, set_colormap_from_vec, set_plot_y_axis,
    AxisFlags, Colormap, ImPlotLimits, ImPlotPoint, ImPlotRange, ImVec2, ImVec4, Marker, Plot,
    PlotColorElement, PlotFlags, PlotLine, PlotLocation, PlotOrientation, PlotUi, StyleVar,
    YAxisChoice,
};

pub fn show_basic_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!(
        "This header just plots a line with as little code as possible."
    ));
    let content_width = ui.window_content_region_width();
    Plot::new("Simple line plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(plot_ui, || {
            // If this is called outside a plot build callback, the program will panic.
            let x_positions = vec![0.1, 0.9];
            let y_positions = vec![0.1, 0.9];
            PlotLine::new("legend label").plot(&x_positions, &y_positions);
        });
}

pub fn show_two_yaxis_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!(
        "This header shows how to create a plot with multiple Y axes."
    ));
    let content_width = ui.window_content_region_width();
    Plot::new("Multiple Y axis plots")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .with_plot_flags(&(PlotFlags::NONE | PlotFlags::Y_AXIS_2))
        .y_limits(
            &ImPlotRange { Min: 0.0, Max: 1.0 },
            YAxisChoice::First,
            Condition::Always,
        )
        .y_limits(
            &ImPlotRange { Min: 1.0, Max: 3.5 },
            YAxisChoice::Second,
            Condition::Always,
        )
        .build(plot_ui, || {
            let x_positions = vec![0.1, 0.9];

            // The first Y axis is the default
            let y_positions = vec![0.1, 0.9];
            PlotLine::new("legend label").plot(&x_positions, &y_positions);

            // Now we switch to the second axis for the next call
            set_plot_y_axis(YAxisChoice::Second);
            let y_positions = vec![3.3, 1.2];
            PlotLine::new("legend label two").plot(&x_positions, &y_positions);
        });
}

pub fn show_axis_equal_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!("This plot has axis equal set (1:1 aspect ratio)."));
    let content_width = ui.window_content_region_width();
    Plot::new("Axis equal line plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .with_plot_flags(&(PlotFlags::NONE | PlotFlags::AXIS_EQUAL))
        .build(plot_ui, || {
            // If this is called outside a plot build callback, the program will panic.
            let x_positions = vec![0.1, 0.9];
            let y_positions = vec![0.1, 0.9];
            PlotLine::new("legend label").plot(&x_positions, &y_positions);
        });
}

pub fn show_configurable_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!(
        "This header demos what we can configure about plots."
    ));

    // Settings for the plot
    // - X and Y size in pixels
    let x_size = 300.0;
    let y_size = 200.0;
    // - Strings for the axis labels
    let x_label = "X label!";
    let y_label = "Y label!";
    // - Plot limits
    let x_min = 2.0;
    let x_max = 3.0;
    let y_min = 1.0;
    let y_max = 2.0;
    // - Plot flags, see the PlotFlags docs for more info
    let plot_flags = PlotFlags::NONE;
    // - Axis flags, see the AxisFlags docs for more info. All flags are bitflags-created,
    //   so they support a bunch of convenient operations, see https://docs.rs/bitflags
    let x_axis_flags = AxisFlags::NONE;
    let y_axis_flags = AxisFlags::NONE;

    // - Unlabelled X axis ticks
    let x_ticks = vec![2.2, 2.5, 2.8];

    // - Labelled Y axis ticks
    let y_ticks = vec![(1.1, "A".to_owned()), (1.4, "B".to_owned())];

    // Axis labels
    Plot::new("Configured line plot")
        .size(x_size, y_size)
        .x_label(&x_label)
        .y_label(&y_label)
        .x_limits(
            &ImPlotRange {
                Min: x_min,
                Max: x_max,
            },
            // Always means that the limits stay what we force them to here, even if the user
            // scrolls or drags in the plot with the mouse. FirstUseEver sets the limits the
            // first time the plot is drawn, but the user can then modify them and the change
            // will stick.
            Condition::Always,
        )
        .y_limits(
            &ImPlotRange {
                Min: y_min,
                Max: y_max,
            },
            YAxisChoice::First,
            Condition::Always,
        )
        .x_ticks(&x_ticks, false)
        .y_ticks_with_labels(YAxisChoice::First, &y_ticks, false)
        // If any of these flag setting calls are omitted, the defaults are used.
        .with_plot_flags(&plot_flags)
        .with_x_axis_flags(&x_axis_flags)
        .with_y_axis_flags(YAxisChoice::First, &y_axis_flags)
        .with_legend_location(&PlotLocation::West, &PlotOrientation::Horizontal, true)
        .build(plot_ui, || {
            PlotLine::new("A line 2").plot(&vec![2.4, 2.9], &vec![1.1, 1.9]);
        });
}

pub fn show_query_features_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!(
        "This header demos how to use the querying features."
    ));
    let content_width = ui.window_content_region_width();

    // Create some containers for exfiltrating data from the closure below
    let mut hover_pos_plot: Option<ImPlotPoint> = None;
    let mut hover_pos_pixels: Option<ImVec2> = None;
    let mut hover_pos_from_pixels: Option<ImPlotPoint> = None;
    let mut plot_limits: Option<ImPlotLimits> = None;
    let mut query_limits: Option<ImPlotLimits> = None;

    // Draw a plot
    Plot::new("Plot querying")
        .size(content_width, 300.0)
        .x_limits(&ImPlotRange { Min: 0.0, Max: 5.0 }, Condition::FirstUseEver)
        .y_limits(
            &ImPlotRange { Min: 0.0, Max: 5.0 },
            YAxisChoice::First,
            Condition::FirstUseEver,
        )
        .with_plot_flags(&(PlotFlags::NONE | PlotFlags::QUERY))
        .build(plot_ui, || {
            if is_plot_hovered() {
                hover_pos_plot = Some(get_plot_mouse_position(None));
                hover_pos_pixels = Some(plot_to_pixels_vec2(&(hover_pos_plot.unwrap()), None));
            }

            // Getting the plot position from pixels also works when the plot is not hovered,
            // the coordinates are then simply outside the visible range.
            hover_pos_from_pixels = Some(pixels_to_plot_vec2(
                &ImVec2 {
                    x: ui.io().mouse_pos[0],
                    y: ui.io().mouse_pos[1],
                },
                None,
            ));

            if is_plot_queried() {
                query_limits = Some(get_plot_query(None));
            }
            plot_limits = Some(get_plot_limits(None));
        });

    // Print some previously-exfiltrated info. This is because calling
    // things like is_plot_hovered or get_plot_mouse_position() outside
    // of an actual Plot is not allowed.
    if let Some(pos) = hover_pos_plot {
        ui.text(im_str!("hovered at {}, {}", pos.x, pos.y));
    }
    if let Some(pixel_position) = hover_pos_pixels {
        // Try out converting plot mouse position to pixel position
        ui.text(im_str!(
            "pixel pos from plot:  {}, {}",
            pixel_position.x,
            pixel_position.y
        ));
        ui.text(im_str!(
            "pixel pos from imgui: {}, {}",
            ui.io().mouse_pos[0],
            ui.io().mouse_pos[1]
        ));
    }
    if let Some(limits) = plot_limits {
        ui.text(im_str!("Plot limits are {:#?}", limits));
    }
    if let Some(query) = query_limits {
        ui.text(im_str!("Query limits are {:#?}", query));
    }

    // Try out converting pixel position to plot position
    if let Some(pos) = hover_pos_from_pixels {
        ui.text(im_str!("plot pos from imgui: {}, {}", pos.x, pos.y,));
    }
}

pub fn show_style_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!(
        "This header demos how to use the styling features."
    ));
    let content_width = ui.window_content_region_width();

    // The style stack works the same as for other imgui things - we can push
    // things to have them apply, then pop again to undo the change. In implot-rs,
    // pushing returns a value on which we have to call .pop() later. Pushing
    // variables can be done outside of plot calls as well.
    let style = push_style_color(&PlotColorElement::PlotBg, 1.0, 1.0, 1.0, 0.2);
    Plot::new("Style demo plot")
        .size(content_width, 300.0)
        .x_limits(&ImPlotRange { Min: 0.0, Max: 6.0 }, Condition::Always)
        .y_limits(
            &ImPlotRange {
                Min: -1.0,
                Max: 3.0,
            },
            YAxisChoice::First,
            Condition::Always,
        )
        .with_plot_flags(&(PlotFlags::NONE))
        .with_y_axis_flags(YAxisChoice::First, &(AxisFlags::NONE))
        .build(plot_ui, || {
            // Markers can be selected as shown here. The markers are internally represented
            // as an u32, hence this calling style.
            let markerchoice = push_style_var_i32(&StyleVar::Marker, Marker::Cross as i32);
            PlotLine::new("Left eye").plot(&vec![2.0, 2.0], &vec![2.0, 1.0]);
            // Calling pop() on the return value of the push above will undo the marker choice.
            markerchoice.pop();

            // Line weights can be set the same way, along with some other things - see
            // the docs of StyleVar for more info.
            let lineweight = push_style_var_f32(&StyleVar::LineWeight, 5.0);
            PlotLine::new("Right eye").plot(&vec![4.0, 4.0], &vec![2.0, 1.0]);
            lineweight.pop();

            let x_values = vec![1.0, 2.0, 4.0, 5.0];
            let y_values = vec![1.0, 0.0, 0.0, 1.0];
            PlotLine::new("Mouth").plot(&x_values, &y_values);
        });

    style.pop();
}

pub fn show_colormaps_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!("This header demos how to select colormaps."));
    let content_width = ui.window_content_region_width();

    // Select a colormap from the presets. The presets are listed in the Colormap enum
    // and usually have something from 9 to 11 colors in them, with the second number
    // being the option to resample the colormap to a custom number of colors if picked
    // higher than 1.
    set_colormap_from_preset(Colormap::Plasma, 1);

    Plot::new("Colormap demo plot")
        .size(content_width, 300.0)
        .build(plot_ui, || {
            (1..10)
                .map(|x| x as f64 * 0.1)
                .map(|x| PlotLine::new(&format!("{:3.3}", x)).plot(&vec![0.1, 0.9], &vec![x, x]))
                .count();
        });

    // One can also specify a colormap as a vector of RGBA colors. ImPlot uses ImVec4 for this,
    // so we follow suit. Make sure to set the last number (w in ImVec4) to 1.0 to see anything -
    // it's the alpha channel.
    set_colormap_from_vec(vec![
        ImVec4 {
            x: 0.9,
            y: 0.9,
            z: 0.0,
            w: 1.0,
        },
        ImVec4 {
            x: 0.0,
            y: 0.9,
            z: 0.9,
            w: 1.0,
        },
    ]);

    Plot::new("Colormap demo plot #2")
        .size(content_width, 300.0)
        .build(plot_ui, || {
            (1..10)
                .map(|x| x as f64 * 0.1)
                .map(|x| PlotLine::new(&format!("{:3.3}", x)).plot(&vec![0.1, 0.9], &vec![x, x]))
                .count();
        });

    // Colormaps are not pushed, they are simply set, because they don't stack or anything.
    // We can reset to the default by just setting the "Standard" preset.
    set_colormap_from_preset(Colormap::Standard, 0);
}

pub fn show_demo_headers(ui: &Ui, plot_ui: &PlotUi) {
    if CollapsingHeader::new(im_str!("Line plot: Basic")).build(&ui) {
        show_basic_plot(&ui, &plot_ui);
    }
    if CollapsingHeader::new(im_str!("Line plot: Configured")).build(&ui) {
        show_configurable_plot(&ui, &plot_ui);
    }
    if CollapsingHeader::new(im_str!("Line Plot: Plot queries")).build(&ui) {
        show_query_features_plot(&ui, &plot_ui);
    }
    if CollapsingHeader::new(im_str!("Line plot: Plot styling")).build(&ui) {
        show_style_plot(&ui, &plot_ui);
    }
    if CollapsingHeader::new(im_str!("Line plot: Colormaps")).build(&ui) {
        show_colormaps_plot(&ui, &plot_ui);
    }
    if CollapsingHeader::new(im_str!("Line plot: Multiple Y Axes")).build(&ui) {
        show_two_yaxis_plot(&ui, &plot_ui);
    }
    if CollapsingHeader::new(im_str!("Line plot: \"Axis equal\"")).build(&ui) {
        show_axis_equal_plot(&ui, &plot_ui);
    }
}
