use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

//  //  //  //  //  //  //  //
use properties3d as plib;

pub(crate) fn invoke(
    i_max: usize,
    j_max: usize,
    k_max: usize,
    actnum_file: Option<&str>,
    bw_file: &str,
    result_file: &str,
    undef_value: &str,
    k_mult: f64,
) -> Result<()> {
    trace!(
        "params: {}x{}x{} ({}) - {:?} -> {} => {} ({})",
        i_max,
        j_max,
        k_max,
        k_mult,
        actnum_file,
        bw_file,
        result_file,
        undef_value
    );
    if let Ok(()) = std::fs::remove_file(result_file) {
        debug!("old result_file <{}> has been deleted", result_file);
    } else {
        debug!("there no (yet) result_file <{}> for deleting", result_file);
    }

    let g = plib::Grid::new(i_max, j_max, k_max);

    let bw = plib::UpscdProperty::<plib::Continuous>::from_file(bw_file)?;
    let actnum = match actnum_file {
        None => None,
        Some(file_name) => Some(plib::ActnumProperty::from_file(file_name, g.get_size())?),
    };
    let mut result = plib::Property::<plib::Continuous>::new(g.get_size());

    for grid_index in 0..g.get_size() {
        let is_cell_defined = match actnum {
            None => true,
            Some(ref act) => {
                if act[grid_index] == true {
                    true
                }else{
                    false
                }
            },
        };
        if is_cell_defined == true {
            let coord = g.index_to_coord(grid_index).unwrap();
            let nearest_value = find_nearest(coord, &bw, &k_mult);
            result[grid_index] = Some(nearest_value);
        }else{
            result[grid_index] = None;
        }
    }

    result.save_to_file(result_file, undef_value)?;
    Ok(())
}

//  //  //  //  //  //  //  //
fn find_nearest<T>(coord: plib::IJK, bw: &plib::UpscdProperty<T>, k_mult: &f64) -> T
where
    T: std::str::FromStr + Clone,
{
    let mut nearest = bw.get_via_index(0);
    let mut dist = distance(&coord, &nearest.0, &k_mult);

    for bw_index in 1..bw.len() {
        let alt_bw = bw.get_via_index(bw_index);
        let alt_dist = distance(&coord, &alt_bw.0, &k_mult);
        if alt_dist < dist {
            nearest = alt_bw;
            dist = alt_dist;
        }
    }

    return nearest.1.clone();
}

fn distance(a: &plib::IJK, b: &plib::IJK, k_mult: &f64) -> f64 {
    let di = a.i as f64 - b.i as f64;
    let dj = a.j as f64 - b.j as f64;
    let dk = a.k as f64 - b.k as f64;

    return (di * di + dj * dj + k_mult * k_mult * dk * dk).sqrt();
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod calculus {
    use super::*;

    #[test]
    fn one_k_mult_10() {
        let a = plib::IJK { i: 0, j: 0, k: 0 };
        let b = plib::IJK { i: 0, j: 0, k: 1 };
        let dist_ab = distance(&a, &b, &10.0);
        let dist_ba = distance(&b, &a, &10.0);
        assert!(dist_ab == 10.0);
        assert!(dist_ba == 10.0);
    }

    #[test]
    fn two_k() {
        let a = plib::IJK { i: 0, j: 0, k: 0 };
        let b = plib::IJK { i: 0, j: 0, k: 2 };
        let dist_ab = distance(&a, &b, &1.0);
        let dist_ba = distance(&b, &a, &1.0);
        assert!(dist_ab == 2.0);
        assert!(dist_ba == 2.0);
    }
    #[test]
    fn two_j() {
        let a = plib::IJK { i: 0, j: 0, k: 0 };
        let b = plib::IJK { i: 0, j: 2, k: 0 };
        let dist_ab = distance(&a, &b, &1.0);
        let dist_ba = distance(&b, &a, &1.0);
        assert!(dist_ab == 2.0);
        assert!(dist_ba == 2.0);
    }
    #[test]
    fn two_i() {
        let a = plib::IJK { i: 0, j: 0, k: 0 };
        let b = plib::IJK { i: 2, j: 0, k: 0 };
        let dist_ab = distance(&a, &b, &1.0);
        let dist_ba = distance(&b, &a, &1.0);
        assert!(dist_ab == 2.0);
        assert!(dist_ba == 2.0);
    }

    #[test]
    fn zero_1() {
        let a = plib::IJK { i: 1, j: 1, k: 1 };
        let b = plib::IJK { i: 1, j: 1, k: 1 };
        let dist_ab = distance(&a, &b, &1.0);
        let dist_ba = distance(&b, &a, &1.0);
        assert!(dist_ab == 0.0);
        assert!(dist_ba == 0.0);
    }
    #[test]
    fn zero_0() {
        let a = plib::IJK { i: 0, j: 0, k: 0 };
        let b = plib::IJK { i: 0, j: 0, k: 0 };
        let dist_ab = distance(&a, &b, &1.0);
        let dist_ba = distance(&b, &a, &1.0);
        assert!(dist_ab == 0.0);
        assert!(dist_ba == 0.0);
    }
}
