use crate::consts::*;
use crate::diag::{diag};
use crate::model::{Param, SpinSeq};

use nalgebra::{Vector2,};
use std::fs;

//--------------------------------------------------------------------
//                          バンド図<->波数空間                    
//--------------------------------------------------------------------

//バンド図における横軸から対応するk空間のベクトルを返す
fn band_2_kk (x : f64) -> Vector2<f64> {
    if x < G_X_B{
        {
            let div = (x - MPX_B) / (G_X_B - MPX_B);
            MP_KS * (1. - div) + GAMMA * div
        }
    }
    else if  x < M_X_B{
        {
            let div = (x - G_X_B) / (M_X_B - G_X_B);
            GAMMA * (1. - div) + MINKS * div
        }
    }
    else if x < K_X_B{
        {
            let div = (x - M_X_B) / (K_X_B - M_X_B);
            MINKS * (1. - div) + KINKS * div
        }
    }
    else{
        {
            let div = (x - K_X_B) / (G2X_B - K_X_B);
            KINKS * (1. - div) + GAMMA * div
        }
    }
}

//--------------------------------------------------------------------
//                          処理                   
//--------------------------------------------------------------------

pub fn export_band_dat_uuuddd(param: &Param, spin_seq: &SpinSeq, comment : &str) {
    let graph_div = 1000;

    let mut file_str = "x u1 u2 u3 u4 u5 u6 d1 d2 d3 d4 d5 d6".to_string();

    for i in 0..(graph_div + 1){
        let per = i as f64 / graph_div as f64;

        let x = per * MPX_B + (1. - per) * G2X_B;

        let seud = diag(band_2_kk(x), param, spin_seq).sort();

        file_str = format!(
            "{}\n{} {} {} {} {} {} {} {} {} {} {} {} {}",
            file_str,
            x,
            seud.u.eigenvalues[0],
            seud.u.eigenvalues[1],
            seud.u.eigenvalues[2],
            seud.u.eigenvalues[3],
            seud.u.eigenvalues[4],
            seud.u.eigenvalues[5],
            seud.d.eigenvalues[0],
            seud.d.eigenvalues[1],
            seud.d.eigenvalues[2],
            seud.d.eigenvalues[3],
            seud.d.eigenvalues[4],
            seud.d.eigenvalues[5],
        );
    }
    fs::write(format!("./output/bands/dats/kk/band_kk_{}_{}_{}.dat",param.debug(),spin_seq.debug(),comment), file_str).unwrap();

}