pub use algorithms::generator::{Generator, DefaultGenrator, DigitBaseGenerator};

pub use expectest::prelude::be_equal_to;

describe! generator_tests {

    describe! default_generator_tests {

        it "should generate numbers from 0 till 100" {
            let mut generator = DefaultGenrator::new();

            let data = (0..).take(101).collect::<Vec<usize>>();
            let mut vec = Vec::with_capacity(100);
            for _ in (0..).take(101) {
                vec.push((*generator).next());
            }

            expect!(vec).to(be_equal_to(data));
        }
    }

    describe! digit_base_generator_tests {

        it "should generate numebrs with spaces between 10..16, 26..32 and so on" {
            let mut generator = DigitBaseGenerator::new(16, 10);

            let mut data = Vec::new();
            for i in (0..).take(101) {
                let iter = i / 16;
                if i < 10 + iter*16 {
                    data.push(i);
                }
            }

            let mut vec = Vec::new();
            for _ in 0..data.len() {
                vec.push((*generator).next());
            }

            expect!(vec).to(be_equal_to(data));
        }
    }
}
