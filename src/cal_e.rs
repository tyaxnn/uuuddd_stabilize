use crate ::consts::*;
use crate::diag::diag;
use crate::model::{Param, SpinSeq};
use crate::dv2::DV2;

use nalgebra::{Vector2,};

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn phase_diagram_lambda(
    graph_mesh: usize,
    lambda : f64
) { 

    let filename = &format!(
        "./output/lambda_phase_diagram_data/lambda_phase_diagram_{}_{}.dat",
        lambda.to_string().replace('.', "p"),
        graph_mesh
    );

    let path = Path::new(filename);
    let file = File::create(path).expect("ファイル作成に失敗しました");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "n,jj,stable").unwrap();

    let j_div = 40; // λの分割数
    let j_max = 3.; // λの最大値

    for i in 0..j_div{
        let jj = i as f64 / j_div as f64 * j_max;

        let param = &Param::new(lambda, jj, 0.0);
        let energy_fm = cal_e_vs_n(param, &SpinSeq::fm(), graph_mesh);
        let energy_afm = cal_e_vs_n(param, &SpinSeq::afm(), graph_mesh);
        let energy_uuuddd = cal_e_vs_n(param, &SpinSeq::uuuddd(), graph_mesh);
        let energy_tri = cal_e_vs_n(param, &SpinSeq::tri(), graph_mesh);
        let energy_twin = cal_e_vs_n(param, &SpinSeq::twin(), graph_mesh);
        let energy_one = cal_e_vs_n(param, &SpinSeq::one(), graph_mesh);

        for j in 0..=100 {
            let n = j as f64 / 100.0;
            let efm = energy_fm[j];
            let eafm = energy_afm[j];
            let euuuddd = energy_uuuddd[j];
            let etri = energy_tri[j];
            let etwin = energy_twin[j];
            let eone = energy_one[j];

            // ベクタで最小値と対応する名前を取得
            let energies = [
                (efm, "fm"),
                (eafm, "afm"),
                (euuuddd, "uuuddd"),
                (etri, "ududdd"),
                (etwin, "uudddd"),
                (eone, "uddddd"),
            ];

            // 最小エネルギーとその名前を選ぶ
            let (_, stable) = energies
                .iter()
                .copied()
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .unwrap();

            writeln!(writer, "{:.6},{:.6},{}", n, jj, stable).unwrap();
        }

    }
    

    println!("3スピン系列のエネルギーを '{filename}' に保存しました。");
}

pub fn phase_diagram_j(
    graph_mesh: usize,
    jj : f64
) { 

    let filename = &format!(
        "./output/phase_diagram_data/phase_diagram_{}_{}.dat",
        jj.to_string().replace('.', "p"),
        graph_mesh
    );

    let path = Path::new(filename);
    let file = File::create(path).expect("ファイル作成に失敗しました");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "n,lambda,stable").unwrap();

    let lambda_div = 40; // λの分割数
    let lambda_max = 0.5; // λの最大値

    for i in 0..lambda_div{
        let lambda = i as f64 / lambda_div as f64 * lambda_max;

        let param = &Param::new(lambda, jj, 0.0);
        let energy_fm = cal_e_vs_n(param, &SpinSeq::fm(), graph_mesh);
        let energy_afm = cal_e_vs_n(param, &SpinSeq::afm(), graph_mesh);
        let energy_uuuddd = cal_e_vs_n(param, &SpinSeq::uuuddd(), graph_mesh);
        let energy_tri = cal_e_vs_n(param, &SpinSeq::tri(), graph_mesh);
        let energy_twin = cal_e_vs_n(param, &SpinSeq::twin(), graph_mesh);
        let energy_one = cal_e_vs_n(param, &SpinSeq::one(), graph_mesh);

        for j in 0..=100 {
            let n = j as f64 / 100.0;
            let efm = energy_fm[j];
            let eafm = energy_afm[j];
            let euuuddd = energy_uuuddd[j];
            let etri = energy_tri[j];
            let etwin = energy_twin[j];
            let eone = energy_one[j];

            // ベクタで最小値と対応する名前を取得
            let energies = [
                (efm, "fm"),
                (eafm, "afm"),
                (euuuddd, "uuuddd"),
                (etri, "ududdd"),
                (etwin, "uudddd"),
                (eone, "uddddd"),
            ];

            // 最小エネルギーとその名前を選ぶ
            let (_, stable) = energies
                .iter()
                .copied()
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                .unwrap();

            writeln!(writer, "{:.6},{:.6},{}", n, lambda, stable).unwrap();
        }

    }
    

    println!("3スピン系列のエネルギーを '{filename}' に保存しました。");
}

