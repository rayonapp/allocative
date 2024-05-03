#![cfg(feature = "arrayvec")]

use arrayvec::{ArrayString, ArrayVec};

use crate::allocative_trait::Allocative;
use crate::key::Key;
use crate::visitor::Visitor;

impl<T, const CAP: usize> Allocative for ArrayVec<T, CAP>
where
    T: Allocative,
{
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        let mut visitor = visitor.enter_self_sized::<Self>();
        for item in self {
            visitor.visit_field(Key::new("data"), item);
        }
        visitor.exit();
    }
}

impl<const CAP: usize> Allocative for ArrayString<CAP> {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        visitor.visit_simple_sized::<Self>();
    }
}
