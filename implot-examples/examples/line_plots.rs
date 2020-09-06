// This example demonstrates how line plots are to be used, along with some querying features
// that will be applicable to all kinds of plots.

use imgui::{im_str, CollapsingHeader, Condition, Ui, Window};
use implot::{
    get_plot_limits, get_plot_mouse_position, get_plot_query, is_plot_hovered, is_plot_queried,
    push_style_color, push_style_var_f32, push_style_var_u32, AxisFlags, ImPlotLimits, ImPlotPoint,
    ImPlotRange, Marker, Plot, PlotColorElement, PlotFlags, PlotLine, StyleVar,
};

mod support;

fn show_basic_plot(ui: &Ui) {
    ui.text(im_str!(
        "This header just plots a line with as little code as possible."
    ));
    let content_width = ui.window_content_region_width();
    Plot::new("Simple line plot")
        // The size call could also be omitted, though the defaults don't consider content
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(|| {
            // If this is called outside a plot build callback, the program will panic.
            PlotLine::new("A line").plot(&vec![0.1, 0.9], &vec![0.1, 0.9]);
        });
}

fn show_configurable_plot(ui: &Ui) {
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
    let plot_flags = PlotFlags::DEFAULT;
    // - Axis flags, see the AxisFlags docs for more info. All flags are bitflags-created,
    //   so they support a bunch of convenient operations, see https://docs.rs/bitflags
    let x_axis_flags = AxisFlags::DEFAULT - AxisFlags::TICK_LABELS;
    let y_axis_flags = AxisFlags::DEFAULT;

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
            Condition::Always,
        )
        // If any of these flag setting calls are omitted, the defaults are used.
        .with_plot_flags(&plot_flags)
        .with_x_axis_flags(&x_axis_flags)
        .with_y_axis_flags(&y_axis_flags)
        .build(|| {
            PlotLine::new("A line").plot(&vec![2.1, 2.9], &vec![1.1, 1.9]);
        });
}

fn show_query_features_plot(ui: &Ui) {
    ui.text(im_str!(
        "This header demos how to use the querying features."
    ));
    let content_width = ui.window_content_region_width();

    // Create some containers for exfiltrating data from the closure below
    let mut hover_pos: Option<ImPlotPoint> = None;
    let mut plot_limits: Option<ImPlotLimits> = None;
    let mut query_limits: Option<ImPlotLimits> = None;

    // Draw a plot
    Plot::new("Plot querying")
        .size(content_width, 300.0)
        .x_limits(&ImPlotRange { Min: 0.0, Max: 5.0 }, Condition::FirstUseEver)
        .y_limits(&ImPlotRange { Min: 0.0, Max: 5.0 }, Condition::FirstUseEver)
        .build(|| {
            if is_plot_hovered() {
                hover_pos = Some(get_plot_mouse_position());
            }

            if is_plot_queried() {
                query_limits = Some(get_plot_query());
            }
            plot_limits = Some(get_plot_limits());
        });

    // Print some previously-exfiltrated info. This is because calling
    // things like is_plot_hovered or get_plot_mouse_position() outside
    // of an actual Plot is not allowed.
    if let Some(pos) = hover_pos {
        ui.text(im_str!("hovered at {}, {}", pos.x, pos.y));
    }
    if let Some(limits) = plot_limits {
        ui.text(im_str!("Plot limits are {:#?}", limits));
    }
    if let Some(query) = query_limits {
        ui.text(im_str!("Query limits are {:#?}", query));
    }
}

fn show_style_plot(ui: &Ui) {
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
            Condition::Always,
        )
        .with_plot_flags(&(PlotFlags::DEFAULT))
        .with_y_axis_flags(&(AxisFlags::DEFAULT))
        .build(|| {
            // Markers can be selected as shown here. The markers are internally represented
            // as an u32, hence this calling style.
            let markerchoice = push_style_var_u32(&StyleVar::Marker, Marker::CROSS.bits());
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

fn main() {
    let system = support::init(file!());
    let mut showing_demo = false;
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello world"))
            .size([430.0, 450.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello from implot-rs!"));
                ui.text_wrapped(im_str!(
                    "The headers here demo some of the features of the library. \
                    Have a look at the example source code to see how they are 
                    implemented.\n\
                    Check out the demo from ImPlot itself first \
                    (by enabling the 'Show demo' checkbox) for instructions \
                    on how to interact with ImPlot plots."
                ));
                ui.checkbox(im_str!("Show demo"), &mut showing_demo);

                // Show individual examples in collapsed headers
                if CollapsingHeader::new(im_str!("Basic lineplot")).build(&ui) {
                    show_basic_plot(&ui);
                }
                if CollapsingHeader::new(im_str!("Configurable lineplot")).build(&ui) {
                    show_configurable_plot(&ui);
                }
                if CollapsingHeader::new(im_str!("Querying a plot")).build(&ui) {
                    show_query_features_plot(&ui);
                }
                if CollapsingHeader::new(im_str!("Styling a plot")).build(&ui) {
                    show_style_plot(&ui);
                }
            });

        if showing_demo {
            implot::show_demo_window(&mut showing_demo);
        }
    });
}
