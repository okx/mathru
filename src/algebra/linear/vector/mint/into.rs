use crate::algebra::linear::Vector;
use crate::algebra::abstr::Real;
use mint::{Vector2, Vector3, Vector4};


macro_rules! impl_into_mint(
    ($($rows: literal => $MV: ident, $data: ident, $ret: expr);* $(;)*) => {$(
 
        impl<T> Into<$MV<T>> for Vector<T>
            where T: Real 
        {
            fn into(self) -> $MV<T> {
                let (m, n) = self.dim();
                let $data = self.data;
                println!("{}, {}", m, n);
                if (m != 1 && n != $rows) && (m != $rows && n != 1) {
                    panic!("Vector can not be converted into a $MV because it is not a 1 by {} or {} by 1 vector", $rows, $rows)
                }
                 
                $ret
            }
        }
    )*}
);

impl_into_mint!(
    2 => Vector2, data, Vector2{ x: data[[0, 0]], y: data[[1, 0]]};
    3 => Vector3, data, Vector3{ x: data[[0, 0]], y: data[[1, 0]], z: data[[2, 0]]};
    4 => Vector4, data, Vector4{ x: data[[0, 0]], y: data[[1, 0]], z: data[[2, 0]], w: data[[3, 0]]};
);