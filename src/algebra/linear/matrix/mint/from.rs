use crate::algebra::linear::Matrix;
use crate::algebra::abstr::Real;


macro_rules! impl_from_mint(
    ($(($rows: literal, $columns: literal) => $MV: ident, $m: ident, $ret: expr);* $(;)*) => {$(
 
        impl<T> From<mint::$MV<T>> for Matrix<T>
            where T: Real 
        {
            fn from(m_mint: mint::$MV<T>) -> Self 
            {
                let $m = m_mint;
                 
                $ret
            }
        }
    )*}
);

impl_from_mint!(
    (2, 2) => ColumnMatrix2, m,
    matrix![  
        m.x.x, m.y.x;
        m.x.y, m.y.y                  
    ];

    (2, 3) => ColumnMatrix2x3, m,
    matrix![  
        m.x.x, m.y.x, m.z.x;
        m.x.y, m.y.y, m.z.y
    ];

    (2, 4) => ColumnMatrix2x4, m,
    matrix![  
        m.x.x, m.y.x, m.z.x, m.w.x;
        m.x.y, m.y.y, m.z.y, m.w.y
    ];

    (3, 2) => ColumnMatrix3x2, m,
    matrix![  
        m.x.x, m.y.x;
        m.x.y, m.y.y;
        m.x.z, m.y.z  
    ];

    (3, 3) => ColumnMatrix3, m,
    matrix![  
        m.x.x, m.y.x, m.z.x;
        m.x.y, m.y.y, m.z.y;
        m.x.z, m.y.z, m.z.z
    ];

    (3, 4) => ColumnMatrix3x4, m,
    matrix![  
        m.x.x, m.y.x, m.z.x, m.w.x;
        m.x.y, m.y.y, m.z.y, m.w.y;
        m.x.z, m.y.z, m.z.z, m.w.z
    ];

    (4, 2) => ColumnMatrix4x2, m,
    matrix![  
        m.x.x, m.y.x;
        m.x.y, m.y.y;
        m.x.z, m.y.z;
        m.x.w, m.y.w
    ];

    (4, 3) => ColumnMatrix4x3, m,
    matrix![  
        m.x.x, m.y.x, m.z.x;
        m.x.y, m.y.y, m.z.y;
        m.x.z, m.y.z, m.z.z;
        m.x.w, m.y.w, m.z.w
    ];

    (4, 4) => ColumnMatrix4, m,
    matrix![  
        m.x.x, m.y.x, m.z.x, m.w.x;
        m.x.y, m.y.y, m.z.y, m.w.y;
        m.x.z, m.y.z, m.z.z, m.w.z;
        m.x.w, m.y.w, m.z.w, m.w.w
    ];

    (2, 2) => RowMatrix2, m,
    matrix![  
        m.x.x, m.x.y;
        m.y.x, m.y.y                 
    ];

    (2, 3) => RowMatrix2x3, m,
    matrix![  
        m.x.x, m.x.y, m.x.z;
        m.y.x, m.y.y, m.y.z                  
    ];

    (2, 4) => RowMatrix2x4, m,
    matrix![  
        m.x.x, m.x.y, m.x.z, m.x.w;
        m.y.x, m.y.y, m.y.z, m.y.w       
    ];

    (3, 2) => RowMatrix3x2, m,
    matrix![  
        m.x.x, m.x.y;
        m.y.x, m.y.y;
        m.z.x, m.z.y                
    ];

    (3, 3) => RowMatrix3, m,
    matrix![ 
        m.x.x, m.x.y, m.x.z;
        m.y.x, m.y.y, m.y.z;
        m.z.x, m.z.y, m.z.z                 
    ];

    (3, 4) => RowMatrix3x4, m,
    matrix![  
        m.x.x, m.x.y, m.x.z, m.x.w;
        m.y.x, m.y.y, m.y.z, m.y.w;
        m.z.x, m.z.y, m.z.z, m.z.w                   
    ];

    (4, 2) => RowMatrix4x2, m,
    matrix![  
        m.x.x, m.x.y;
        m.y.x, m.y.y;
        m.z.x, m.z.y;
        m.w.x, m.w.y
    ];

    (4, 3) => RowMatrix4x3, m,
    matrix![  
        m.x.x, m.x.y, m.x.z;
        m.y.x, m.y.y, m.y.z;
        m.z.x, m.z.y, m.z.z;
        m.w.x, m.w.y, m.w.z
    ];

    (4, 4) => RowMatrix4, m,
    matrix![  
        m.x.x, m.x.y, m.x.z, m.x.w;
        m.y.x, m.y.y, m.y.z, m.y.w;
        m.z.x, m.z.y, m.z.z, m.z.w;
        m.w.x, m.w.y, m.w.z, m.w.w 
    ];
);