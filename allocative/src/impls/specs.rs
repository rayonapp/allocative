#![cfg(feature = "specs")]

use specs::storage::ComponentEvent;
use specs::Entity;
use specs::ReaderId;

use crate::allocative_trait::Allocative;
use crate::visitor::Visitor;

impl Allocative for Entity {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        visitor.visit_simple_sized::<Self>();
    }
}

impl Allocative for ComponentEvent {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        visitor.visit_simple_sized::<Self>();
    }
}

impl<T: 'static> Allocative for ReaderId<T> {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        visitor.visit_simple_sized::<Self>();
    }
}
