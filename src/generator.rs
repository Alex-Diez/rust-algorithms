pub trait Generator {

    fn next(&mut self) -> usize;
}

pub struct DefaultGenrator {
    current: usize
}

impl DefaultGenrator {

    pub fn new() -> Box<DefaultGenrator> {
        let g = DefaultGenrator {
            current: 0
        };
        Box::new(g)
    }
}

impl Generator for DefaultGenrator {

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

    pub fn new(base: usize, limit: usize) -> Box<DigitBaseGenerator> {
        let g = DigitBaseGenerator {
            current: 0,
            base: base,
            limit: limit,
            iter: 0
        };
        Box::new(g)
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
