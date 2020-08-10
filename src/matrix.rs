use std::ops;

use num::traits::Num;

#[derive(Clone)]
pub struct Matrix<T: Num + Copy, const M: usize, const N: usize> {
    pub data: [[T; M]; N],
}

#[test]
fn test1() {
    let m = Matrix {
        data: [[1, 4], [2, 5], [3, 6]],
    };
    //
    let v = &m * &[7, 8, 9];
    //
    println!("{:?}", v);
}

impl<
        T: std::fmt::Debug + Num + Clone + Copy,
        const M: usize,
        const N: usize,
    > std::fmt::Debug for Matrix<T, M, N>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..M {
            write!(f, "[")?;
            for j in 0..N {
                write!(f, "{:?}, ", self.data[j][i])?;
            }
            writeln!(f, "]")?;
        }
        //
        Ok(())
    }
}

impl<T: Num + Copy, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new() -> Self {
        Matrix {
            data: [[T::zero(); M]; N],
        }
    }
    //
    pub fn transpose(&self) -> Matrix<T, N, M> {
        let mut other = Matrix::new();
        //
        for j in 0..N {
            for i in 0..M {
                other.data[i][j] = self.data[j][i];
            }
        }
        //
        other
    }
}

impl<T: Num + Copy, const M: usize, const N: usize> ops::Add<Matrix<T, M, N>>
    for Matrix<T, M, N>
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut ans = Self::new();
        //
        for j in 0..N {
            for i in 0..M {
                ans.data[j][i] = self.data[j][i] + other.data[j][i];
            }
        }
        //
        ans
    }
}

impl<T: Num + Copy, const M: usize, const N: usize, const O: usize>
    ops::Mul<&Matrix<T, N, O>> for &Matrix<T, M, N>
{
    type Output = Matrix<T, M, O>;
    fn mul(self, other: &Matrix<T, N, O>) -> Matrix<T, M, O> {
        let mut ans = Matrix::new();
        //
        for i in 0..M {
            for j in 0..O {
                for k in 0..N {
                    ans.data[j][i] =
                        ans.data[j][i] + self.data[k][i] * other.data[j][k];
                }
            }
        }
        //
        ans
    }
}

impl<T: Num + Copy, const M: usize, const N: usize> ops::Mul<&[T; N]>
    for &Matrix<T, M, N>
{
    type Output = [T; M];
    fn mul(self, other: &[T; N]) -> [T; M] {
        let wrap = Matrix { data: [*other] };
        (self * &wrap).data[0]
    }
}
