/// 1-dimensional sampled function
/// x correspond to x-axis values
/// values correspond to function values
#[derive(Default)]
struct SampledFn<T> {
    x: Vec<T>,
    values: Vec<T>,
}

impl<T> SampledFn<T> {
    /// Create a sampled function by giving x-axis values and function values
    /// The struct created take ownership of object given in argument
    pub fn new(x: Vec<T>, values: Vec<T>) -> Self {
        return Self { x, values };
    }
}

#[cfg(test)]
mod tests {
    use super::super::cmp_approx::*;
    use super::*;

    #[test]
    fn test_sampled_fn_constructor() {
        let x_reference: Vec<f64> = vec![0.0, 1.0, 2.0];
        let values_reference: Vec<f64> = vec![0.0, 1.0, 4.0];

        let fun: SampledFn<f64> = SampledFn::new(x_reference.clone(), values_reference.clone());

        let precision: f64 = 0.01;
        assert!(fun.x.approx(&x_reference, precision));
        assert!(fun.values.approx(&values_reference, precision));
    }
}
