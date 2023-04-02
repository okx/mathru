use crate::algebra::linear::Vector;
use crate::algebra::abstr::Real;


macro_rules! impl_from_mint_v(
    ($($rows: literal => $MV: ident, $v: ident, $ret: expr);* $(;)*) => {$(
 
        impl<T> From<mint::$MV<T>> for Vector<T>
            where T: Real 
        {
            fn from(v_mint: mint::$MV<T>) -> Self 
            {
                let $v = v_mint;
                 
                $ret
            }
        }
    )*}
);

impl_from_mint_v!(
    2 => Vector2, v, Vector::new_column(vec![v.x, v.y]);
    3 => Vector3, v, Vector::new_column(vec![v.x, v.y, v.z]);
    4 => Vector4, v, Vector::new_column(vec![v.x, v.y, v.z, v.w]);
);