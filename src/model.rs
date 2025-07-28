use crate::consts::*;
use nalgebra::{Complex, Matrix6};

pub struct SpinSeq {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl SpinSeq {
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        SpinSeq { a, b, c, d, e, f }
    }
    pub fn diag_matrix(&self , j : f64) -> Matrix6<Complex<f64>> {

        let jj = j * ONE;

        Matrix6::new(
            self.a * jj, ZERO, ZERO, ZERO, ZERO, ZERO,
            ZERO, self.b * jj, ZERO, ZERO, ZERO, ZERO,
            ZERO, ZERO, self.c * jj, ZERO, ZERO, ZERO,
            ZERO, ZERO, ZERO, self.d * jj, ZERO, ZERO,
            ZERO, ZERO, ZERO, ZERO, self.e * jj, ZERO,
            ZERO, ZERO, ZERO, ZERO, ZERO, self.f * jj,
        )
    }
    pub fn debug(&self) -> String {
        format!(
            "a{}_b{}_c{}_d{}_e{}_f{}",
            self.a.to_string().replace('.', "p"),
            self.b.to_string().replace('.', "p"),
            self.c.to_string().replace('.', "p"),
            self.d.to_string().replace('.', "p"),
            self.e.to_string().replace('.', "p"),
            self.f.to_string().replace('.', "p")
        )
    }
    pub fn fm() -> Self{
        SpinSeq::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0)
    }
    pub fn afm() -> Self{
        SpinSeq::new(1.0, -1.0, 1.0, -1.0, 1.0, -1.0)
    }
    pub fn uuuddd() -> Self{
        SpinSeq::new(1.0, 1.0, 1.0, -1.0, -1.0, -1.0)
    }
    pub fn one() -> Self{
        SpinSeq::new(1.0, 1.0, 1.0, 1.0, 1.0, -1.0)
    }
    pub fn twin() -> Self{
        SpinSeq::new(1.0, 1.0, 1.0, 1.0, -1.0, -1.0)
    }
    pub fn tri() -> Self{
        SpinSeq::new(1.0, 1.0, 1.0, -1.0, 1.0, -1.0)
    }
    pub fn face() -> Self{
        SpinSeq::new(1.0, 1.0, -1.0, 1.0, 1.0, -1.0)
    }
    pub fn para() -> Self{
        SpinSeq::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }
}

pub struct Param{
    pub lambda : f64,
    pub jj : f64,
    pub mu : f64,
}

impl Param{
    pub fn new(lambda: f64, jj: f64, mu: f64) -> Self{
        Param { lambda, jj, mu }
    }
    pub fn test() -> Self{
        let lambda = 0.1 * T;
        let jj = 0.25;
        let mu = 0.0;

        Param { lambda, jj, mu }
    }
    pub fn sj() -> Self{
        let lambda = 0.1 * T;
        let jj = 10.0;
        let mu = 0.0;

        Param { lambda, jj, mu }
    }
    pub fn debug(&self) -> String {
        format!("lambda{}_j{}_mu{}", 
            self.lambda.to_string().replace('.', "p"),
            self.jj.to_string().replace('.', "p"), 
            self.mu.to_string().replace('.', "p")
    )
    }
}