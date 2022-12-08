use crate::data::DataSet;
use crate::test::Test;
use std::time::{Duration, Instant};

const WARMUP: Duration = Duration::new(3, 0);

/// Contains a warmups too
pub struct Bench {
    setup: Box<dyn Fn() -> Test>,
    test: Box<dyn Fn(&Test) -> Option<()>>,
    runs: u32,
    data: DataSet,
}

impl Bench {
    pub fn new<S, T>(runs: u32, setup: S, test: T) -> Self
    where
        S: Fn() -> Test + 'static,
        T: Fn(&Test) -> Option<()> + 'static,
    {
        Self {
            runs,
            setup: Box::new(setup),
            test: Box::new(test),
            data: DataSet::default(),
        }
    }

    /// Execute the benchmark runs.
    ///
    /// Runs with a 3-second warmup to ensure a warm cache.
    pub fn run(&mut self) {
        let t = (self.setup)();
        if self.runs == 0 {
            return;
        }
        eprintln!("warming up...");
        let warmup = Instant::now();
        while warmup.elapsed() < WARMUP {
            (self.test)(&t);
        }
        eprintln!("running");
        for _ in 0..self.runs {
            let i = Instant::now();
            let res = (self.test)(&t);
            let i = i.elapsed();
            self.data.add_run(res.map(|_| i));
        }
        self.data.process();
    }
}

impl std::fmt::Display for Bench {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
