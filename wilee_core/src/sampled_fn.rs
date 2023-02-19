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

use super::point2::Point2;

/// Representation of univariate sampled function
/// coodinates correspond to pair of (x-axis, y-axis) values
#[derive(Default)]
struct SampledFn<T> {
    coordinates: Vec<Point2<T>>,
}

impl<T> SampledFn<T>
where
    T: PartialOrd + Copy,
{
    /// Create a sampled function by giving x-axis values and function values
    /// The struct created take ownership of object given in argument
    pub fn new(args: Vec<T>, values: Vec<T>) -> Self {
        let mut coordinates: Vec<Point2<T>> = args
            .iter()
            .zip(values.iter())
            .map(|(&arg, &value)| Point2::new(arg, value))
            .collect();

        coordinates.sort_by(|pt_left, pt_right| pt_left.x.partial_cmp(&pt_right.x).unwrap());

        return Self { coordinates };
    }

    /// Search interval where value given in argument is.
    /// We return the index of x-axis such as value is in [x[index], x[index + 1]]
    pub fn search_interval(&self, x: T) -> Option<usize> {
        match self.coordinates.iter().position(|point| x < point.x) {
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

    /// Get x-axis values
    /// This function is private because is only useful for test
    fn get_x_vec(&self) -> Vec<T> {
        return self.coordinates.iter().map(|point| point.x).collect();
    }

    /// Get y-axis values
    /// This function is private because is only useful for test
    fn get_y_vec(&self) -> Vec<T> {
        return self.coordinates.iter().map(|point| point.y).collect();
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
        assert!(fun.get_x_vec().approx(&args_reference, precision));
        assert!(fun.get_y_vec().approx(&values_reference, precision));
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
