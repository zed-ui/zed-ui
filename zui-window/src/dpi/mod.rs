//! Functionality around dots-per-inch (DPI).

use std::ops;

mod size;
mod position;

pub use size::*;
pub use position::*;

macro_rules! impl_ops {
    ($($t:ty)+ => $op:ident $op_f:ident $op_assign:ident $op_assign_f:ident) => {
        $(
            impl ops::$op<f64> for $t {
                type Output = Self;

                fn $op_f(self, other: f64) -> Self {
                    self.map(|v| v.$op_f(other))
                }
            }

            impl ops::$op_assign<f64> for $t {
                fn $op_assign_f(&mut self, other: f64) {
                    self.apply(|v| v.$op_assign_f(other));
                }
            }
        )+
    };
}

impl_ops! {
    PhysicalSize LogicalSize PhysicalPosition LogicalPosition => Add add AddAssign add_assign
}

impl_ops! {
    PhysicalSize LogicalSize PhysicalPosition LogicalPosition => Sub sub SubAssign sub_assign
}

impl_ops! {
    PhysicalSize LogicalSize PhysicalPosition LogicalPosition => Mul mul MulAssign mul_assign
}

impl_ops! {
    PhysicalSize LogicalSize PhysicalPosition LogicalPosition => Div div DivAssign div_assign
}
