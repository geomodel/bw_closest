use anyhow::Result;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

//  //  //  //  //  //  //  //
use properties3d as plib;

pub(crate) fn invoke(
    i_max: usize,
    j_max: usize,
    k_max: usize,
    actnum: Option<&str>,
    bw: &str,
    result: &str,
    undef_value: &str,
) -> Result<()> {
    trace!(
        "params: {}x{}x{} - {:?} -> {} => {} ({})",
        i_max, j_max, k_max, actnum, bw, result, undef_value
    );

    let g = plib::Grid::new(i_max, j_max, k_max);

    plib::UpscdProperty::<plib::Continuous>::from_file(bw)?;

    Ok(())
}
