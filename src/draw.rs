#![allow(dead_code, unreachable_code, unused)]

pub struct CornerPathParams {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    p: f64,
    corner_radius: f64,
    arc_section_length: f64,
}

pub struct CornerParams {
    pub corner_radius: f64,
    pub corner_smoothing: f64,
    pub preserve_smoothing: bool,
    pub rounding_and_smoothing_budget: f64,
}

#[inline]
fn to_radians(degrees: f64) -> f64 {
    (degrees * std::f64::consts::PI) / 180.0
}

// The article from figma's blog
// https://www.figma.com/blog/desperately-seeking-squircles/
//
// The original code by MartinRGB
// https://github.com/MartinRGB/Figma_Squircles_Approximation/blob/bf29714aab58c54329f3ca130ffa16d39a2ff08c/js/rounded-corners.js#L64
pub fn get_path_params_for_corner(corner_params: CornerParams) -> CornerPathParams {
    let corner_radius = corner_params.corner_radius;
    let mut corner_smoothing = corner_params.corner_smoothing;
    let preserve_smoothing = corner_params.preserve_smoothing;
    let rounding_and_smoothing_budget = corner_params.rounding_and_smoothing_budget;

    // From figure 12.2 in the article
    // p = (1 + corner_smoothing) * q
    // in this case q = R because theta = 90deg
    let mut p = (1.0 + corner_smoothing) * corner_radius;

    // When there's not enough space left (p > roundingAndSmoothingBudget), there are 2 options:
    //
    // 1. What figma's currently doing: limit the smoothing value to make sure p <= roundingAndSmoothingBudget
    // But what this means is that at some point when cornerRadius is large enough,
    // increasing the smoothing value wouldn't do anything
    //
    // 2. Keep the original smoothing value and use it to calculate the bezier curve normally,
    // then adjust the control points to achieve similar curvature profile
    //
    // preserveSmoothing is an option added by @phamfoo
    //
    // If preserveSmoothing is on then we'll just keep using the original smoothing value
    // and adjust the bezier curve later
    if !preserve_smoothing {
        let max_corner_smoothing = (rounding_and_smoothing_budget / corner_radius) - 1.0;
        // Minimum between `corner_smoothing` and `max_corner_smoothing`
        corner_smoothing = f64::min(corner_smoothing, max_corner_smoothing);
        p = f64::min(p, rounding_and_smoothing_budget);
    }

    let arc_measure = 90.0 * (1.0 - corner_smoothing);
    let arc_section_length =
        (to_radians(arc_measure / 2.0)).sin() * corner_radius * (2.0_f64).sqrt();

    // In the article this is the distance between 2 control points: P3 and P4
    let angle_alpha = (90.0 - arc_measure) / 2.0;
    let p3_to_p4_distance = corner_radius * (to_radians(angle_alpha / 2.0)).tan();

    // a, b, c and d are from figure 11.1 in the article
    let angle_beta = 45.0 * corner_smoothing;
    let angle_beta_rad = to_radians(angle_beta);
    let c = p3_to_p4_distance * (angle_beta_rad).cos();
    let d = c * (angle_beta_rad).tan();

    let mut b = (p - arc_section_length - c - d) / 3.0;
    let mut a = 2.0 * b;

    // Adjust the P1 and P2 control points if there's not enough space left
    if preserve_smoothing && p > rounding_and_smoothing_budget {
        let p1_to_p3_max_distance = rounding_and_smoothing_budget - d - arc_section_length - c;

        // Try to maintain some distance between P1 and P2 so the curve wouldn't look weird
        let min_a = p1_to_p3_max_distance / 6.0;
        let max_b = p1_to_p3_max_distance - min_a;

        b = f64::min(b, max_b);
        a = p1_to_p3_max_distance - b;
        p = f64::min(p, rounding_and_smoothing_budget);
    }

    CornerPathParams {
        a,
        b,
        c,
        d,
        p,
        corner_radius,
        arc_section_length,
    }
}

pub struct SVGPathInput<'a> {
    pub width: f64,
    pub height: f64,
    pub top_right_path_params: &'a CornerPathParams,
    pub bottom_right_path_params: &'a CornerPathParams,
    pub bottom_left_path_params: &'a CornerPathParams,
    pub top_left_path_params: &'a CornerPathParams,
}

