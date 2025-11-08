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

    fn sub(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x - y) % &self.p
    }
    
    fn mul(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x * y) % &self.p
    }

    fn div(&self, x: &BigUint, y: &BigUint) -> BigUint {
        // Division in finite field: x / y = x * y^(-1) mod p
        // Using Fermat's little theorem: y^(-1) = y^(p-2) mod p (when p is prime)
        
        let p_minus_2 = &self.p - BigUint::from(2u32);
        let y_inverse = y.modpow(&p_minus_2, &self.p);
        (x * y_inverse) % &self.p
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
}
    


//modpow(exponent,modulus)