pub fn output_all_energy_vs_n(
    param: &Param,
    graph_mesh: usize,
) {
        let energy_fm = cal_e_vs_n(param, &SpinSeq::fm(), graph_mesh);
        let energy_afm = cal_e_vs_n(param, &SpinSeq::afm(), graph_mesh);
        let energy_uuuddd = cal_e_vs_n(param, &SpinSeq::uuuddd(), graph_mesh);
        let energy_tri = cal_e_vs_n(param, &SpinSeq::tri(), graph_mesh);
        let energy_twin = cal_e_vs_n(param, &SpinSeq::twin(), graph_mesh);
        let energy_one = cal_e_vs_n(param, &SpinSeq::one(), graph_mesh);

    let filename = &format!(
        "./output/change_n_data/energy_vs_n_{}.dat",
        param.debug(),
    );

    let path = Path::new(filename);
    let file = File::create(path).expect("ファイル作成に失敗しました");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "#n,fm,afm,uuuddd,ududddd,uudddd,uddddd").unwrap();

    for i in 0..=100 {
        //let epara = energy_para[i];

        let n = i as f64 / 100.0;
        let efm = energy_fm[i];
        let eafm = energy_afm[i] - efm;
        let euuuddd = energy_uuuddd[i] - efm;
        let etri = energy_tri[i] - efm;
        let etwin = energy_twin[i] - efm;
        let eone = energy_one[i] - efm;


        writeln!(writer, "{:.3},{:.10},{:.10},{:.10},{:.10},{:.10},{:.10}", n, efm - efm, eafm, euuuddd, etri, etwin, eone).unwrap();
    }

    println!("3スピン系列のエネルギーを '{filename}' に保存しました。");
}

pub fn cal_e_vs_n(param: &Param, spin_seq: &SpinSeq, graph_mesh: usize) -> Vec<f64> {
    let mesh_kx = graph_mesh;
    let mesh_ky = graph_mesh;

    let mut all_eigens = Vec::new();

    for i in 0..mesh_kx {
        for j in 0..mesh_ky {
            let kk = i_j_to_kk(i, j, mesh_kx, mesh_ky, true);
            let eigens = diag(kk, param, spin_seq).eigenvalues();
            all_eigens.extend_from_slice(&eigens);
        }
    }

    all_eigens.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let total_kpoints = mesh_kx * mesh_ky;
    let total_states = all_eigens.len();
    let bands_per_k = total_states / total_kpoints;

    assert_eq!(
        total_states,
        total_kpoints * bands_per_k,
        "固有値数とバンド数が一致しません"
    );

    let mut energy_vs_n = Vec::with_capacity(101); // n=0.0〜1.0（101個）

    // nを0.0から1.0まで0.01刻みでループ
    for step in 0..=100 {
        let n = step as f64 / 100.0;

        // 電子数に対応する占有状態数（スピン縮重なしの場合）
        let num_occupied_states = ((n * total_states as f64 * 0.5).round()) as usize;

        let energy_sum: f64 = all_eigens
            .iter()
            .take(num_occupied_states)
            .sum();

        energy_vs_n.push(energy_sum / mesh_kx as f64 / mesh_ky as f64);
    }

    energy_vs_n
}



pub fn i_j_to_kk(i : usize, j : usize, mesh_kx : usize, mesh_ky : usize, hex : bool,) -> Vector2<f64>{
    let dv2_k1 = DV2::from_car(KPPKS) - DV2::from_car(-KINKS);
    let dv2_k2 = DV2::from_car(KP_KS) - DV2::from_car(-KINKS);

    let if64 = i as f64 / mesh_kx as f64;
    let jf64 = j as f64 / mesh_ky as f64;

    let mut kk_dv2 = dv2_k1 * if64 + dv2_k2 * jf64 + DV2::from_car(-KINKS);

    if hex{
        if point_in_triangle_simple(kk_dv2.to_car(), KINKS, 2. * KINKS, KPPKS){
            kk_dv2 = kk_dv2 - dv2_k1;
        }
        else if point_in_triangle_simple(kk_dv2.to_car(), KINKS, 2. * KINKS, KP_KS){
            kk_dv2 = kk_dv2 - dv2_k2;
        }
    }

    let kx = kk_dv2.to_car().x;
    let ky = kk_dv2.to_car().y;

    Vector2::new(kx,ky)
}


pub fn point_in_triangle_simple(
    x: Vector2<f64>,
    a: Vector2<f64>,
    b: Vector2<f64>,
    c: Vector2<f64>,
) -> bool {
    const EPSILON: f64 = 0.;

    let cross_ab_ax = (b.x - a.x) * (x.y - a.y) - (b.y - a.y) * (x.x - a.x);
    let cross_bc_bx = (c.x - b.x) * (x.y - b.y) - (c.y - b.y) * (x.x - b.x);
    let cross_ca_cx = (a.x - c.x) * (x.y - c.y) - (a.y - c.y) * (x.x - c.x);

    let all_non_negative = cross_ab_ax >= -EPSILON 
                        && cross_bc_bx >= -EPSILON 
                        && cross_ca_cx >= -EPSILON;

    let all_non_positive = cross_ab_ax <= EPSILON 
                        && cross_bc_bx <= EPSILON 
                        && cross_ca_cx <= EPSILON;

    all_non_negative || all_non_positive
}