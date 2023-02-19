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

/// Representation of  2-dimensional point
/// x and y correspond to first and second coordinates
#[derive(Default)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T> {
    /// Create a 2-dimensionals point from two values
    fn new(x: T, y: T) -> Self {
        return Self { x, y };
    }
}

#[cfg(test)]
mod tests {
    use super::super::cmp_approx::*;
    use super::*;

    #[test]
    fn test_point2_constructor() {
        let x: f64 = 1.3;
        let y: f64 = 4.3;
        let point: Point2<f64> = Point2::new(x, y);

        let precision: f64 = 0.01;
        assert!(point.x.approx(&x, precision));
        assert!(point.y.approx(&y, precision));
    }
}