fn draw_top_right_path(path_params: &CornerPathParams) -> String {
    let CornerPathParams {
        corner_radius,
        a,
        b,
        c,
        d,
        p,
        arc_section_length,
    } = path_params;

    if *corner_radius > 0.0 {
        format!(
            "c {:.4} 0 {:.4} 0 {:.4} {:.4} \
             a {:.4} {:.4} 0 0 1 {:.4} {:.4} \
             c {:.4} {:.4} \
               {:.4} {:.4} \
               {:.4} {:.4}",
            a,
            a + b,
            a + b + c,
            d,
            corner_radius,
            corner_radius,
            arc_section_length,
            arc_section_length,
            d,
            c,
            d,
            b + c,
            d,
            a + b + c
        )
    } else {
        format!("l {:.4} 0", p)
    }
}

fn draw_bottom_right_path(path_params: &CornerPathParams) -> String {
    let CornerPathParams {
        corner_radius,
        a,
        b,
        c,
        d,
        p,
        arc_section_length,
    } = path_params;

    if *corner_radius > 0.0 {
        format!(
            "c 0 {:.4} \
               0 {:.4} \
               {:.4} {:.4} \
             a {:.4} {:.4} 0 0 1 {:.4} {:.4} \
             c {:.4} {:.4} \
               {:.4} {:.4} \
               {:.4} {:.4}",
            a,
            a + b,
            -d,
            a + b + c,
            corner_radius,
            corner_radius,
            -arc_section_length,
            arc_section_length,
            -c,
            d,
            -(b + c),
            d,
            -(a + b + c),
            d
        )
    } else {
        format!("l 0 {:.4}", p)
    }
}

fn draw_bottom_left_path(path_params: &CornerPathParams) -> String {
    let CornerPathParams {
        corner_radius,
        a,
        b,
        c,
        d,
        p,
        arc_section_length,
    } = path_params;

    if *corner_radius > 0.0 {
        format!(
            "c {:.4} 0 \
               {:.4} 0 \
               {:.4} {:.4} \
             a {:.4} {:.4} 0 0 1 {:.4} {:.4} \
             c {:.4} {:.4} \
               {:.4} {:.4} \
               {:.4} {:.4}",
            -a,
            -(a + b),
            -(a + b + c),
            -d,
            corner_radius,
            corner_radius,
            -arc_section_length,
            -arc_section_length,
            -d,
            -c,
            -d,
            -(b + c),
            -d,
            -(a + b + c),
        )
    } else {
        format!("l {:.4} 0", -p)
    }
}

fn draw_top_left_path(path_params: &CornerPathParams) -> String {
    let CornerPathParams {
        corner_radius,
        a,
        b,
        c,
        d,
        p,
        arc_section_length,
    } = path_params;

    if *corner_radius > 0.0 {
        format!(
            "c 0 {:.4} \
               0 {:.4} \
               {:.4} {:.4} \
             a {:.4} {:.4} 0 0 1 {:.4} {:.4} \
             c {:.4} {:.4} \
               {:.4} {:.4} \
               {:.4} {:.4}",
            -a,
            -(a + b),
            d,
            -(a + b + c),
            corner_radius,
            corner_radius,
            arc_section_length,
            -arc_section_length,
            c,
            -d,
            b + c,
            -d,
            a + b + c,
            -d
        )
    } else {
        format!("l 0 {:.4}", -p)
    }
}

pub fn get_svg_path_from_path_params(input: &SVGPathInput) -> String {
    let SVGPathInput {
        width,
        height,
        top_right_path_params,
        bottom_right_path_params,
        bottom_left_path_params,
        top_left_path_params,
    } = input;

    format!(
        "M {:.4} 0 \
           {} \
         L {:.4} {:.4} \
           {} \
         L {:.4} {:.4} \
           {} \
         L 0 {:.4} \
           {} \
         Z",
        width - top_right_path_params.p,
        draw_top_right_path(top_right_path_params),
        width,
        height - bottom_right_path_params.p,
        draw_bottom_right_path(bottom_right_path_params),
        bottom_left_path_params.p,
        height,
        draw_bottom_left_path(bottom_left_path_params),
        top_left_path_params.p,
        draw_top_left_path(top_left_path_params),
    )
}
