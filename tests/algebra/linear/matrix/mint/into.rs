use mathru::algebra::linear::Matrix;
use mint::{ ColumnMatrix2, ColumnMatrix2x3 ,ColumnMatrix2x4, ColumnMatrix3x2, ColumnMatrix3, ColumnMatrix3x4, 
    ColumnMatrix4x2, ColumnMatrix4x3, ColumnMatrix4,
    RowMatrix2, RowMatrix2x3 ,RowMatrix2x4, RowMatrix3x2, RowMatrix3, RowMatrix3x4, 
    RowMatrix4x2, RowMatrix4x3, RowMatrix4
};
use mint::{Vector2, Vector3, Vector4};

#[test]
fn into_column_matrix_2()
{
    let m: Matrix<f64> = matrix![0.0, 1.0; 2.0, 3.0];

    let m_mint_ref =  ColumnMatrix2{
        x: Vector2{x: 0.0, y: 2.0},
        y: Vector2{x: 1.0, y: 3.0}
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_column_matrix_2x3()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0; 
                                    4.0, 5.0, 6.0];

    let m_mint_ref = ColumnMatrix2x3{
        x: Vector2{x: 1.0, y: 4.0},
        y: Vector2{x: 2.0, y: 5.0},
        z: Vector2{x: 3.0, y: 6.0}
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_column_matrix_2x4()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0, 4.0; 
                                    5.0, 6.0, 7.0, 8.0];

    let m_mint_ref = ColumnMatrix2x4{
        x: Vector2{x: 1.0, y: 5.0},
        y: Vector2{x: 2.0, y: 6.0},
        z: Vector2{x: 3.0, y: 7.0},
        w: Vector2{x: 4.0, y: 8.0}
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_column_matrix_3x2()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0; 
                                    3.0, 4.0;
                                    5.0, 6.0];

    let m_mint_ref = ColumnMatrix3x2{
        x: Vector3{x: 1.0, y: 3.0, z: 5.0},
        y: Vector3{x: 2.0, y: 4.0, z: 6.0},
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_column_matrix_3()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0; 
                                    4.0, 5.0, 6.0;
                                    7.0, 8.0, 9.0];

    let m_mint_ref = ColumnMatrix3{
        x: Vector3{x: 1.0, y: 4.0, z: 7.0},
        y: Vector3{x: 2.0, y: 5.0, z: 8.0},
        z: Vector3{x: 3.0, y: 6.0, z: 9.0},
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_column_matrix_3x4()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0, 4.0; 
                                    5.0, 6.0, 7.0, 8.0;
                                    9.0, 10.0, 11.0, 12.0];

    let m_mint_ref = ColumnMatrix3x4{
        x: Vector3{x: 1.0, y: 5.0, z: 9.0},
        y: Vector3{x: 2.0, y: 6.0, z: 10.0},
        z: Vector3{x: 3.0, y: 7.0, z: 11.0},
        w: Vector3{x: 4.0, y: 8.0, z: 12.0}
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_column_matrix_4x2()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0; 
                                    3.0, 5.0;
                                    6.0, 7.0;
                                    8.0, 9.0];

    let m_mint_ref = ColumnMatrix4x2{
        x: Vector4{x: 1.0, y: 3.0, z: 6.0, w: 8.0},
        y: Vector4{x: 2.0, y: 5.0, z: 7.0, w: 9.0},
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_column_matrix_4x3()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0; 
                                    4.0, 5.0, 6.0;
                                    7.0, 8.0, 9.0;
                                    10.0, 11.0, 12.0];

    let m_mint_ref = ColumnMatrix4x3{
        x: Vector4{x: 1.0, y: 4.0, z: 7.0, w: 10.0},
        y: Vector4{x: 2.0, y: 5.0, z: 8.0, w: 11.0},
        z: Vector4{x: 3.0, y: 6.0, z: 9.0, w: 12.0},
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_column_matrix_4()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0, 4.0; 
                                    5.0, 6.0, 7.0, 8.0;
                                    9.0, 10.0, 11.0, 12.0;
                                    13.0, 14.0, 15.0, 16.0];

    let m_mint_ref = ColumnMatrix4{
        x: Vector4{x: 1.0, y: 5.0, z: 9.0, w: 13.0},
        y: Vector4{x: 2.0, y: 6.0, z: 10.0, w: 14.0},
        z: Vector4{x: 3.0, y: 7.0, z: 11.0, w: 15.0},
        w: Vector4{x: 4.0, y: 8.0, z: 12.0, w: 16.0}
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_row_matrix_2()
{
    let m: Matrix<f64> = matrix![   0.0, 1.0; 
                                    2.0, 3.0];

    let m_mint_ref =  RowMatrix2{
        x: Vector2{x: 0.0, y: 1.0},
        y: Vector2{x: 2.0, y: 3.0}
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_row_matrix_2x3()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0; 
                                    4.0, 5.0, 6.0];

    let m_mint_ref = RowMatrix2x3{
        x: Vector3{x: 1.0, y: 2.0, z: 3.0},
        y: Vector3{x: 4.0, y: 5.0, z: 6.0},
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_row_matrix_2x4()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0, 4.0; 
                                    5.0, 6.0, 7.0, 8.0];

    let m_mint_ref = RowMatrix2x4{
        x: Vector4{x: 1.0, y: 2.0, z: 3.0, w: 4.0 },
        y: Vector4{x: 5.0, y: 6.0, z: 7.0, w: 8.0},
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_row_matrix_3x2()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0; 
                                    3.0, 4.0;
                                    5.0, 6.0];

    let m_mint_ref = RowMatrix3x2{
        x: Vector2{x: 1.0, y: 2.0},
        y: Vector2{x: 3.0, y: 4.0},
        z: Vector2{x: 5.0, y: 6.0}
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_row_matrix_3()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0; 
                                    4.0, 5.0, 6.0;
                                    7.0, 8.0, 9.0];

    let m_mint_ref = RowMatrix3{
        x: Vector3{x: 1.0, y: 2.0, z: 3.0},
        y: Vector3{x: 4.0, y: 5.0, z: 6.0},
        z: Vector3{x: 7.0, y: 8.0, z: 9.0},
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_row_matrix_3x4()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0, 4.0; 
                                    5.0, 6.0, 7.0, 8.0;
                                    9.0, 10.0, 11.0, 12.0];

    let m_mint_ref = RowMatrix3x4{
        x: Vector4{x: 1.0, y: 2.0, z: 3.0, w: 4.0},
        y: Vector4{x: 5.0, y: 6.0, z: 7.0, w: 8.0},
        z: Vector4{x: 9.0, y: 10.0, z: 11.0, w: 12.0}
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_row_matrix_4x2()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0; 
                                    3.0, 4.0;
                                    5.0, 7.0;
                                    7.0, 8.0];

    let m_mint_ref = RowMatrix4x2{
        x: Vector2{x: 1.0, y: 2.0},
        y: Vector2{x: 3.0, y: 4.0},
        z: Vector2{x: 5.0, y: 7.0},
        w: Vector2{x: 7.0, y: 8.0},
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_row_matrix_4x3()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0; 
                                    4.0, 5.0, 6.0;
                                    7.0, 8.0, 9.0;
                                    10.0, 11.0, 12.0];

    let m_mint_ref = RowMatrix4x3{
        x: Vector3{x: 1.0, y: 2.0, z: 3.0},
        y: Vector3{x: 4.0, y: 5.0, z: 6.0},
        z: Vector3{x: 7.0, y: 8.0, z: 9.0},
        w: Vector3{x: 10.0, y: 11.0, z: 12.0}
    };

    assert_eq!(m_mint_ref, m.into());
}

#[test]
fn into_row_matrix_4()
{
    let m: Matrix<f64> = matrix![   1.0, 2.0, 3.0, 4.0; 
                                    5.0, 6.0, 7.0, 8.0;
                                    9.0, 10.0, 11.0, 12.0;
                                    13.0, 14.0, 15.0, 16.0];

    let m_mint_ref = RowMatrix4{
        x: Vector4{x: 1.0, y: 2.0, z: 3.0, w: 4.0},
        y: Vector4{x: 5.0, y: 6.0, z: 7.0, w: 8.0},
        z: Vector4{x: 9.0, y: 10.0, z: 11.0, w: 12.0},
        w: Vector4{x: 13.0, y: 14.0, z: 15.0, w: 16.0}
    };

    assert_eq!(m_mint_ref, m.into());
}