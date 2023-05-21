pub struct RoundedRectangle {
    pub top_left_corner_radius: f64,
    pub top_right_corner_radius: f64,
    pub bottom_right_corner_radius: f64,
    pub bottom_left_corner_radius: f64,
    pub width: f64,
    pub height: f64,
}

pub struct NormalizedCorner {
    pub radius: f64,
    pub rounding_and_smoothing_budget: f64,
}

pub struct NormalizedCorners {
    pub top_left: NormalizedCorner,
    pub top_right: NormalizedCorner,
    pub bottom_left: NormalizedCorner,
    pub bottom_right: NormalizedCorner,
}

pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub enum Side {
    Top,
    Left,
    Right,
    Bottom,
}

pub struct Adjacent {
    corner: Corner,
    side: Side,
}

fn get_adjacents(corner: &Corner) -> (Adjacent, Adjacent) {
    match corner {
        Corner::TopLeft => (
            Adjacent {
                corner: Corner::TopRight,
                side: Side::Top,
            },
            Adjacent {
                corner: Corner::BottomLeft,
                side: Side::Left,
            },
        ),
        Corner::TopRight => (
            Adjacent {
                corner: Corner::TopLeft,
                side: Side::Top,
            },
            Adjacent {
                corner: Corner::BottomRight,
                side: Side::Right,
            },
        ),
        Corner::BottomLeft => (
            Adjacent {
                corner: Corner::BottomRight,
                side: Side::Bottom,
            },
            Adjacent {
                corner: Corner::TopLeft,
                side: Side::Left,
            },
        ),
        Corner::BottomRight => (
            Adjacent {
                corner: Corner::BottomLeft,
                side: Side::Bottom,
            },
            Adjacent {
                corner: Corner::TopRight,
                side: Side::Right,
            },
        ),
    }
}

struct Budget {
    top_left: f64,
    top_right: f64,
    bottom_left: f64,
    bottom_right: f64,
}

impl Budget {
    fn new() -> Self {
        Self {
            top_left: -1.0,
            top_right: -1.0,
            bottom_left: -1.0,
            bottom_right: -1.0,
        }
    }

    fn get(&self, corner: &Corner) -> f64 {
        match corner {
            Corner::TopLeft => self.top_left,
            Corner::TopRight => self.top_right,
            Corner::BottomLeft => self.bottom_left,
            Corner::BottomRight => self.bottom_right,
        }
    }

    fn set(&mut self, corner: &Corner, value: f64) {
        match corner {
            Corner::TopLeft => self.top_left = value,
            Corner::TopRight => self.top_right = value,
            Corner::BottomLeft => self.bottom_left = value,
            Corner::BottomRight => self.bottom_right = value,
        }
    }
}

struct CornerRadiusMap {
    top_left: f64,
    top_right: f64,
    bottom_left: f64,
    bottom_right: f64,
}

impl CornerRadiusMap {
    fn new(rectangle: &RoundedRectangle) -> Self {
        Self {
            top_left: rectangle.top_left_corner_radius,
            top_right: rectangle.top_right_corner_radius,
            bottom_left: rectangle.bottom_left_corner_radius,
            bottom_right: rectangle.bottom_right_corner_radius,
        }
    }

    fn get(&self, corner: &Corner) -> f64 {
        match corner {
            Corner::TopLeft => self.top_left,
            Corner::TopRight => self.top_right,
            Corner::BottomLeft => self.bottom_left,
            Corner::BottomRight => self.bottom_right,
        }
    }

    fn set(&mut self, corner: &Corner, value: f64) {
        match corner {
            Corner::TopLeft => self.top_left = value,
            Corner::TopRight => self.top_right = value,
            Corner::BottomLeft => self.bottom_left = value,
            Corner::BottomRight => self.bottom_right = value,
        }
    }

    fn get_items(self: &Self) -> Vec<(Corner, f64)> {
        let mut items = vec![
            (Corner::TopLeft, self.top_left),
            (Corner::TopRight, self.top_right),
            (Corner::BottomLeft, self.bottom_left),
            (Corner::BottomRight, self.bottom_right),
        ];
        items.sort_by(|(_, radius1), (_, radius2)| radius2.partial_cmp(radius1).unwrap());
        items
    }
}

pub fn distribute_and_normalize(rectangle: RoundedRectangle) -> NormalizedCorners {
    let mut rounding_and_smoothing_budget_map = Budget::new();
    let mut corner_radius_map = CornerRadiusMap::new(&rectangle);

    for (corner, radius) in corner_radius_map.get_items() {
        let calc_budget = |adjacent: &Adjacent| -> f64 {
            let corner_radius = corner_radius_map.get(&adjacent.corner);
            if radius == 0.0 && corner_radius == 0.0 {
                return 0.0;
            }

            let adjacent_corner_budget = rounding_and_smoothing_budget_map.get(&adjacent.corner);

            let side_length = match adjacent.side {
                Side::Top | Side::Bottom => rectangle.width,
                Side::Left | Side::Right => rectangle.height,
            };

            // If the adjacent corner's already been given the rounding and smoothing budget,
            // we'll just take the rest
            if adjacent_corner_budget >= 0.0 {
                return side_length - adjacent_corner_budget;
            } else {
                return (radius / (radius + corner_radius)) * side_length;
            }
        };

        let (adjacent1, adjacent2) = get_adjacents(&corner);
        let budget1 = calc_budget(&adjacent1);
        let budget2 = calc_budget(&adjacent2);
        let budget = f64::min(budget1, budget2);

        rounding_and_smoothing_budget_map.set(&corner, budget);
        corner_radius_map.set(&corner, f64::min(radius, budget));
    }

    NormalizedCorners {
        top_left: NormalizedCorner {
            radius: corner_radius_map.top_left,
            rounding_and_smoothing_budget: rounding_and_smoothing_budget_map.top_left,
        },
        top_right: NormalizedCorner {
            radius: corner_radius_map.top_right,
            rounding_and_smoothing_budget: rounding_and_smoothing_budget_map.top_right,
        },
        bottom_left: NormalizedCorner {
            radius: corner_radius_map.bottom_left,
            rounding_and_smoothing_budget: rounding_and_smoothing_budget_map.bottom_left,
        },
        bottom_right: NormalizedCorner {
            radius: corner_radius_map.bottom_right,
            rounding_and_smoothing_budget: rounding_and_smoothing_budget_map.bottom_right,
        },
    }
}
