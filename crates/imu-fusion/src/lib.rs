//! Hardware-agnostic Madgwick AHRS sensor fusion.
//! Consumes BNO085-style quaternion streams; no ROS dependency.

#[derive(Debug, Clone, Copy, Default)]
pub struct Quaternion(pub [f64; 4]); // w, x, y, z

#[derive(Debug, Clone, Copy, Default)]
pub struct ImuSample {
    pub timestamp_us: u64,
    pub accel: [f64; 3],
    pub gyro: [f64; 3],
    pub mag: Option<[f64; 3]>,
}

pub struct Madgwick {
    pub beta: f64,
    pub sample_rate_hz: f64,
    pub q: Quaternion,
}

impl Madgwick {
    pub fn new(beta: f64, sample_rate_hz: f64) -> Self {
        Self { beta, sample_rate_hz, q: Quaternion([1.0, 0.0, 0.0, 0.0]) }
    }

    /// Update step implemented in Week 4 of the plan.
    pub fn update(&mut self, _sample: &ImuSample) -> Quaternion {
        self.q
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_quaternion_is_unit() {
        let m = Madgwick::new(0.1, 400.0);
        let n = m.q.0.iter().map(|v| v * v).sum::<f64>().sqrt();
        assert!((n - 1.0).abs() < 1e-9);
    }
}
