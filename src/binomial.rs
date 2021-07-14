/// Binomial coefficients iterator.
pub(crate) struct Binomial {
    n: u64,
    idx: u64,
}

impl Binomial {
    /// Creates a binomial coefficients iterator for the given `n`.
    pub fn new(n: u32) -> Binomial {
        Binomial {
            n: n as u64,
            idx: 0,
        }
    }
}

impl Iterator for Binomial {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let c = if self.idx > self.n {
            return None;
        } else if self.idx == 0 || self.idx == self.n {
            1
        } else if self.idx == 1 || self.idx == self.n - 1 {
            self.n
        } else {
            let k = if self.idx > self.n / 2 {
                self.n - self.idx
            } else {
                self.idx
            };
            let mut a = 1;
            for i in self.n - k + 1..self.n + 1 {
                a *= i;
            }
            let mut b = 2;
            for i in 2..k {
                b *= i;
            }
            a / b
        };
        self.idx += 1;
        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::Binomial;

    #[test]
    fn test_binomial() {
        assert_binomial(0, &[1]);
        assert_binomial(1, &[1, 1]);
        assert_binomial(2, &[1, 2, 1]);
        assert_binomial(3, &[1, 3, 3, 1]);
        assert_binomial(4, &[1, 4, 6, 4, 1]);
        assert_binomial(5, &[1, 5, 10, 10, 5, 1]);
    }

    fn assert_binomial(n: u32, expect: &[u64]) {
        let v: Vec<u64> = Binomial::new(n).collect();
        assert_eq!(v, expect);
    }
}
