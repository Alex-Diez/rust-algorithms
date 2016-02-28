pub trait Generator {

    fn next(&mut self) -> usize;
}

pub struct DefaultGenerator {
    current: usize
}

impl DefaultGenerator {

    pub fn new() -> DefaultGenerator {
        DefaultGenerator {
            current: 0
        }
    }
}

impl Generator for DefaultGenerator {

    fn next(&mut self) -> usize {
        let ret = self.current;
        self.current += 1;
        ret
    }
}

pub struct DigitBaseGenerator {
    current: usize,
    base: usize,
    limit: usize,
    iter: usize
}

impl DigitBaseGenerator {

    pub fn new(base: usize, limit: usize) -> DigitBaseGenerator {
        DigitBaseGenerator {
            current: 0,
            base: base,
            limit: limit,
            iter: 0
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }
}

impl Generator for DigitBaseGenerator {

    fn next(&mut self) -> usize {
        if self.current == self.iter * self.base + self.limit {
            self.iter += 1;
            self.current = self.iter * self.base;
        }
        let ret = self.current;
        self.current += 1;
        ret
    }
}
