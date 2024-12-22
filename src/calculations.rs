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
) -> Result<()> {
    trace!(
        "params: {}x{}x{} - {:?} -> {} => {} ({})",
        i_max,
        j_max,
        k_max,
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
    let result = plib::Property::<plib::Continuous>::new(g.get_size());

    for grid_index in 0..g.get_size() {
    }

    result.save_to_file(result_file, undef_value)?;
    Ok(())
}
