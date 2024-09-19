fn main() {
    println!("{x}", x = square_area_to_circle(42.0));
}

fn square_area_to_circle(size:f64) -> f64 {
    //resp = pi/4*size
    size * std::f64::consts::FRAC_PI_4
}


#[cfg(test)]
mod tests {
    use super::square_area_to_circle;

    fn assert_close(a:f64, b:f64, epsilon:f64) {
        assert!( (a-b).abs() < epsilon, "Expected: {}, got: {}",b,a);
    }

    #[test]
    fn returns_expected() {
    assert_close(square_area_to_circle(9.0), 7.0685834705770345, 1e-8);
    }

    #[test]
    fn returns_expected2() {    
    assert_close(square_area_to_circle(20.0), 15.70796326794897, 1e-8);
    }

    #[test]
    fn returns_expected3() {    
    assert_close(square_area_to_circle(42.0), 32.98672286269283, 1e-8);
    }

    #[test]
    fn returns_expected4() {    
    assert_close(square_area_to_circle(0.0), 0.0, 1e-8);
    }

    #[test]
    fn returns_expected5() {    
    assert_close(square_area_to_circle(f64::MAX), 141190488647306400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0, 1e-8);
    }
}