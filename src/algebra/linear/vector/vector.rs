//! Vector

use super::{VectorIntoIterator, VectorIterator, VectorIteratorMut};
use crate::{
    algebra::{
        abstr::{Field, Scalar, Sign},
        linear::Matrix,
        linear::matrix::Transpose,
        abstr::{AbsDiffEq, RelativeEq},
    },
    elementary::{Exponential, Power},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fmt::Display,
    iter::IntoIterator,
    ops::{Neg},
};

/// Macro to construct vectors
///
/// ```
/// # #[macro_use]
/// # extern crate mathru;
/// # fn main()
/// # {
/// use mathru::algebra::linear::Vector;
///
/// // Construct a column vector of f64
/// let v1: Vector<f64> = vector![1.0; 2.0; 3.0];
/// // Construct a row vector of f32
/// let v2: Vector<f32> = vector![2.0, 3.0, 4.0];
/// # }
/// ```
#[macro_export]
macro_rules! vector
{
    ($( $x: expr ),*) =>
    {
        {
            let data_array = vec![ $($x),* ];
            Vector::new_row(data_array)
        }
    };

    ($( $x: expr );*) =>
    {
        {
            let data_array = vec![ $($x),* ];
            Vector::new_column(data_array)
        }
    };
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Vector<T>
{
    pub(super) data: Matrix<T>,
}

impl<T> IntoIterator for Vector<T> where T: Field + Scalar
{
    type Item = T;
    type IntoIter = VectorIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        VectorIntoIterator::new(self.data.into_iter() )
    }
}

//impl<T> FromIterator for Matrix<T>
//    where T: Field + Scalar
//{
//    fn from_iter<T>(iter: T) -> Se
//    T: IntoIterator<Item = A>,
//}

impl<T> Vector<T>
{
    pub fn iter(&self) -> VectorIterator<T>
    {
        VectorIterator::new(self.data.iter())
    }

    pub fn iter_mut(&mut self) -> VectorIteratorMut<T>
    {
        VectorIteratorMut::new(self.data.iter_mut())
    }
}

impl<T> Vector<T> 
    where T: Field + Scalar + Power
{
    /// Computes the p norm
    ///
    /// # Arguments
    ///
    /// p >= 1.0
    ///
    /// # Panics
    ///
    /// p < 1.0
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -2.0]);
    /// let p: f64 = 2.0;
    /// let norm_ref: f64 = a.eucl_norm();
    /// let norm: f64 = a.p_norm(&p);
    ///
    /// assert_eq!(norm_ref, norm);
    /// ```
    pub fn p_norm(&self, p: &T) -> T
    {
        assert!(*p >= T::one());

        let (m, n): (usize, usize) = self.dim();
        let mut sum: T = T::zero();
        for i in 0..(m * n)
        {
            let b: T = self[i];
            sum += b.pow(*p);
        }
        let norm: T = sum.pow(T::one() / *p);
        norm
    }
}

impl<T> Neg for Vector<T> where T: Field + Scalar
{
    type Output = Vector<T>;

    fn neg(self) -> Self::Output
    {
        self.apply(&|&x| -x)
    }
}

impl<T> Vector<T>
{
    pub fn convert_to_vec(self) -> Vec<T>
    {
        self.data.convert_to_vec()
    }
}

impl<T> Vector<T>
    where T: Field + Scalar + Power + Exponential
{
    /// Computes the euclidean norm
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -2.0]);
    /// let norm_ref: f64 = 3.7416573867739413;
    /// let norm: f64 = a.eucl_norm();
    ///
    /// assert_eq!(norm_ref, norm);
    /// ```
    pub fn eucl_norm(&self) -> T
    {
        let exp: T = T::from_f64(2.0);

        self.p_norm(&exp)
    }
}

impl<T> Vector<T> where T: Clone + Copy
{
    /// Returns a row vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_row(vec![1.0, 0.0, 3.0, -2.0]);
    /// ```
    pub fn new_row(data: Vec<T>) -> Self
    {
        Vector { data: Matrix::new(1, data.len(), data) }
    }

    /// Returns a column vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -2.0]);
    /// ```
    pub fn new_column(data: Vec<T>) -> Self
    {
        Vector { data: Matrix::new(data.len(), 1, data) }
    }

    pub fn apply(mut self: Vector<T>, f: &dyn Fn(&T) -> T) -> Self
    {
        self.data = self.data.apply(f);
        self
    }
}

impl<T> Vector<T> where T: Scalar
{
    /// Returns a row vector initialized with random numbers
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_row_random(4);
    /// ```
    pub fn new_row_random(n: usize) -> Self
    {
        Vector { data: Matrix::new_random(1, n) }
    }

