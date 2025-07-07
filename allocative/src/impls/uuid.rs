use uuid::Uuid;

use crate::Allocative;
use crate::Visitor;

impl Allocative for Uuid {
    fn visit<'a, 'b: 'a>(&self, visitor: &'a mut Visitor<'b>) {
        visitor.enter_self_sized::<Self>().exit();
    }
}
