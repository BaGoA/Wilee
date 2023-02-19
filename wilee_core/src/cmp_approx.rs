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

/// Compare two values using a precision
/// That allow to know if two values are approximatively equals
pub trait Approx<T> {
    fn approx(&self, reference: &Self, precision: T) -> bool;
}

/// Implementation for f64
/// The precision is in 64 bits
impl Approx<f64> for f64 {
    fn approx(&self, reference: &Self, precision: f64) -> bool {
        let mut error: f64 = (self - reference).abs();

        if *reference != 0.0 {
            error /= reference.abs();
        }

        return error < precision;
    }
}

/// Implementation for f32
/// The precision is in 32 bits
impl Approx<f32> for f32 {
    fn approx(&self, reference: &Self, precision: f32) -> bool {
        let mut error: f32 = (self - reference).abs();

        if *reference != 0.0 {
            error /= reference.abs();
        }

        return error < precision;
    }
}

/// Implementation for Vec<f64>
/// The precision is in 64 bits
impl Approx<f64> for Vec<f64> {
    fn approx(&self, reference: &Self, precision: f64) -> bool {
        let count_of_true: usize = self
            .iter()
            .zip(reference.iter())
            .filter(|(value, value_ref)| value.approx(&value_ref, precision))
            .count();

        return count_of_true == self.len();
    }
}

/// Implementation for Vec<f32>
/// The precision is in 32 bits
impl Approx<f32> for Vec<f32> {
    fn approx(&self, reference: &Self, precision: f32) -> bool {
        let count_of_true: usize = self
            .iter()
            .zip(reference.iter())
            .filter(|(value, value_ref)| value.approx(&value_ref, precision))
            .count();

        return count_of_true == self.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_for_f32() {
        let reference: f32 = 1.0;
        let precision: f32 = 0.01;

        let mut value: f32 = 1.0;
        assert!(value.approx(&reference, precision));

        value = 1.2;
        assert!(!value.approx(&reference, precision));
    }

    #[test]
    fn test_approx_for_f64() {
        let reference: f64 = 1.0;
        let precision: f64 = 0.01;

        let mut value: f64 = 1.0;
        assert!(value.approx(&reference, precision));

        value = 1.2;
        assert!(!value.approx(&reference, precision));
    }

    #[test]
    fn test_approx_for_vec_f32() {
        let reference: Vec<f32> = vec![1.0, 2.0, 3.0];
        let precision: f32 = 0.01;

        let mut value: Vec<f32> = vec![1.0, 2.0, 3.0];
        assert!(value.approx(&reference, precision));

        value = vec![1.0, 2.2, 3.0];
        assert!(!value.approx(&reference, precision));
    }

    #[test]
    fn test_approx_for_vec_f64() {
        let reference: Vec<f64> = vec![1.0, 2.0, 3.0];
        let precision: f64 = 0.01;

        let mut value: Vec<f64> = vec![1.0, 2.0, 3.0];
        assert!(value.approx(&reference, precision));

        value = vec![1.0, 2.2, 3.0];
        assert!(!value.approx(&reference, precision));
    }
}
