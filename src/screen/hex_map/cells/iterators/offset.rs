use crate::screen::hex_map::cells::HexId;

pub struct Offset<T: Iterator<Item = HexId>> {
    iter: T,
    offset: HexId,
}

impl<T: Iterator<Item = HexId>> Iterator for Offset<T> {
    type Item = HexId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|next| Some(next + self.offset))
    }
}

pub trait WithOffset: Iterator<Item = HexId> {
    fn with_offset(self, offset: HexId) -> Offset<impl Iterator<Item = HexId>>;
}

impl<T: Iterator<Item = HexId>> WithOffset for T {
    fn with_offset(self, offset: HexId) -> Offset<impl Iterator<Item = HexId>> {
        Offset { iter: self, offset }
    }
}
