use std::fmt::{Display, Formatter, Result};
use std::time::Duration;

#[derive(Default)]
pub struct DataSet {
    timings: Vec<Duration>,
    fail_count: u32,
    std_dev: Option<Duration>,
    average: Option<Duration>,
}

impl DataSet {
    pub fn add_run(&mut self, duration: Option<Duration>) {
        match duration {
            Some(v) => self.timings.push(v),
            None => self.fail_count += 1,
        }
    }

    /// run this after all timings have been collected
    pub fn process(&mut self) {
        let avg = average(&self.timings);
        self.average = Some(avg);
        self.std_dev = std_dev(&self.timings, &avg);
    }
}

fn average(timings: &Vec<Duration>) -> Duration {
    if timings.is_empty() {
        return Duration::ZERO;
    }
    let n = timings.len() as u32;
    timings.iter().fold(Duration::ZERO, |a, t| a + *t) / n
}

fn std_dev(timings: &Vec<Duration>, mean: &Duration) -> Option<Duration> {
    if timings.is_empty() {
        return None;
    }
    let (m, n) = (mean.as_secs_f64(), timings.len() as f64);
    // square mean difference
    let smd = |t: f64, mean: f64| (t - mean).powi(2);
    // sum of squares of absolute difference between mean
    let seconds = timings.iter().map(|d| d.as_secs_f64());
    let sum_of_squares = seconds.fold(0.0, |a, t| a + smd(m, t));
    Some(Duration::from_secs_f64((sum_of_squares / (n - 1.0)).sqrt()))
}

impl Display for DataSet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "
runs:     {runs}
std.dev:  {std_dev:?}
average:  {avg:?}",
            avg = self.average.unwrap_or(Duration::ZERO),
            std_dev = self.std_dev.unwrap_or(Duration::ZERO),
            runs = self.timings.len(),
        )
    }
}
