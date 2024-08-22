#[derive(Debug)]
struct Tuple<const N: usize> {
    data: [f64; N],
}

impl<const N: usize> Tuple<N> {
    fn iter(&self) -> std::slice::Iter<f64> {
        self.data.iter()
    }
}

impl<const N: usize> From<[f64; N]> for Tuple<N> {
    fn from(array: [f64; N]) -> Self {
        Tuple { data: array }
    }
}

macro_rules! implement_operations {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<const N: usize> std::ops::$trait<&Tuple<N>> for &Tuple<N> {
            type Output = Tuple<N>;

            fn $method(self, other: &Tuple<N>) -> Self::Output {
                Tuple::from(std::array::from_fn(|i| self.data[i] $op other.data[i]))
            }
        }
    };

    ($trait:ident, $method:ident, $op:tt, $scalar:ty) => {
        impl<const N: usize> std::ops::$trait<$scalar> for &Tuple<N> {
            type Output = Tuple<N>;

            fn $method(self, scalar: $scalar) -> Tuple<N> {
                Tuple::from(std::array::from_fn(|i| self.data[i] $op scalar))
            }
        }
    };
}

implement_operations!(Add, add, +);
implement_operations!(Sub, sub, -);
implement_operations!(Mul, mul, *);
implement_operations!(Div, div, /);
implement_operations!(Mul, mul, *, f64);
implement_operations!(Div, div, /, f64);

impl<const N: usize> std::ops::Neg for &Tuple<N> {
    type Output = Tuple<N>;

    fn neg(self) -> Self::Output {
        Tuple::from(std::array::from_fn(|i| -self.data[i]))
    }
}

impl<const N: usize> std::cmp::PartialEq for Tuple<N> {
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| (a - b).abs() < 0.00001)
    }
}

impl<const N: usize> std::ops::Index<usize> for Tuple<N> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(
            index < N,
            "Index out of bounds: the len is {} but the index is {}",
            N,
            index
        );
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod creation {
        use super::*;

        #[test]
        fn access_elements() {
            let t = Tuple::from([1.0, 2.0, 3.0, 4.0]);
            assert_eq!(t[0], 1.0);
            assert_eq!(t[1], 2.0);
            assert_eq!(t[2], 3.0);
            assert_eq!(t[3], 4.0);
        }

        #[test]
        fn from_array() {
            let t: Tuple<4> = [1.0, 2.0, 3.0, 4.0].into();
            assert_eq!(t, Tuple::from([1.0, 2.0, 3.0, 4.0]));
        }
    }

    mod comparison {
        use super::*;

        #[test]
        fn identical_tuples() {
            assert_eq!(
                Tuple::from([1.0, 2.0, 3.0, 4.0]),
                Tuple::from([1.0, 2.0, 3.0, 4.0])
            );
        }

        #[test]
        fn small_difference_within_epsilon() {
            assert_eq!(
                Tuple::from([1.0, 2.0, 3.0, 4.0]),
                Tuple::from([1.0, 2.0, 3.0, 4.00001])
            );
        }

        #[test]
        fn different_tuples() {
            assert_ne!(
                Tuple::from([1.0, 2.0, 3.0, 4.0]),
                Tuple::from([2.0, 3.0, 4.0, 5.0])
            );
        }
    }

    mod arithmetic {
        use super::*;

        mod tuple_operations {
            use super::*;

            #[test]
            fn addition_of_tuples() {
                assert_eq!(
                    &Tuple::from([1.0, 2.0, 3.0, 4.0]) + &Tuple::from([2.0, 3.0, 4.0, 5.0]),
                    Tuple::from([3.0, 5.0, 7.0, 9.0])
                );
            }

            #[test]
            fn subtraction_of_tuples() {
                assert_eq!(
                    &Tuple::from([1.0, 2.0, 3.0, 4.0]) - &Tuple::from([2.0, 3.0, 4.0, 5.0]),
                    Tuple::from([-1.0, -1.0, -1.0, -1.0])
                );
            }

            #[test]
            fn multiplication_of_tuples() {
                assert_eq!(
                    &Tuple::from([1.0, 2.0, 3.0, 4.0]) * &Tuple::from([2.0, 3.0, 4.0, 5.0]),
                    Tuple::from([2.0, 6.0, 12.0, 20.0])
                );
            }

            #[test]
            fn division_by_a_tuple() {
                assert_eq!(
                    &Tuple::from([2.0, 4.0, 6.0, 8.0]) / &Tuple::from([2.0, 2.0, 2.0, 2.0]),
                    Tuple::from([1.0, 2.0, 3.0, 4.0])
                );
            }
        }

        mod scalar_operations {
            use super::*;

            #[test]
            fn multiplication_by_a_scalar() {
                assert_eq!(
                    &Tuple::from([1.0, 2.0, 3.0, 4.0]) * 3.5,
                    Tuple::from([3.5, 7.0, 10.5, 14.0])
                );
            }

            #[test]
            fn multiplication_by_a_fraction() {
                assert_eq!(
                    &Tuple::from([1.0, 2.0, 3.0, 4.0]) * 0.5,
                    Tuple::from([0.5, 1.0, 1.5, 2.0])
                );
            }

            #[test]
            fn division_by_a_scalar() {
                assert_eq!(
                    &Tuple::from([1.0, 2.0, 3.0, 4.0]) / 2.0,
                    Tuple::from([0.5, 1.0, 1.5, 2.0])
                );
            }
        }

        #[test]
        fn negation_of_tuple() {
            assert_eq!(
                -&Tuple::from([1.0, 2.0, 3.0, 4.0]),
                Tuple::from([-1.0, -2.0, -3.0, -4.0])
            );
        }
    }
}
