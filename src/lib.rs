use num_bigint::BigUint;

// y^2 = x^3 + ax + b (mod p)
struct EllipticCurve {
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

// (x, y)
struct Point {
    x: BigUint,
    y: BigUint,
}


impl EllipticCurve {
    fn add(x: &Point, y: &Point) -> Point {
        todo!()
    }    

    fn double(x: &Point) -> Point {
        todo!()
    }

    // k * x
    fn scalar_mult(x: &Point, k: &BigUint) -> Point {
        // double-and-add algorithm OR binary-method
        todo!()
    }
}

struct FiniteField {
    p: BigUint,
}

impl FiniteField {
    fn add(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x + y) % &self.p
    }

    // addition by additive inverse (subtraction)
    fn sub(&self, x: &BigUint, y: &BigUint) -> BigUint {
        // Subtraction: x - y = x + (-y) mod p
        // Additive inverse: -y = (p - y) mod p
        let neg_y = (&self.p - y) % &self.p;
        self.add(x, &neg_y)
    }
    
    fn mul(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x * y) % &self.p
    }

    // multiplication by multiplicative inverse (division)
    fn div(&self, x: &BigUint, y: &BigUint) -> BigUint {
        // division in finite field: x / y = x * y^(-1) mod p

        if *y == BigUint::from(0u32) {
            panic!("?division by zero in finite field?");
        }
        
        // Using Fermat's little theorem: y^(-1) = y^(p-2) mod p (when p is prime)
        let p_minus_2 = &self.p - BigUint::from(2u32);
        let y_inverse = y.modpow(&p_minus_2, &self.p);
        self.mul(x, &y_inverse)
    }

    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x = BigUint::from(17u32);
        let y = BigUint::from(3u32);

        let res = BigUint::from(9u32);

        let field = FiniteField { p: BigUint::from(11u32) };
        assert_eq!(field.add(&x, &y), res);
    }
    
    #[test]
    fn test_sub() {
        let x = BigUint::from(17u32);
        let y = BigUint::from(3u32);

        let res = BigUint::from(3u32);

        let field = FiniteField { p: BigUint::from(11u32) };
        assert_eq!(field.sub(&x, &y), res);
    }

    #[test]
    fn test_mul() {
        let x = BigUint::from(17u32);
        let y = BigUint::from(3u32);

        let res = BigUint::from(7u32);

        let field = FiniteField { p: BigUint::from(11u32) };
        assert_eq!(field.mul(&x, &y), res);
    }

    #[test]
    fn test_div() {
        let x = BigUint::from(17u32);
        let y = BigUint::from(3u32);

        let res = BigUint::from(2u32);

        let field = FiniteField { p: BigUint::from(11u32) };
        assert_eq!(field.div(&x, &y), res);
    }

    //fail test case 
    #[test]
    #[should_panic]
    fn test_div_fail() {
        let x = BigUint::from(17u32);
        let y = BigUint::from(0u32);

        let field = FiniteField { p: BigUint::from(11u32) };
        assert_eq!(field.div(&x, &y), BigUint::from(0u32));
    }

    #[test]
    fn test_multiplicative_identity() {
        let x = BigUint::from(4u32);
        let field = FiniteField { p: BigUint::from(11u32) };
        
        // Test: x * x^(-1) = 1
        // 4^(-1) mod 11 = 3, so 4 * 3 = 12 ≡ 1 mod 11
        let one = BigUint::from(1u32);
        let x_inverse = field.div(&one, &x);  // 1/x = x^(-1)
        assert_eq!(field.mul(&x, &x_inverse), one);
    }

    #[test]
    fn test_additive_identity() {
        let x = BigUint::from(7u32);
        let field = FiniteField { p: BigUint::from(11u32) };
        
        // Test: x + (-x) = 0
        // -7 mod 11 = 4, so 7 + 4 = 11 ≡ 0 mod 11
        let zero = BigUint::from(0u32);
        let neg_x = field.sub(&zero, &x);  // 0 - x = -x
        assert_eq!(field.add(&x, &neg_x), zero);
    }
}
    


//modpow(exponent,modulus)