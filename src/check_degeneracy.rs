//6サイト周期の磁気構造を網羅できているのかを総当たりでチェックする
use crate::model::{Param, SpinSeq};
use crate::cal_e::cal_e_vs_n;

use std::time::Instant;


pub fn check_degeneracy(){
    let param = Param::new(0.3, 0.25, 0.0);
    let graph_mesh = 2; // Mesh size for k-space sampling

    let mut all_spins_evec = Vec::new();
    for i in 0..64 {
        let start = Instant::now();
        // iの各ビットを調べて、0なら-1.0、1なら1.0を割り当てる
        let spin_seq = SpinSeq {
            // iの0番目のビットを確認 (iを0ビット右シフトし、1とAND演算)
            a: if (i >> 0) & 1 == 0 { -1.0 } else { 1.0 },
            // iの1番目のビットを確認
            b: if (i >> 1) & 1 == 0 { -1.0 } else { 1.0 },
            // iの2番目のビットを確認
            c: if (i >> 2) & 1 == 0 { -1.0 } else { 1.0 },
            // iの3番目のビットを確認
            d: if (i >> 3) & 1 == 0 { -1.0 } else { 1.0 },
            // iの4番目のビットを確認
            e: if (i >> 4) & 1 == 0 { -1.0 } else { 1.0 },
            // iの5番目のビットを確認
            f: if (i >> 5) & 1 == 0 { -1.0 } else { 1.0 },
        };

        let energyvec = cal_e_vs_n(&param, &spin_seq, graph_mesh);

        all_spins_evec.push((energyvec,spin_seq.debug_ud()));

        let duration = start.elapsed();

        println!("Combination #{}: {}, {:?}", i, spin_seq.debug_ud(),duration);
    }

    let unique_num = count_unique_vectors(&all_spins_evec);

    println!("unique num : {} , {:?}",unique_num.len(),unique_num)
}

pub fn count_unique_vectors(vectors: &Vec<(Vec<f64>,String)>) -> Vec<&String> {
    // 許容誤差。必要に応じて調整してください。
    const TOLERANCE: f64 = 1e-9;
    
    let mut unique_vectors: Vec<&Vec<f64>> = Vec::new();
    let mut unique_vec_name: Vec<&String> = Vec::new();

    for v_candidate in vectors {
        let mut is_degenerate = false;
        // これまでに見つかったユニークなベクトルの中に、同じものがないか調べる
        for v_unique in &unique_vectors {
            if are_vectors_close(&v_candidate.0, v_unique, TOLERANCE) {
                is_degenerate = true;
                break; // 同じものが見つかったので、これ以上探す必要はない
            }
        }

        // どのユニークなベクトルとも一致しなかった場合、新しいユニークなベクトルとして追加
        if !is_degenerate {
            unique_vectors.push(&v_candidate.0);
            unique_vec_name.push(&v_candidate.1);
        }
    }

    unique_vec_name
}

fn are_vectors_close(v1: &Vec<f64>, v2: &Vec<f64>, tolerance: f64) -> bool {
    // 要素数が違えば明らかに異なる
    if v1.len() != v2.len() {
        return false;
    }

    // zipで2つのベクトルの要素をペアにして順番に比較
    for (a, b) in v1.iter().zip(v2.iter()) {
        // 差の絶対値が許容誤差より大きければ、異なるベクトルとみなす
        if (a - b).abs() > tolerance {
            return false;
        }
    }

    // 全ての要素が許容誤差内であれば、同じベクトルとみなす
    true
}