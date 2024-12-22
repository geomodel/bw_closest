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
    }else{
        debug!("there no (yet) result_file <{}> for deleting", result_file);
    }

    let g = plib::Grid::new(i_max, j_max, k_max);

    let bw = plib::UpscdProperty::<plib::Continuous>::from_file(bw_file)?;
    let mut result = plib::Property::<plib::Continuous>::new(g.get_size());

    for grid_index in 0..g.get_size() {
        //let plib::IJK{i, j, k} = g.index_to_coord(grid_index).unwrap();
        //let plib::IJK{i, j, k} = g.index_to_coord(grid_index).unwrap();
        //new_value = Some((1000 + (1+i)*100 + (1+j)*10 + (1+k)) as plib::Continuous);

        let coord = g.index_to_coord(grid_index).unwrap();
        let nearest_value = find_nearest(coord, &bw, &k_mult);

        result[grid_index] = Some(nearest_value);
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
    let mut dist = calc_distance(&coord, &nearest.0, &k_mult);

    for bw_index in 1..bw.len() {
        let alt_bw = bw.get_via_index(bw_index);
        let alt_dist = calc_distance(&coord, &alt_bw.0, &k_mult);
        if alt_dist < dist {
            nearest = alt_bw;
            dist = alt_dist;
        }
    }

    return nearest.1.clone();
}

fn calc_distance(a: &plib::IJK, b: &plib::IJK, k_mult: &f64) -> f64 {
    let di = (a.i - b.i) as f64;
    let dj = (a.j - b.j) as f64;
    let dk = k_mult * (a.k - b.k) as f64;

    return (di*di + dj*dj + dk*dk).sqrt();
}
