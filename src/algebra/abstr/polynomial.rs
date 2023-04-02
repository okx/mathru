//! Polynomial
use std::fmt::{Result, Formatter, Display};
use crate::algebra::abstr::{Real, AbsDiffEq, Monoid, Semigroup, SemigroupAdd, Quasigroup, RelativeEq};
use crate::algebra::{
    abstr::{Field, Scalar},
};
use std::ops::{Add, Mul, Div, Sub, Neg, AddAssign, MulAssign, SubAssign};
use crate::algebra::abstr::Zero;
use crate::algebra::abstr::magma::{MagmaAdd, MagmaMul, Magma};
use crate::algebra::abstr::monoid::{MonoidAdd};
use crate::algebra::abstr::identity::{Identity};
use crate::algebra::abstr::operator::{Addition, Multiplication};
use crate::algebra::abstr::group::{Group, GroupAdd};
use crate::algebra::abstr::loop_::{Loop};


/// Polynomial expression
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Polynomial<T> {
    coef: Vec<T>,
}

impl<T> Polynomial<T>
{
    /// Creates a new polynomial with the given coefficients
    ///
    /// # Arguments
    ///
    /// * `coef:  Coefficients
    ///
    /// # Panics
    ///
    /// If the coef is an empty vector
    ///
    /// # Example
    ///
    /// ```math
    /// (3x^2 + 2x + 1)
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    /// ```
    pub fn from_coef(mut coef: Vec<T>) -> Polynomial<T>
    {
        if coef.is_empty()
        {
            panic!()
        }
        coef.reverse();
        Polynomial
        {
            coef
        }
    }

    /// Creates a new polynomial from the given roots
    ///
    /// <https://en.wikipedia.org/wiki/Vieta%27s_formulas>
    ///
    /// # Arguments
    ///
    /// * `root`:  Roots
    ///
    /// # Panics
    ///
    /// If the root is an empty vector
    ///
    /// # Example
    ///
    /// ```math
    /// (x - 1)(x - 2)(x - 3) = -6 + 11x -6x^2 + x^3
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_root(vec![1.0, 2.0, 3.0]);
    ///
    /// let b: Polynomial<f64> = Polynomial::from_coef(vec![1.0, -6.0, 11.0, -6.0]);
    ///
    /// assert_eq!(b, a);
    /// ```
    pub fn from_root(root: Vec<T>) -> Polynomial<T>
        where T: Field + Scalar
    {
        if root.is_empty()
        {
            panic!()
        }

        let n: usize = root.len();
        let mut coef: Vec<T> = vec![T::zero(); n + 1];

        // Set highest order coefficient as 1
        coef[n] = T::one();

        for i in 1..n + 1
        {
            for j in n - i..n
            {
                let c: T = coef[j+1];
                let r: T = root[i - 1];
                coef[j] += (-T::one()) * r * c;
            }
        }

        Polynomial{coef}
    }

    /// Creates a Legendre polynomial with the given degree
    ///
    /// <https://en.wikipedia.org/wiki/Legendre_polynomials>
    /// 
    /// # Arguments
    /// * `n`: Degree of the Legendre polynomial
    /// 
    /// # Example
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    /// 
    /// let p = Polynomial::from_legendre(2);
    /// let p_ref = Polynomial::from_coef(vec![1.5, 0.0, -0.5]);
    /// 
    /// assert_eq!(p_ref, p)
    /// ```
    pub fn from_legendre(n: u32) -> Polynomial<T>
        where T: Field + Scalar + AbsDiffEq<Epsilon = T>
    {
        let n_f64 = n as f64;

        match n {
            0 => Polynomial{coef: vec![T::one()]},      
            1 => Polynomial{coef: vec![T::zero(), T::one()]}, 
            _ => {
                let p_1 = Polynomial{coef: vec![T::zero(), T::from_f64(2.0f64 * n_f64 - 1.0f64)]};
                (( p_1 * Polynomial::from_legendre(n - 1)
                    - Polynomial{coef: vec![T::from_f64(n_f64 - 1.0f64)]} * Polynomial::from_legendre(n - 2)) / (Polynomial{coef: vec![T::from_f64(n_f64)]})).0
            }
        }
    }