    /// Returns a column vector initialized with random numbers
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column_random(4);
    /// ```
    pub fn new_column_random(m: usize) -> Self
    {
        Vector { data: Matrix::new_random(m, 1) }
    }
}

impl<T> Vector<T>
    where T: Field + Scalar
{
    /// Returns the transposed vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -2.0]);
    /// let b: Vector<f64> = a.transpose();
    /// ```
    pub fn transpose(mut self) -> Self
    {
        self.data = self.data.transpose();

        self
    }
}

impl<T> Vector<T>
    where T: Field + Scalar
{
    /// Computes the dot product of two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 0.0, 3.0, -2.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![-1.0, 2.0, 3.0, 5.0]);
    /// let dotp_ref: f64 = -2.0;
    ///
    /// let dotp: f64 = a.dotp(&b);
    ///
    /// assert_eq!(dotp_ref, dotp);
    /// ```
    pub fn dotp(&self, rhs: &Self) -> T
    {
        let (lhs_m, lhs_n) = self.dim();
        let (rhs_m, rhs_n) = rhs.dim();
        assert_ne!(lhs_m, 0);
        assert_eq!(lhs_n, 1);
        assert_eq!(lhs_m, rhs_m);
        assert_eq!(lhs_n, rhs_n);

        let temp: Vector<T> = self.clone().transpose();
        let res: Matrix<T> = &temp.data * &rhs.data;

        res[[0, 0]]
    }

    /// Find the argmax of the vector.
    ///
    /// Returns the index of the largest value in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a = Vector::new_column(vec![1.0, 2.0, -3.0, 5.0]);
    /// let idx = a.argmax();
    /// assert_eq!(idx, 3);
    /// ```
    pub fn argmax(&self) -> usize
    {
        let (m, n) = self.dim();

        let mut max_index: usize = 0;
        let mut max = self[max_index];

        let limit: usize = m.max(n);

        assert_ne!(limit, 0);

        for idx in 0..limit
        {
            let element: T = self[idx];
            if element > max
            {
                max_index = idx;
                max = element;
            }
        }

        max_index
    }

    /// Find the argmin of the vector.
    ///
    /// Returns the index of the smallest value in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a = Vector::new_column(vec![1.0, -2.0, -6.0, 75.0]);
    /// let b = a.argmin();
    /// assert_eq!(b, 2);
    /// ```
    pub fn argmin(&self) -> usize
    {
        let (m, n) = self.dim();

        let mut min_index: usize = 0;
        let mut min: T = self[min_index];

        let limit: usize = m.max(n);

        assert_ne!(limit, 0);

        for idx in 0..limit
        {
            let element: T = self[idx];
            if element < min
            {
                min_index = idx;
                min = element;
            }
        }

        min_index
    }
}

impl<T> Vector<T>
    where T: Field + Scalar
{
    /// Computes the dyadic product of two vectors
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::{Matrix, Vector};
    ///
    /// let a: Vector<f64> = Vector::new_row(vec![1.0, 0.0, 3.0, -2.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![-1.0, 2.0, 3.0, 5.0]);
    ///
    /// let m: Matrix<f64> = a.dyadp(&b);
    /// ```
    pub fn dyadp(&self, rhs: &Self) -> Matrix<T>
    {
        let (x_m, _x_n): (usize, usize) = self.dim();
        let (y_m, _y_n): (usize, usize) = rhs.dim();
        let mut c: Matrix<T> = Matrix::zero(x_m, y_m);

        for i in 0..x_m
        {
            for j in 0..y_m
            {
                c[[i, j]] = self[i] * rhs[j];
            }
        }
        c
    }
}

impl<T> Vector<T>
    where T: Field + Scalar + Power
{
    pub fn reflector(&self) -> Vector<T>
    {
        let two = T::one() + T::one();
        let mut x_temp: Vector<T> = self.clone();

        let norm_x: T = self.p_norm(&two);

        x_temp[0] += self[0].sign() * norm_x;
        let x_temp_norm: T = x_temp.p_norm(&two);
        x_temp[0] /= x_temp_norm;

        x_temp
    }
}

impl<T> Vector<T>
    where T: Field + Scalar
{
    /// Returns the zero vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![0.0, 0.0, 0.0, 0.0]);
    /// let b: Vector<f64> = Vector::zero(4);
    ///
    /// assert_eq!(a, b)
    /// ```
    pub fn zero(m: usize) -> Self
    {
        Vector { data: Matrix::zero(m, 1) }
    }
}

