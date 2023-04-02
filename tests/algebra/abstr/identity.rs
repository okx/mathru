macro_rules! test_identity
{
    ($add_id:expr, $mul_id:expr, $(($id:ident, $s:ty)),*) =>
    {
        $(
            mod $id
            {
                use mathru::algebra::abstr::{Identity, Addition, Multiplication};

                #[test]
                fn test_identity_addition()
                {
                    let id: $s = Identity::<Addition>::id();
                    assert_eq!($add_id, id);
                }

                #[test]
                fn test_identity_multiplication()
                {
                    let id: $s = Identity::<Multiplication>::id();
                    assert_eq!($mul_id, id);
                }
            }
        )*
    };
}

test_identity!(0,
               1,
               (u8, u8),
               (u16, u16),
               (u32, u32),
               (u64, u64),
               (u128, u128));
test_identity!(0,
               1,
               (i8, i8),
               (i16, i16),
               (i32, i32),
               (i64, i64),
               (i128, i128));
test_identity!(0.0, 1.0, (f32, f32), (f64, f64));
