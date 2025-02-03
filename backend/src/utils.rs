#[derive(Debug)]
pub struct Quintiles {
    // 20% of values are >= this amount
    quintile1: usize,
    // 40% of values are >= this amount
    quintile2: usize,
    quintile3: usize,
    quintile4: usize,
    // quintile5 is the minimum value
    pub total_quintile_sums: [usize; 5],
    // Just show in Debug
    #[allow(dead_code)]
    max: usize,
}

impl Quintiles {
    /// 0s are ignored
    pub fn new(values: &Vec<usize>) -> Self {
        let mut sorted = values.clone();
        sorted.retain(|x| *x > 0);
        sorted.sort();
        sorted.reverse();
        let n = ((sorted.len() as f64) / 5.0).ceil() as usize;

        let mut total_quintile_sums = [0; 5];
        for (idx, x) in sorted.iter().enumerate() {
            total_quintile_sums[idx / n] += *x;
        }

        Self {
            quintile1: sorted[n],
            quintile2: sorted[n * 2],
            quintile3: sorted[n * 3],
            quintile4: sorted[n * 4],
            total_quintile_sums,
            max: sorted[0],
        }
    }

    // Returns [1, 5]
    pub fn quintile(&self, value: usize) -> usize {
        if value >= self.quintile1 {
            1
        } else if value >= self.quintile2 {
            2
        } else if value >= self.quintile3 {
            3
        } else if value >= self.quintile4 {
            4
        } else {
            5
        }
    }
}
