/*
Wilee is scientific computing library
Copyright (C) 2022  Bastian Gonzalez Acevedo

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

/// 1-dimensional sampled function
/// args correspond to x-axis values
/// values correspond to function values
#[derive(Default)]
struct SampledFn<T> {
    args: Vec<T>,
    values: Vec<T>,
}

impl<T> SampledFn<T>
where
    T: PartialOrd + Copy,
{
    /// Sort sampled function coordinates according to x-axis values
    /// This function is private, indeed it is only used in other function.
    fn sort_according_to_args(&mut self) {
        let mut coordinates: Vec<(&T, &T)> = self.args.iter().zip(self.values.iter()).collect();

        coordinates.sort_by(|(arg_left, _value_left), (arg_right, _value_right)| {
            arg_left.partial_cmp(arg_right).unwrap()
        });

        (self.args, self.values) = coordinates.iter().cloned().unzip();
    }

    /// Create a sampled function by giving x-axis values and function values
    /// The struct created take ownership of object given in argument
    pub fn new(args: Vec<T>, values: Vec<T>) -> Self {
        let mut sampled_fn: Self = Self { args, values };
        sampled_fn.sort_according_to_args();

        return sampled_fn;
    }

    /// Search interval where value given in argument is.
    /// We return the index of x-axis such as value is in [x[index], x[index + 1]]
    pub fn search_interval(&self, x: T) -> Option<usize> {
        match self.args.iter().position(|&arg| x < arg) {
            Some(index) => {
                if index == 0 {
                    return None;
                } else {
                    return Some(index - 1);
                }
            }
            None => {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::cmp_approx::*;
    use super::*;

    #[test]
    fn test_sampled_fn_constructor() {
        let args: Vec<f64> = vec![0.0, 2.0, 1.0];
        let values: Vec<f64> = vec![0.0, 4.0, 1.0];

        let args_reference: Vec<f64> = vec![0.0, 1.0, 2.0];
        let values_reference: Vec<f64> = vec![0.0, 1.0, 4.0];

        let fun: SampledFn<f64> = SampledFn::new(args.clone(), values.clone());

        let precision: f64 = 0.01;
        assert!(fun.args.approx(&args_reference, precision));
        assert!(fun.values.approx(&values_reference, precision));
    }

    #[test]
    fn test_sampled_fn_search_interval_must_return_none() {
        let args: Vec<f64> = vec![0.0, 2.0, 1.0];
        let values: Vec<f64> = vec![0.0, 4.0, 1.0];

        let fun: SampledFn<f64> = SampledFn::new(args, values);

        assert!(fun.search_interval(-0.75).is_none());
        assert!(fun.search_interval(2.98).is_none());
    }

    #[test]
    fn test_sampled_fn_search_interval_must_return_some_index() {
        let args: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0];
        let values: Vec<f64> = vec![0.0, 1.0, 4.0, 9.0];

        let fun: SampledFn<f64> = SampledFn::new(args, values);

        match fun.search_interval(0.65) {
            Some(index) => assert_eq!(index, 0),
            None => assert!(false),
        }

        match fun.search_interval(1.54) {
            Some(index) => assert_eq!(index, 1),
            None => assert!(false),
        }

        match fun.search_interval(2.75) {
            Some(index) => assert_eq!(index, 2),
            None => assert!(false),
        }
    }
}
