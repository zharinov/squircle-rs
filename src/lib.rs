#![allow(unused_macros, unused_variables)]

mod distribute;
mod draw;

use crate::{
    distribute::{NormalizedCorners, RoundedRectangle},
    draw::SVGPathInput,
};
use draw::CornerParams;

pub struct SquircleParams {
    pub width: f64,
    pub height: f64,
    pub corner_smoothing: f64,
    pub corner_radius: Option<f64>,
    pub top_left_corner_radius: Option<f64>,
    pub top_right_corner_radius: Option<f64>,
    pub bottom_right_corner_radius: Option<f64>,
    pub bottom_left_corner_radius: Option<f64>,
    pub preserve_smoothing: Option<bool>,
}

pub fn get_svg_path(params: &SquircleParams) -> String {
    let SquircleParams {
        width,
        height,
        corner_smoothing,
        corner_radius,
        top_left_corner_radius,
        top_right_corner_radius,
        bottom_right_corner_radius,
        bottom_left_corner_radius,
        preserve_smoothing,
    } = params;
    let width = *width;
    let height = *height;
    let corner_smoothing = *corner_smoothing;

    let corner_radius = corner_radius.unwrap_or(0.0);
    let top_left_corner_radius = top_left_corner_radius.unwrap_or(corner_radius);
    let top_right_corner_radius = top_right_corner_radius.unwrap_or(corner_radius);
    let bottom_left_corner_radius = bottom_left_corner_radius.unwrap_or(corner_radius);
    let bottom_right_corner_radius = bottom_right_corner_radius.unwrap_or(corner_radius);
    let preserve_smoothing = preserve_smoothing.unwrap_or(false);

    if top_left_corner_radius == top_right_corner_radius
        && top_right_corner_radius == bottom_right_corner_radius
        && bottom_right_corner_radius == bottom_left_corner_radius
        && bottom_left_corner_radius == top_left_corner_radius
    {
        let rounding_and_smoothing_budget = f64::min(width, height) / 2.0;
        let corner_radius = f64::min(top_left_corner_radius, rounding_and_smoothing_budget);

        let path_params = draw::get_path_params_for_corner(CornerParams {
            corner_radius,
            corner_smoothing,
            preserve_smoothing,
            rounding_and_smoothing_budget,
        });

        return draw::get_svg_path_from_path_params(&SVGPathInput {
            width,
            height,
            top_left_path_params: &path_params,
            top_right_path_params: &path_params,
            bottom_left_path_params: &path_params,
            bottom_right_path_params: &path_params,
        });
    }

    let NormalizedCorners {
        top_left,
        top_right,
        bottom_left,
        bottom_right,
    } = distribute::distribute_and_normalize(RoundedRectangle {
        top_left_corner_radius,
        top_right_corner_radius,
        bottom_right_corner_radius,
        bottom_left_corner_radius,
        width,
        height,
    });

    let result = draw::get_svg_path_from_path_params(&SVGPathInput {
        width,
        height,
        top_left_path_params: &draw::get_path_params_for_corner(CornerParams {
            corner_radius: top_left.radius,
            corner_smoothing,
            preserve_smoothing,
            rounding_and_smoothing_budget: top_left.rounding_and_smoothing_budget,
        }),
        top_right_path_params: &draw::get_path_params_for_corner(CornerParams {
            corner_radius: top_right.radius,
            corner_smoothing,
            preserve_smoothing,
            rounding_and_smoothing_budget: top_right.rounding_and_smoothing_budget,
        }),
        bottom_right_path_params: &draw::get_path_params_for_corner(CornerParams {
            corner_radius: bottom_right.radius,
            corner_smoothing,
            preserve_smoothing,
            rounding_and_smoothing_budget: bottom_right.rounding_and_smoothing_budget,
        }),
        bottom_left_path_params: &draw::get_path_params_for_corner(CornerParams {
            corner_radius: bottom_left.radius,
            corner_smoothing,
            preserve_smoothing,
            rounding_and_smoothing_budget: bottom_left.rounding_and_smoothing_budget,
        }),
    });

    result
}
