use std::ops::Range;

use crate::Allocative;
use crate::Visitor;

impl<T: Allocative> Allocative for Range<T> {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        self.start.visit(visitor);
        self.end.visit(visitor);
    }
}
