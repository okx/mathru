use crate::algebra::linear::Matrix;
use crate::algebra::abstr::Real;
use mint::{ ColumnMatrix2, ColumnMatrix2x3 ,ColumnMatrix2x4, ColumnMatrix3x2, ColumnMatrix3, ColumnMatrix3x4, 
    ColumnMatrix4x2, ColumnMatrix4x3, ColumnMatrix4,
    RowMatrix2, RowMatrix2x3 ,RowMatrix2x4, RowMatrix3x2, RowMatrix3, RowMatrix3x4, 
    RowMatrix4x2, RowMatrix4x3, RowMatrix4
};

use mint::{Vector2, Vector3, Vector4};


macro_rules! impl_into_mint(
    ($(($rows: literal, $columns: literal) => $MV: ident, $data: ident, $ret: expr);* $(;)*) => {$(
 
        impl<T> Into<$MV<T>> for Matrix<T>
            where T: Real 
        {
            fn into(self) -> $MV<T> {
                let (m, n) = self.dim();
                let $data = self.data;
                if m != $rows || n != $columns {
                    panic!("Matrix can not be converted into a $MV because it is not a {} by {} matrix", $rows, $columns)
                }
                 
                $ret
            }
        }
    )*}
);

impl_into_mint!(
    (2, 2) => ColumnMatrix2, data,
    ColumnMatrix2{  
        x: Vector2{x: data[0], y: data[1]},
        y: Vector2{x: data[2], y: data[3]}                    
    };

    (2, 3) => ColumnMatrix2x3, data,
    ColumnMatrix2x3{  
        x: Vector2{x: data[0], y: data[1]},
        y: Vector2{x: data[2], y: data[3]},     
        z: Vector2{x: data[4], y: data[5]},     
    };

    (2, 4) => ColumnMatrix2x4, data,
    ColumnMatrix2x4{  
        x: Vector2{x: data[0], y: data[1]},
        y: Vector2{x: data[2], y: data[3]},     
        z: Vector2{x: data[4], y: data[5]},     
        w: Vector2{x: data[6], y: data[7]},
    };

    (3, 2) => ColumnMatrix3x2, data,
    ColumnMatrix3x2{  
        x: Vector3{x: data[0], y: data[1], z: data[2]},
        y: Vector3{x: data[3], y: data[4], z: data[5]},     
    };

    (3, 3) => ColumnMatrix3, data,
    ColumnMatrix3{  
        x: Vector3{x: data[0], y: data[1], z: data[2]},
        y: Vector3{x: data[3], y: data[4], z: data[5]},     
        z: Vector3{x: data[6], y: data[7], z: data[8]},     
    };

    (3, 4) => ColumnMatrix3x4, data,
    ColumnMatrix3x4{  
        x: Vector3{x: data[0], y: data[1], z: data[2]},
        y: Vector3{x: data[3], y: data[4], z: data[5]},     
        z: Vector3{x: data[6], y: data[7], z: data[8]},     
        w: Vector3{x: data[9], y: data[10], z: data[11]},
    };

    (4, 2) => ColumnMatrix4x2, data,
    ColumnMatrix4x2{  
        x: Vector4{x: data[0], y: data[1], z: data[2], w: data[3]},
        y: Vector4{x: data[4], y: data[5], z: data[6], w: data[7]},     
    };

    (4, 3) => ColumnMatrix4x3, data,
    ColumnMatrix4x3{  
        x: Vector4{x: data[0], y: data[1], z: data[2], w: data[3]},
        y: Vector4{x: data[4], y: data[5], z: data[6], w: data[7]},     
        z: Vector4{x: data[8], y: data[9], z: data[10], w: data[11]},     
    };

    (4, 4) => ColumnMatrix4, data,
    ColumnMatrix4{  
        x: Vector4{x: data[0], y: data[1], z: data[2], w: data[3]},
        y: Vector4{x: data[4], y: data[5], z: data[6], w: data[7]},     
        z: Vector4{x: data[8], y: data[9], z: data[10], w: data[11]},     
        w: Vector4{x: data[12], y: data[13], z: data[14], w: data[15]},
    };

    (2, 2) => RowMatrix2, data,
    RowMatrix2{  
        x: Vector2{x: data[0], y: data[2]},
        y: Vector2{x: data[1], y: data[3]}                    
    };

    (2, 3) => RowMatrix2x3, data,
    RowMatrix2x3{  
        x: Vector3{x: data[0], y: data[2], z: data[4]},
        y: Vector3{x: data[1], y: data[3], z: data[5]}                    
    };

    (2, 4) => RowMatrix2x4, data,
    RowMatrix2x4{  
        x: Vector4{x: data[0], y: data[2], z: data[4], w: data[6]},
        y: Vector4{x: data[1], y: data[3], z: data[5], w: data[7]}                    
    };

    (3, 2) => RowMatrix3x2, data,
    RowMatrix3x2{  
        x: Vector2{x: data[0], y: data[3]},
        y: Vector2{x: data[1], y: data[4]},
        z: Vector2{x: data[2], y: data[5]}                   
    };

    (3, 3) => RowMatrix3, data,
    RowMatrix3{  
        x: Vector3{x: data[0], y: data[3], z: data[6]},
        y: Vector3{x: data[1], y: data[4], z: data[7]},
        z: Vector3{x: data[2], y: data[5], z: data[8]}                   
    };

    (3, 4) => RowMatrix3x4, data,
    RowMatrix3x4{  
        x: Vector4{x: data[0], y: data[3], z: data[6], w: data[9]},
        y: Vector4{x: data[1], y: data[4], z: data[7], w: data[10]},
        z: Vector4{x: data[2], y: data[5], z: data[8], w: data[11]}                   
    };

    (4, 2) => RowMatrix4x2, data,
    RowMatrix4x2{  
        x: Vector2{x: data[0], y: data[4]},
        y: Vector2{x: data[1], y: data[5]},
        z: Vector2{x: data[2], y: data[6]},    
        w: Vector2{x: data[3], y: data[7]}  
    };


    (4, 3) => RowMatrix4x3, data,
    RowMatrix4x3{  
        x: Vector3{x: data[0], y: data[4], z: data[8]},
        y: Vector3{x: data[1], y: data[5], z: data[9]},
        z: Vector3{x: data[2], y: data[6], z: data[10]},    
        w: Vector3{x: data[3], y: data[7], z: data[11]}  
    };

    (4, 4) => RowMatrix4, data,
    RowMatrix4{  
        x: Vector4{x: data[0], y: data[4], z: data[8], w: data[12]},
        y: Vector4{x: data[1], y: data[5], z: data[9], w: data[13]},
        z: Vector4{x: data[2], y: data[6], z: data[10], w: data[14]},    
        w: Vector4{x: data[3], y: data[7], z: data[11], w: data[15]}  
    };
);