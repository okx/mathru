use mathru::algebra::linear::Vector;
use mint::{Vector2, Vector3, Vector4};

#[test]
fn into_row_vector_2()
{
    let v: Vector<f64> = vector![0.0, 1.0];

    let v_mint_ref = Vector2{x: 0.0, y: 1.0};

    assert_eq!(v_mint_ref, v.into());
}

#[test]
fn into_row_vector_3()
{
    let v: Vector<f64> = vector![1.0, 2.0, 3.0];

    let v_mint_ref = Vector3{x: 1.0, y: 2.0, z: 3.0};
      
    assert_eq!(v_mint_ref, v.into());
}

#[test]
fn into_row_vector_4()
{
    let v: Vector<f64> = vector![   1.0, 2.0, 3.0, 4.0]; 

    let v_mint_ref = Vector4{ x: 1.0, y: 2.0, z: 3.0, w: 4.0};

    assert_eq!(v_mint_ref, v.into());
}

#[test]
fn into_column_vector_2()
{
    let v: Vector<f64> = vector![   1.0; 
                                    2.0];

    let v_mint_ref = Vector2{ x: 1.0, y: 2.0 };

    assert_eq!(v_mint_ref, v.into());
}

#[test]
fn into_column_vector_3()
{
    let v: Vector<f64> = vector![   1.0; 
                                    2.0;
                                    3.0];

    let v_mint_ref = Vector3{ x: 1.0, y: 2.0, z: 3.0};

    assert_eq!(v_mint_ref, v.into());
}

#[test]
fn into_column_vector_4()
{
    let v: Vector<f64> = vector![   1.0; 
                                    2.0;
                                    3.0;
                                    4.0];

    let v_mint_ref = Vector4{
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 4.0
    };

    assert_eq!(v_mint_ref, v.into());
}