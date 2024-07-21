use crate::cells::HexId;

use super::Ranged;


pub struct FastFill {
    q: std::ops::RangeInclusive<i32>,
    current_q: i32,
    r: std::ops::RangeInclusive<i32>,
    range: i32,
}

impl FastFill {
    pub fn new(range: u32) -> FastFill {
        let range = range as i32;
        FastFill {
            q: -range + 1..=range,
            current_q: -range,
            r: 0..=range,
            range,
        }
    }
}

impl Iterator for FastFill {
    type Item = HexId;
    fn next(&mut self) -> Option<Self::Item> {
        match self.r.next() {
            None => match self.q.next() {
                Some(q) => {
                    self.current_q = q;
                    self.r = (-self.range).max(-q - self.range)..=(self.range).min(-q + self.range);
                    if let Some(r) = self.r.next() {
                        Some(HexId::new(self.current_q, r))
                    } else {
                        None
                    }
                }
                None => None,
            },
            Some(r) => Some(HexId::new(self.current_q, r)),
        }
    }

    // fn size_hint(&self) -> (usize, Option<usize>) {
    //     (0, Some(ChunkId::in_range(self.range as u32) as usize))
    // }
}

impl Ranged for FastFill {
    fn in_range(range: u32) -> Self {
        Self::new(range)
    }
}