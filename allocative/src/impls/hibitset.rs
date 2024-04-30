#![cfg(feature = "hibitset")]

use hibitset::BitSet;

use crate::allocative_trait::Allocative;
use crate::visitor::Visitor;

impl Allocative for BitSet {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        let mut visitor = visitor.enter_self_sized::<Self>();
        // TODO: Spare capacity.
        visitor.visit_slice(self.layer0_as_slice());
        visitor.visit_slice(self.layer1_as_slice());
        visitor.visit_slice(self.layer2_as_slice());
        visitor.exit();
    }
}
