use mathru::algebra::linear::Vector;
use mint::{Vector2, Vector3, Vector4};


#[test]
fn from_column_vector_2()
{
    let m_ref: Vector<f64> = vector![1.0; 2.0];

    let m_mint = Vector2{ x: 1.0, y: 2.0};

    assert_eq!(m_ref, Vector::from(m_mint));
}

#[test]
fn from_column_vector_3()
{
    let m_ref: Vector<f64> = vector![1.0; 2.0; 3.0];

    let m_mint = Vector3{ x: 1.0, y: 2.0, z: 3.0};

    assert_eq!(m_ref, Vector::from(m_mint));
}

#[test]
fn from_column_vector_4()
{
    let m_ref: Vector<f64> = vector![1.0; 2.0; 3.0; 4.0];

    let m_mint = Vector4{ x: 1.0, y: 2.0, z: 3.0, w: 4.0};

    assert_eq!(m_ref, Vector::from(m_mint));
}