    /// Creates a Chebyshev polynomial of first kind with the given degree
    ///
    /// <https://en.wikipedia.org/wiki/Chebyshev_polynomials>
    /// 
    /// # Arguments
    /// * `n`: Degree of the Chebyshev polynomial
    /// 
    /// # Example
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    /// 
    /// let p = Polynomial::from_chebyshev_t(2);
    /// let p_ref = Polynomial::from_coef(vec![2.0, 0.0, -1.0]);
    /// 
    /// assert_eq!(p_ref, p)
    /// ```
    pub fn from_chebyshev_t(n: u32) -> Polynomial<T>
        where T: Field + Scalar + AbsDiffEq<Epsilon = T>
    {
        match n {
            0 => Polynomial{ coef: vec![T::one()]},      
            1 => Polynomial{ coef: vec![T::zero(), T::one()]}, 
            _ => {
                let p_1 = Polynomial{ coef: vec![T::zero(), T::from_f64(2.0f64)]};
                p_1 * Polynomial::from_chebyshev_t(n - 1) -  Polynomial::from_chebyshev_t(n - 2)
            }
        }
    }

    /// Creates a Chebyshev polynomial of second kind with the given degree
    ///
    /// <https://en.wikipedia.org/wiki/Chebyshev_polynomials>
    /// 
    /// # Arguments
    /// * `n`: Degree of the Chebyshev polynomial
    /// 
    /// # Example
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    /// 
    /// let p = Polynomial::from_chebyshev_u(2);
    /// let p_ref = Polynomial::from_coef(vec![4.0, 0.0, -1.0]);
    /// 
    /// assert_eq!(p_ref, p)
    /// ```
    pub fn from_chebyshev_u(n: u32) -> Polynomial<T>
        where T: Field + Scalar + AbsDiffEq<Epsilon = T>
    {
        match n {
            0 => Polynomial{coef: vec![T::one()]},      
            1 => Polynomial{coef: vec![T::zero(), T::from_f64(2.0)]}, 
            _ => {
                let p_1 = Polynomial{coef: vec![T::zero(), T::from_f64(2.0f64)]};
                p_1 * Polynomial::from_chebyshev_u(n - 1) -  Polynomial::from_chebyshev_u(n - 2)
            }
        }
    }
}



impl<T> Display for Polynomial<T>
    where T: Display + Real
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        let mut only_zero_terms: bool = true;

        for (i, a_i) in self.coef.iter().enumerate()
        {
            if a_i.abs_diff_ne(&T::zero(), T::default_epsilon())
            {
                if i > 1
                {
                    if only_zero_terms
                    {
                        write!(f, "{}x^{}", a_i, i)?;
                    }
                    else
                    {
                        write!(f, " + {}x^{}", a_i, i)?;
                    }
                } else {
                    if i == 0
                    {
                        write!(f, "{}", a_i)?;
                    }
                    else {
                        if only_zero_terms
                        {
                            write!(f, "{}x", a_i)?;
                        } else {
                            write!(f, " + {}x", a_i)?
                        }
                    }
                }

                only_zero_terms = false
            }
        }

        Ok(())
    }
}

impl<T> Polynomial<T>
    where T: Field + Scalar
{
    /// Evaluate the polynomial with horner's rule
    ///
    /// # Argument
    ///
    /// * `x`: The polynomial is evaluated at this value
    ///
    /// # Example
    ///
    /// ```math
    /// (1 + 2x + 3x^2) = p(x) \\\\
    /// p(2) = 17
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    ///
    /// assert_eq!(17.0, a.eval(2.0));
    /// ```
    pub fn eval(&self, x: T) -> T
    {
        let mut s: T = T::zero();

        for v in self.coef.iter().rev()
        {
            s = *v + (x * s);
        }
        s
    }
}


