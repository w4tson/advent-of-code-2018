pub struct Digits {
    n: usize,
    divisor: usize,
}

impl Digits {
    pub fn new(n: usize) -> Self {
        let mut divisor = 1;
        while n >= divisor * 10 {
            divisor *= 10;
        }

        Digits {
            n: n,
            divisor: divisor,
        }
    }
}

impl Iterator for Digits {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let v = Some(self.n / self.divisor);
            self.n %= self.divisor;
            self.divisor /= 10;
            v
        }

    }
}