impl<T> Vector<T>
    where T: Field + Scalar
{
    /// Returns the one vector
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 1.0, 1.0, 1.0]);
    /// let b: Vector<f64> = Vector::one(4);
    ///
    /// assert_eq!(a, b)
    /// ```
    pub fn one(m: usize) -> Self
    {
        let mut vec: Vec<T> = Vec::with_capacity(m);

        for _i in 0..m
        {
            vec.push(T::one());
        }

        Vector::new_column(vec)
    }
}

impl<T> Vector<T>
{
    /// Returns the vector dimension
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![1.0, 2.0, 3.0, 4.0]);
    /// let (m, n): (usize, usize) = a.dim();
    /// assert_eq!(4, m);
    /// assert_eq!(1, n);
    /// ```
    pub fn dim(&self) -> (usize, usize)
    {
        self.data.dim()
    }
}

impl<T> Vector<T>
    where T: Field + Scalar
{
    /// Returns a slice of the vector
    ///
    /// # Arugments
    ///
    /// 0 <= s < m \
    /// 0 <= e < m \
    ///
    /// s: start \
    /// e: end \
    ///
    /// # Panics
    ///
    /// iff s and e are out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, -2.0, 3.0, -7.0]);
    /// a = a.get_slice(1, 2);
    ///
    /// let a_ref: Vector<f64> = Vector::new_column(vec![-2.0, 3.0]);
    ///
    /// assert_eq!(a_ref, a);
    /// ```
    pub fn get_slice(&self, s: usize, e: usize) -> Vector<T>
    {
        let (m, n): (usize, usize) = self.dim();
        if m == 1
        {
            assert!(s < n);
            assert!(e < n);
        }
        else
        {
            assert!(s < m);
            assert!(e < m);
        }

        let mut slice: Vector<T> = Vector::zero(e - s + 1);

        for r in s..(e + 1)
        {
            slice[r - s] = self[r]
        }

        slice
    }

    /// Overwrite a slice of the vector with the given values
    ///
    /// # Arugments
    ///
    /// 0 <= s < m \
    ///
    /// s: start \
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let mut a: Vector<f64> = Vector::new_column(vec![1.0, -2.0, 3.0, -7.0]);
    /// let b: Vector<f64> = Vector::new_column(vec![-5.0, 4.0]);
    /// a.set_slice(&b, 1);
    ///
    /// let a_ref: Vector<f64> = Vector::new_column(vec![1.0, -5.0, 4.0, -7.0]);
    ///
    /// assert_eq!(a_ref, a);
    /// ```
    pub fn set_slice(&mut self, rhs: &Self, s: usize)
    {
        let (m, _n): (usize, usize) = self.dim();
        let (s_m, _s_n): (usize, usize) = rhs.dim();
        assert!(s + s_m <= m);

        for r in s..(s + s_m)
        {
            self[r] = rhs[r - s];
        }
    }
}

impl<T> PartialEq<Self> for Vector<T> where T: Scalar
{
    /// Compares if two vectors are equal
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::linear::Vector;
    ///
    /// let a: Vector<f64> = Vector::new_column(vec![0.0, 0.0, 0.0, 0.0]);
    /// let b: Vector<f64> = Vector::zero(4);
    ///
    /// assert_eq!(true, a.eq(&b))
    /// ```
    fn eq(&self, other: &Self) -> bool
    {
        if self.data == other.data
        {
            return true;
        }
        false
    }
}

impl<T> Display for Vector<T> where T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        self.data.fmt(f)
    }
}


impl<T> Sign for Vector<T> where T: Field + Scalar
{
    fn sign(&self) -> Self
    {
        (self.clone()).apply(&|x: &T| x.sign())
    }

    fn abs(&self) -> Self
    {
        (self.clone()).apply(&|x: &T| x.abs())
    }

    fn is_positive(&self) -> bool
    {
        unimplemented!();
    }

    fn is_negative(&self) -> bool
    {
        unimplemented!();
    }
}

impl<T> AbsDiffEq for Vector<T>
    where T: Field + Scalar + AbsDiffEq<Epsilon = T>
{
    type Epsilon = T;

    fn default_epsilon() -> T
    {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Vector<T>, epsilon: T) -> bool
    {
        self.data.abs_diff_eq(&other.data, epsilon)
    }
}

impl<T> RelativeEq for Vector<T>
    where T: Field + Scalar + AbsDiffEq<Epsilon = T> + RelativeEq
{

    fn default_max_relative() -> T
    {
        T::default_max_relative()
    }

    /// A test for equality that uses a relative comparison if the values are far apart.
    fn relative_eq(&self, other: &Vector<T>, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool
    {
        self.data.relative_eq(&other.data, epsilon, max_relative)
    }
}