impl<T> Add<Polynomial<T>> for Polynomial<T>
    where T: MagmaAdd + Scalar
{
    type Output = Polynomial<T>;

    /// Adds two polynomials
    ///
    /// # Example
    ///
    /// ```math
    /// (3x^2 + 2x + 1) + (2x + 1) = 3x^2 + 4x + 2
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    /// let b: Polynomial<f64> = Polynomial::from_coef(vec![2.0, 1.0]);
    /// let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 4.0, 2.0]);
    ///
    /// assert_eq!(c, a + b);
    /// ```
    fn add(self, rhs: Polynomial<T>) -> Self::Output
    {
        (&self).add(&rhs)
    }
}

impl<'a, 'b, T> Add<&'b Polynomial<T>> for &'a Polynomial<T>
    where T: MagmaAdd + Scalar
{
    type Output = Polynomial<T>;

    /// Adds two polynomials
    ///
    /// # Example
    ///
    /// ```math
    /// (3x^2 + 2x + 1) + (2x + 1) = 3x^2 + 4x + 2
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    /// let b: Polynomial<f64> = Polynomial::from_coef(vec![2.0, 1.0]);
    /// let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 4.0, 2.0]);
    ///
    /// assert_eq!(c, &a + &b);
    /// ```
    fn add(self, rhs: &'b Polynomial<T>) -> Self::Output
    {
        let mut sum = if self.coef.len() > rhs.coef.len()
        {
            self.coef.clone()
        }
        else {
            rhs.coef.clone()
        };

        for (i, (a_i, b_i)) in self.coef.iter().zip(rhs.coef.iter()).enumerate()
        {
            sum[i] = *a_i + *b_i
        }

        Polynomial {
            coef: sum
        }
    }
}

impl<T> AddAssign for Polynomial<T>
    where T: MagmaAdd + Scalar
{
    fn add_assign(&mut self, rhs: Self)
    {
        *self = (*self).clone().add(rhs)
    }
}

impl<T> Sub<Polynomial<T>> for Polynomial<T>
    where T: Sub<Output = T> + Scalar
{
    type Output = Polynomial<T>;

    /// Subtracts two polynomials
    ///
    /// # Example
    ///
    /// ```math
    /// (3x^2 + 2x + 1) + (2x + 1) = 3x^2 + 4x + 1
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    /// let b: Polynomial<f64> = Polynomial::from_coef(vec![2.0, 1.0]);
    /// let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 0.0, 0.0]);
    ///
    /// assert_eq!(c, a - b);
    /// ```
    fn sub(self, rhs: Polynomial<T>) -> Self::Output
    {
        (&self).sub(&rhs)
    }
}

impl<'a, 'b, T> Sub<&'b Polynomial<T>> for &'a Polynomial<T>
    where T: Sub<Output = T> + Scalar
{
    type Output = Polynomial<T>;

    /// Subtracts two polynomials
    ///
    /// # Example
    ///
    /// ```math
    /// (3x^2 + 2x + 1) - (2x + 21) = 3x^2
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    /// let b: Polynomial<f64> = Polynomial::from_coef(vec![2.0, 1.0]);
    /// let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 0.0, 0.0]);
    ///
    /// assert_eq!(c, &a - &b);
    /// ```
    fn sub(self, rhs: &'b Polynomial<T>) -> Self::Output
    {
        let mut diff = if self.coef.len() > rhs.coef.len()
        {
            self.coef.clone()
        }
        else {
            (-rhs).coef
        };

        for (i, (a_i, b_i)) in self.coef.iter().zip(rhs.coef.iter()).enumerate()
        {
            diff[i] = *a_i - *b_i
        }

        Polynomial {
            coef: diff
        }
    }
}

impl<T> SubAssign for Polynomial<T>
    where T: Sub<Output = T> + Scalar
{
    fn sub_assign(&mut self, rhs: Self)
    {
        *self = self.clone().sub(rhs)
    }
}


impl<T> Zero for Polynomial<T>
    where T: Zero
{
    fn zero() -> Polynomial<T>
    {
        Polynomial::from_coef(vec![T::zero()])
    }
}

impl<'a, T> Neg for &'a Polynomial<T>
    where T: Neg<Output = T> + Clone
{
    type Output = Polynomial<T>;

    /// Negates the polynomial
    ///
    /// # Example
    ///
    /// ```math
    /// -(2x + 1) = -2x - 1
    /// ```
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![2.0, 1.0]);
    ///
    /// assert_eq!(Polynomial::from_coef(vec![-2.0, -1.0]), -a);
    /// ```
    ///
    fn neg(self) -> Self::Output
    {
        Polynomial {
            coef: self.coef.clone().into_iter().map(|x| -x).collect::<Vec<T>>()
        }
    }
}

impl<T> Neg for Polynomial<T>
    where T: Neg<Output = T>
{
    type Output = Polynomial<T>;

    /// Returns the negative of a polynomial
    ///
    /// # Example
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    ///
    /// assert_eq!(Polynomial::from_coef(vec![-1.0, -2.0, -3.0]), -a)
    /// ```
    fn neg(self) -> Self::Output
    {
        Polynomial {
            coef: self.coef.into_iter().map(|x| -x).collect::<Vec<T>>()
        }
    }
}

impl<T> Polynomial<T>
{
    /// Returns the degree of the polynomial
    ///
    /// # Example
    ///
    /// ```math
    /// deg(1 + 2x + 3x^2) =2
    /// ```
   ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    ///
    /// assert_eq!(2, a.degree())
    /// ```
    pub fn degree(&self) -> usize
    {
        self.coef.len() - 1
    }
}

impl<T> Mul<Polynomial<T>> for Polynomial<T>
    where T: MagmaMul + MonoidAdd + Scalar
{
    type Output = Polynomial<T>;

    /// Multiplies two polynomials
    ///
    /// # Arguments
    ///
    /// * `self': Factor
    /// * `rhs?: Factor
    ///
    /// # Example
    ///
    /// ```math
    /// (1 + 2x + 3x^2)(1 + x) = (1 + 3x + 5x^2 + 3x^3)
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    /// let b: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0]);
    /// let c: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 3.0, 5.0, 3.0]);
    ///
    /// assert_eq!(c, a * b)
    /// ```
    fn mul(self, rhs: Polynomial<T>) -> Self::Output
    {
        (&self).mul(&rhs)
    }
}

impl<'a, 'b, T> Mul<&'b Polynomial<T>> for &'a Polynomial<T>
    where T: MagmaMul + MonoidAdd + Scalar
{
    type Output = Polynomial<T>;

    /// Multiplies two polynomials
    ///
    /// # Arguments
    ///
    /// * `self': Factor
    /// * `rhs?: Factor
    ///
    /// # Example
    ///
    /// ```math
    /// (3x^2 + 2x + 1)(x + 1) = (3x^3 + 5x^2 + 3x + 1)
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    /// let b: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0]);
    /// let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 5.0, 3.0, 1.0]);
    ///
    /// assert_eq!(c, &a * &b)
    /// ```
    fn mul(self, rhs: &'b Polynomial<T>) -> Self::Output
    {
        let deg_lhs = self.degree();
        let deg_rhs = rhs.degree();

        let deg_res =  deg_lhs + deg_rhs;
        let mut res: Vec<T> = vec![T::zero(); deg_res + 1];

        for (i, v_i) in self.coef.iter().enumerate()
        {
            for (j, v_j) in rhs.coef.iter().enumerate()
            {
                res[i + j] += *v_i * *v_j
            }
        }

        Polynomial{coef: res}
    }
}

impl<T> MulAssign for Polynomial<T>
    where T: MagmaMul + MonoidAdd + Scalar
{
    fn mul_assign(&mut self, rhs: Self)
    {
        *self = (*self).clone().mul(rhs)
    }
}

impl<T> Div<Polynomial<T>> for Polynomial<T>
    where T: Field + Scalar + AbsDiffEq<Epsilon = T>
{
    type Output = (Polynomial<T>, Polynomial<T>);

    /// Divides two polynomials
    ///
    /// # Example
    ///
    /// ```math
    /// (3x^3 + 5x^2 + 3x + 1) / (3x^2 + 2x + 1) = (x + 1)
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    /// use crate::mathru::algebra::abstr::Zero;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    /// let b: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0]);
    /// let c: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 3.0, 5.0, 3.0]);
    ///
    /// assert_eq!(b, (c.clone() / a.clone()).0);
    /// assert_eq!(Polynomial::zero(), (c / a).1)
    /// ```
    fn div(self, rhs: Polynomial<T>) -> Self::Output
    {
        (&self).div(&rhs)
    }
}

impl<'a, 'b, T> Div<&'b Polynomial<T>> for &'a Polynomial<T>
    where T: Field + Scalar + AbsDiffEq<Epsilon = T>
{
    type Output = (Polynomial<T>, Polynomial<T>);

    /// Divides two polynomials
    ///
    /// # Example
    ///
    /// ```math
    /// (3x^3 + 5x^2 + 3x + 1) / (3x^2 + 2x + 1) = (x + 1)
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    /// use crate::mathru::algebra::abstr::Zero;
    ///
    /// let a: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 2.0, 3.0]);
    /// let b: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0]);
    /// let c: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 3.0, 5.0, 3.0]);
    ///
    /// assert_eq!(b, (&c / &a).0);
    /// assert_eq!(Polynomial::zero(), (&c / &a).1)
    /// ```
    fn div(self, rhs: &'b Polynomial<T>) -> Self::Output
    {
        if rhs.degree() > self.degree()
        {
            return (Polynomial::zero(), self.clone())
        }

        let mut remainder: Vec<T> = self.coef.clone();
        let quotient_degree: usize = self.degree() - rhs.degree();
        let mut quotient = vec![T::zero(); quotient_degree + 1];

        for i in (0..(quotient_degree + 1)).rev()
        {
            let q: T = remainder[rhs.degree()  + i] / rhs.coef[rhs.degree()];

            quotient[i] = q;

            for (k, v_k) in rhs.coef.iter().enumerate()
            {
                remainder[k + i] -= *v_k * q;
            }
        }

        (Polynomial{coef: quotient}, Polynomial{coef: Polynomial::reduce_coef(remainder)})
    }
}

impl<T> Polynomial<T>
    where T: Zero + AbsDiffEq
{
    fn reduce_coef(mut coef: Vec<T>) -> Vec<T>
    {
        let len = coef.len();
        for i in (1..len).rev()
        {
            let v = &coef[i];
            if v.abs_diff_eq(&T::zero(), T::default_epsilon())
            {
                coef.pop();
            }
            else {
                break;
            }
        }

        coef
    }

    pub fn reduce(self) -> Self
    {
       Polynomial{coef: Polynomial::reduce_coef(self.coef)}
    }
}

impl<T> Polynomial<T>
    where T: Field + Scalar + AbsDiffEq<Epsilon = T>
{
    /// Differentiate polynomial
    ///
    /// # Example
    ///
    /// ```math
    /// p(x) = 3x^3 + 5x^2 + 3x + 1
    /// ```
    ///
    /// ```math
    /// \frac{\partial p(x)}{\partial x} = 9x^2 + 10x + 3
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 5.0, 3.0, 1.0]);
    /// let c_s: Polynomial<f64> = Polynomial::from_coef(vec![9.0, 10.0, 3.0]);
    ///
    /// assert_eq!(c_s, c.differentiate());
    /// ```
    pub fn differentiate(&self) -> Polynomial<T>
    {
        if self.degree() == 0
        {
            return Polynomial::from_coef(vec![T::zero()])
        }

        let mut coef_diff = Vec::with_capacity(self.degree());
        for (i, a_i) in self.coef.iter().skip(1).enumerate()
        {
            coef_diff.push(T::from_f64((i + 1) as f64) * *a_i);
        }

        Polynomial{coef: Polynomial::reduce_coef(coef_diff)}
    }

    /// Integrate polynomial
    ///
    /// # Example
    ///
    /// ```math
    /// p(x) = 3x^2 + 2x + 1
    /// ```
    ///
    /// ```math
    /// \int p(x) dx = c + x + x^2 + x^3
    /// ```
    ///
    /// ```
    /// use mathru::algebra::abstr::Polynomial;
    ///
    /// let c: Polynomial<f64> = Polynomial::from_coef(vec![3.0, 2.0, 1.0]);
    /// let c_s: Polynomial<f64> = Polynomial::from_coef(vec![1.0, 1.0, 1.0, 0.0]);
    ///
    /// assert_eq!(c_s, c.integrate());
    /// ```
    pub fn integrate(&self) -> Polynomial<T>
    {
        let mut coef_int = Vec::with_capacity(self.degree() + 1);
        coef_int.push(T::zero());

        for (i, a_i) in self.coef.iter().enumerate()
        {
            coef_int.push(*a_i / T::from_f64((i + 1) as f64));
        }

        Polynomial{coef: Polynomial::reduce_coef(coef_int)}
    }
}

impl<T> Magma<Addition> for Polynomial<T>
    where T: MagmaAdd + Magma<Addition> + Scalar
{
    fn operate(self, rhs: Self) -> Self
    {
        self.add(rhs)
    }
}

impl<T> MagmaAdd for Polynomial<T>
    where T: MagmaAdd + Scalar
{

}

impl<T> Magma<Multiplication> for Polynomial<T>
    where T: MagmaMul + MonoidAdd + Magma<Multiplication> + Scalar
{
    fn operate(self, rhs: Self) -> Self
    {
        self.mul(rhs)
    }
}

impl<T> MagmaMul for Polynomial<T>
    where T: MagmaMul + MonoidAdd + Scalar
{

}

impl<T> Semigroup<Addition> for Polynomial<T>
    where T: MagmaAdd + Scalar
{

}

impl<T> SemigroupAdd for Polynomial<T>
    where T: SemigroupAdd + Semigroup<Addition> + MagmaAdd + Scalar
{
}

impl<T> Identity<Addition> for Polynomial<T>
    where T: Identity<Addition>
{
    fn id() -> Self
    {
        Polynomial{coef: vec![T::id()]}
    }
}

impl<T> Monoid<Addition> for Polynomial<T>
    where T: MagmaAdd + Scalar + Identity<Addition>
{
}

impl<T> MonoidAdd for Polynomial<T>
    where T: Monoid<Addition> + SemigroupAdd + Zero + Scalar
{

}

impl<T> Quasigroup<Addition> for Polynomial<T>
    where T: Quasigroup<Addition> + Scalar + MagmaAdd
{

}

impl<T> Loop<Addition> for Polynomial<T>
    where T: Loop<Addition> + Quasigroup<Addition> + Scalar + MagmaAdd
{

}

impl<T> Group<Addition> for Polynomial<T>
    where T: Group<Addition> + Loop<Addition> + Quasigroup<Addition> + Scalar + MagmaAdd
{

}

impl<T> GroupAdd for Polynomial<T>
    where T: GroupAdd + Group<Addition> + Loop<Addition> + Quasigroup<Addition> + Scalar + MagmaAdd + MonoidAdd
{

}

impl<T> AbsDiffEq for Polynomial<T>
    where T: AbsDiffEq<Epsilon = T> + Clone
{
    type Epsilon = T;

    fn default_epsilon() -> T
    {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Polynomial<T>, epsilon: T) -> bool
    {
        for (a, b) in self.coef.iter().zip(other.coef.iter())
        {
            if a.abs_diff_ne(b, epsilon.clone())
            {
                return false;
            }
        }
        true
    }
}

impl<T> RelativeEq for Polynomial<T>
    where T: RelativeEq<Epsilon = T> + Clone
{
    fn default_max_relative() -> T
    {
        T::default_epsilon()
    }

    fn relative_eq(&self, other: &Self, epsilon: T, max_relative: T) -> bool
    {
        for (a, b) in self.coef.iter().zip(other.coef.iter())
        {
            if a.relative_ne(b, epsilon.clone(), max_relative.clone())
            {
                return false;
            }
        }
        true
    }
}