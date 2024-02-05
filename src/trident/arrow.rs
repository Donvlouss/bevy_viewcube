#[derive(Debug, Copy, Clone)]
pub struct BevyTridentCone {
    pub radius: f32,
    pub height: f32,
    pub subdivisions: usize,
}

impl Default for BevyTridentCone {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 1.0,
            subdivisions: 8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BevyTridentArrow {
    pub cone: BevyTridentCone,
    pub tail_radius: f32,
    pub tail_length: f32,
}

impl Default for BevyTridentArrow {
    fn default() -> Self {
        Self::new(1.0, 0.04, 8)
    }
}

impl BevyTridentArrow {
    pub const TRIDENT_ARROW_10: BevyTridentArrow = BevyTridentArrow {
        cone: BevyTridentCone {
            radius: 0.4,
            height: 2.0,
            subdivisions: 8,
        },
        tail_length: 8.0,
        tail_radius: 0.2,
    };

    pub const TRIDENT_ARROW_100: BevyTridentArrow = BevyTridentArrow {
        cone: BevyTridentCone {
            radius: 4.0,
            height: 20.0,
            subdivisions: 8,
        },
        tail_length: 80.0,
        tail_radius: 1.0,
    };

    pub fn new(length: f32, radius: f32, subdivisions: usize) -> Self {
        BevyTridentArrow::new_with_detail(length, radius, subdivisions, 0.2, 2.0)
    }

    pub fn new_with_detail(
        length: f32, radius: f32, subdivisions: usize,
        cone_length_ratio: f32, cone_radius_ratio: f32
    ) -> Self {
        let subdivisions = if subdivisions < 3 { 3 } else { subdivisions };
        let cone_length = length * cone_length_ratio;
        let cone_radius = radius * cone_radius_ratio;

        Self {
            cone: BevyTridentCone { radius: cone_radius, height: cone_length, subdivisions },
            tail_radius: radius,
            tail_length: length - cone_length,
        }
    }
}