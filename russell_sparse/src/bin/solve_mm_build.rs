use russell_lab::{format_nanoseconds, Stopwatch, StrError, Vector};
use russell_openblas::set_num_threads;
use russell_sparse::{
    enum_ordering, enum_scaling, read_matrix_market, ConfigSolver, LinSolKind, Solver, Symmetry, VerifyLinSys,
};
use std::path::Path;
use structopt::StructOpt;

/// Command line options
#[derive(StructOpt, Debug)]
#[structopt(
    name = "solve_matrix_market",
    about = "Solve a linear system with a Matrix-Market file."
)]
struct Options {
    /// Matrix-market file
    matrix_market_file: String,

    /// Use MMP solver instead of UMF
    #[structopt(short, long)]
    mmp: bool,

    /// Ordering strategy
    #[structopt(short = "o", long, default_value = "Auto")]
    ordering: String,

    /// Scaling strategy
    #[structopt(short = "s", long, default_value = "Auto")]
    scaling: String,

    /// Number of threads for OpenMP
    #[structopt(short = "n", long, default_value = "1")]
    omp_nt: u32,

    /// Activate verbose mode
    #[structopt(short = "v", long)]
    verbose: bool,
}

fn main() -> Result<(), StrError> {
    // parse options
    let opt = Options::from_args();

    // set openblas num of threads to 1
    if opt.omp_nt > 1 {
        set_num_threads(1);
    }

    // select linear solver
    let name = if opt.mmp { LinSolKind::Mmp } else { LinSolKind::Umf };

    // set the sym_mirror flag
    let sym_mirror = match name {
        LinSolKind::Mmp => {
            // MMP uses the lower-diagonal if symmetric.
            false
        }
        LinSolKind::Umf => {
            // UMF uses the full matrix, if symmetric or not
            true
        }
    };

    // read matrix
    let mut sw = Stopwatch::new("");
    let (trip, symmetric) = read_matrix_market(&opt.matrix_market_file, sym_mirror)?;
    let time_read = sw.stop();

    // set the symmetry option
    let symmetry = if symmetric { Some(Symmetry::General) } else { None };

    // set configuration
    let mut config = ConfigSolver::new();
    config
        .lin_sol_kind(name)
        .ordering(enum_ordering(opt.ordering.as_str()))
        .scaling(enum_scaling(opt.scaling.as_str()));
    if opt.omp_nt > 1 {
        config.openmp_num_threads(opt.omp_nt as usize);
    }
    if opt.verbose {
        config.verbose();
    }

    // initialize and factorize
    let (neq, nnz) = (trip.neq(), trip.nnz_current());
    let mut solver = Solver::new(config, neq, nnz, symmetry)?;
    solver.factorize(&trip)?;

    // allocate vectors
    let mut x = Vector::new(neq);
    let rhs = Vector::filled(neq, 1.0);

    // solve linear system
    solver.solve(&mut x, &rhs)?;

    // verify solution
    let triangular = symmetric && !sym_mirror;
    let verify = VerifyLinSys::new(&trip, &x, &rhs, triangular)?;

    // matrix name
    let path = Path::new(&opt.matrix_market_file);
    let matrix_name = path.file_stem().unwrap().to_str().unwrap();

    // output
    println!(
        "{{\n\
            \x20\x20\"platform\": \"russell\",\n\
            \x20\x20\"blasLib\": \"OpenBLAS\",\n\
            \x20\x20\"matrixName\": \"{}\",\n\
            \x20\x20\"read\": {{\n\
                \x20\x20\x20\x20\"timeReadNs\": {},\n\
                \x20\x20\x20\x20\"timeReadStr\": \"{}\"\n\
            \x20\x20}},\n\
            \x20\x20\"triplet\": {{\n\
                {}\n\
            \x20\x20}},\n\
            \x20\x20\"symmetry\": \"{:?}\"\n\
            \x20\x20\"solver\": {{\n\
                {}\n\
                {}\n\
            \x20\x20}},\n\
            \x20\x20\"verify\": {{\n\
                {}\n\
            \x20\x20}}\n\
        }}",
        matrix_name,
        time_read,
        format_nanoseconds(time_read),
        trip,
        symmetry,
        config,
        solver,
        verify
    );

    // check
    if path.ends_with("bfwb62.mtx") {
        let tolerance = if opt.mmp { 1e-10 } else { 1e-15 };
        let correct_x = get_bfwb62_correct_x();
        for i in 0..neq {
            let diff = f64::abs(x.get(i) - correct_x.get(i));
            if diff > tolerance {
                println!("ERROR: diff({}) = {}", i, diff);
            }
        }
    }

    // done
    Ok(())
}

fn get_bfwb62_correct_x() -> Vector {
    Vector::from(&[
        -1.02570048377040759e+05,
        -1.08800418159713998e+05,
        -7.87848688672370918e+04,
        -6.12550631774225840e+04,
        -1.16611533352550643e+05,
        -8.91949258261042705e+04,
        -5.57584825429375196e+04,
        -3.37535346291137103e+04,
        -6.74159236038033268e+04,
        -5.61065283435406673e+04,
        -3.69561341372605821e+04,
        -2.67385128650871302e+04,
        -4.67349124343154253e+04,
        -4.18861901056076676e+04,
        -4.34393771636046149e+04,
        -1.11210692731083000e+04,
        -1.16010526640020762e+04,
        -4.31993854681577286e+04,
        -5.82924327463857844e+03,
        -2.42374319876188747e+04,
        -2.39432136682168457e+04,
        5.27355041927211232e+02,
        -1.24769422505944240e+04,
        -1.47005934749971748e+04,
        -4.95701604733381391e+04,
        -1.38451884223610182e+03,
        -1.57972501695015781e+04,
        -5.19172705598900066e+04,
        -4.99494464999615593e+04,
        -1.19678659380488571e+04,
        -1.56190973892000347e+04,
        -6.18809904102459404e+03,
        -1.05693761694190998e+04,
        -2.93013328593191145e+04,
        -9.15514607143451940e+03,
        -1.27058094439569140e+04,
        -1.93936053067287430e+04,
        -6.84836276779992295e+03,
        -1.07869319688850719e+04,
        -4.61926223513438963e+04,
        -1.99579363156562504e+04,
        -7.83564896339727693e+03,
        -6.37173129434054590e+03,
        -1.88075622025074267e+03,
        -8.71648101674354621e+03,
        -1.21683775603205122e+04,
        -1.91184585274694587e+03,
        -5.64233479410600103e+03,
        -6.47747230904305070e+03,
        -4.47783973932844674e+03,
        -9.82971659947420812e+03,
        -1.95594295004403466e+04,
        -2.09457080830507803e+04,
        -5.46686114796283709e+03,
        -5.28888244321673483e+03,
        -2.07962090362636227e+04,
        -9.33272319073228937e+03,
        1.96672299472196187e+02,
        -4.40813445835840230e+03,
        -4.87188111893421956e+03,
        -1.75640594405328884e+04,
        -1.77959327708208002e+04,
    ])
}
