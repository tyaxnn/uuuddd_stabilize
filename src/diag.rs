use crate::consts::*;
use crate::model::{Param,SpinSeq};
use nalgebra::{Complex, Const,Matrix6, SymmetricEigen, Vector2, DimMin, Dim, OMatrix, OVector};

#[derive(Clone,Debug)]
pub struct SEud {
    pub u: SymmetricEigen<Complex<f64>, Const<6>>,
    pub d: SymmetricEigen<Complex<f64>, Const<6>>,
}

impl SEud
{
    pub fn new(
        u: SymmetricEigen<Complex<f64>, Const<6>>,
        d: SymmetricEigen<Complex<f64>, Const<6>>,
    ) -> Self {
        SEud { u, d }
    }

    pub fn sort(self) -> Self {
        SEud::new(
            sort_symmetric_eigen_ascending(self.u),
            sort_symmetric_eigen_ascending(self.d),
        )
    }

    pub fn eigenvalues(&self) -> [f64;12] {
            let mut out = [0.0; 12];
            for i in 0..6 {
                out[i] = self.u.eigenvalues[i];
                out[i + 6] = self.d.eigenvalues[i];
            }

            out
    }
}

pub fn diag(kk : Vector2<f64>, param : &Param, spin_seq : &SpinSeq) -> SEud{
    let ed1p = Complex::exp( I * kk.dot(&D1)) * -T;
    let ed1m = Complex::exp(-I * kk.dot(&D1)) * -T;
    let ed2p = Complex::exp( I * kk.dot(&D2)) * -T;
    let ed2m = Complex::exp(-I * kk.dot(&D2)) * -T;
    let ed3p = Complex::exp( I * kk.dot(&D3)) * -T;
    let ed3m = Complex::exp(-I * kk.dot(&D3)) * -T;

    let lambda = param.lambda;
    let j = param.jj;
    let mu = param.mu;

    let plu = {
        Complex::exp( I * kk.dot(&A1)) +
        Complex::exp( I * kk.dot(&A2)) +
        Complex::exp( I * kk.dot(&A3))
    } * I * lambda;
    let mnu = {
        Complex::exp(-I * kk.dot(&A1)) +
        Complex::exp(-I * kk.dot(&A2)) +
        Complex::exp(-I * kk.dot(&A3))
    } * I * lambda;

    let mm = -mu * ONE;

    let a = 0.0;

    let hamiltonian_u = Matrix6::new(
        mm  ,ed1p,-plu*a,ed3p, mnu*a,ed2p,
        ed1m,mm  ,ed2m,-plu,ed3m, mnu,
        mnu*a,ed2p,mm ,ed1p,-plu*a, ed3p,
        ed3m, mnu,ed1m,mm  ,ed2m,-plu,
        -plu*a,ed3p, mnu*a,ed2p,mm  ,ed1p,
        ed2m,-plu,ed3m, mnu,ed1m,mm
    ) + spin_seq.diag_matrix(j);

    let hamiltonian_d = Matrix6::new(
        mm  ,ed1p, plu*a,ed3p,-mnu*a,ed2p,
        ed1m,mm  ,ed2m, plu,ed3m,-mnu,
        -mnu*a,ed2p,mm  ,ed1p, plu*a, ed3p,
        ed3m,-mnu,ed1m,mm  ,ed2m, plu,
        plu*a,ed3p,-mnu*a,ed2p,mm  ,ed1p,
        ed2m, plu,ed3m,-mnu,ed1m,mm
    ) - spin_seq.diag_matrix(j); 

    let unsorted = SEud::new(
        SymmetricEigen::new(hamiltonian_u),SymmetricEigen::new(hamiltonian_d)
    );

    unsorted.sort()

}

pub fn sort_symmetric_eigen_ascending<const N: usize>(
    eigen: SymmetricEigen<Complex<f64>, Const<N>>,
) -> SymmetricEigen<Complex<f64>, Const<N>>
where
    Const<N>: Dim + DimMin<Const<N>, Output = Const<N>>,
{
    let mut indexed_eigenvalues: Vec<(usize, f64)> = eigen
        .eigenvalues
        .iter()
        .enumerate()
        .map(|(i, &val)| (i, val))
        .collect();

    indexed_eigenvalues.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // サイズ N のベクトルと行列を作成
    let mut sorted_eigenvalues = OVector::<f64, Const<N>>::zeros();
    let mut sorted_eigenvectors = OMatrix::<Complex<f64>, Const<N>, Const<N>>::zeros();

    for (new_index, (old_index, _)) in indexed_eigenvalues.iter().enumerate() {
        sorted_eigenvalues[new_index] = eigen.eigenvalues[*old_index];
        sorted_eigenvectors.set_column(new_index, &eigen.eigenvectors.column(*old_index));
    }

    SymmetricEigen {
        eigenvalues: sorted_eigenvalues,
        eigenvectors: sorted_eigenvectors,
    }
}