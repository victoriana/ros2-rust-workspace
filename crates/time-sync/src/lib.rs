//! Multi-stream timestamp alignment against a central gateway clock.
//! Generic: any N nodes, no ROS or hardware coupling.

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Timestamped<T> {
    pub node_id: u8,
    pub device_ts_us: u64,
    pub gateway_ts_us: u64,
    pub payload: T,
}

pub struct StreamAligner<T> {
    window: usize,
    buffers: Vec<VecDeque<Timestamped<T>>>,
}

impl<T> StreamAligner<T> {
    pub fn new(node_count: usize, window: usize) -> Self {
        Self { window, buffers: (0..node_count).map(|_| VecDeque::new()).collect() }
    }

    /// Aligned batch extraction implemented in Week 5 of the plan.
    pub fn push(&mut self, item: Timestamped<T>) -> Result<(), String> {
        let idx = item.node_id as usize;
        let buf = self.buffers.get_mut(idx).ok_or("unknown node_id")?;
        if buf.len() >= self.window {
            buf.pop_front();
        }
        buf.push_back(item);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_node() {
        let mut a = StreamAligner::<u8>::new(2, 4);
        let bad = Timestamped { node_id: 9, device_ts_us: 0, gateway_ts_us: 0, payload: 1 };
        assert!(a.push(bad).is_err());
    }
}
