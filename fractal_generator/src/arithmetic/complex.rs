use std::fmt;
use std::ops;

#[derive(
    Debug,
    Clone,
    Copy,    
)]

pub struct  Complex{
    pub real: f64,
    pub imaginary: f64,
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Complex{
        Complex { real, imaginary}
    }
}

impl Default for Complex{
    fn default() -> Self{
        Complex{real: 0.0, imaginary: 0.0}
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}i)", self.real, self.imaginary)
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        Complex{
            real: rhs.real + self.real,
            imaginary: rhs.imaginary + self.imaginary,
        }
    }
}

impl ops::Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex{
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl ops::Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex{
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imaginary == other.imaginary
    }
    
}


#[cfg(test)]
mod complex_numbers_methods_tests{
    use super::*;

    #[test]
    fn is_default_zero(){
        let c = Complex::default();
        assert_eq!(c.real, 0.0);
        assert_eq!(c.imaginary,0.0);
    }

    #[test]
    fn is_addition_ok(){
        let c1 = Complex{real: 1.0, imaginary: 10.0};
        let c2 = Complex{real: 2.0, imaginary: 20.0};

        let c3 = c1+c2;
        let c3_expected = Complex{real: 3.0, imaginary: 30.0};

        assert_eq!(c3, c3_expected);
    }

    #[test]
    fn is_mult_ok(){
        let c1 = Complex{real:4.0, imaginary: -1.0};
        let c2 = Complex{real: -3.0, imaginary: 2.0};

        let c3 = c1*c2;
        let c3_expected = Complex{real: -10.0, imaginary: 11.0};

        assert_eq!(c3, c3_expected);
    }

    #[test]
    fn is_subtraction_ok(){
        let c1 = Complex{real: 1.0, imaginary: 10.0};
        let c2 = Complex{real: 2.0, imaginary: 20.0};

        let c3 = c1-c2;
        let c3_expected = Complex{real: -1.0, imaginary: -10.0};

        assert_eq!(c3, c3_expected);
    }
}