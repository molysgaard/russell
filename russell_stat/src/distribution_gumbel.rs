use crate::{ProbabilityDistribution, StrError, EULER, PI, SQRT_6};
use rand::Rng;
use rand_distr::{Distribution, Gumbel};

/// Defines the Gumbel / Type I Extreme Value Distribution (largest value)
pub struct DistributionGumbel {
    location: f64, // location: characteristic largest value
    scale: f64,    // scale: measure of dispersion of the largest value

    sampler: Gumbel<f64>, // sampler
}

impl DistributionGumbel {
    /// Creates a new Gumbel distribution
    ///
    /// # Input
    ///
    /// * `location` -- characteristic largest value
    /// * `scale` -- measure of dispersion of the largest value
    pub fn new(location: f64, scale: f64) -> Result<Self, StrError> {
        Ok(DistributionGumbel {
            location,
            scale,
            sampler: Gumbel::new(location, scale).map_err(|_| "invalid parameters")?,
        })
    }

    /// Creates a new Gumbel distribution given mean and standard deviation parameters
    ///
    /// # Input
    ///
    /// * `mu` -- mean μ
    /// * `sig` -- standard deviation σ
    pub fn new_from_mu_sig(mu: f64, sig: f64) -> Result<Self, StrError> {
        let scale = sig * SQRT_6 / PI;
        let location = mu - EULER * scale;
        Ok(DistributionGumbel {
            location,
            scale,
            sampler: Gumbel::new(location, scale).map_err(|_| "invalid parameters")?,
        })
    }
}

impl ProbabilityDistribution for DistributionGumbel {
    /// Implements the Probability Density Function (CDF)
    fn pdf(&self, x: f64) -> f64 {
        let mz = (self.location - x) / self.scale;
        f64::exp(mz) * f64::exp(-f64::exp(mz)) / self.scale
    }

    /// Implements the Cumulative Density Function (CDF)
    fn cdf(&self, x: f64) -> f64 {
        let mz = (self.location - x) / self.scale;
        f64::exp(-f64::exp(mz))
    }

    /// Returns the Mean
    fn mean(&self) -> f64 {
        self.location + EULER * self.scale
    }

    /// Returns the Variance
    fn variance(&self) -> f64 {
        self.scale * self.scale * PI * PI / 6.0
    }

    /// Generates a pseudo-random number belonging to this probability distribution
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        self.sampler.sample(rng)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{DistributionGumbel, ProbabilityDistribution, StrError};
    use russell_chk::assert_approx_eq;

    // Data from the following R-code (run with Rscript gumbel.R):
    /*
    # needs r-cran-evd
    library(evd)
    X <- seq(-3, 3, 0.5)
    U <- c(0, 0.5, 1) # location
    B <- c(0.5, 1, 2) # scale
    Y <- matrix(ncol=4)
    first <- TRUE
    for (u in U) {
        for (b in B) {
            pdf <- dgumbel(X, u, b)
            cdf <- pgumbel(X, u, b)
            for (i in 1:length(X)) {
                if (first) {
                    Y <- rbind(c(X[i], u, b, pdf[i], cdf[i]))
                    first <- FALSE
                } else {
                    Y <- rbind(Y, c(X[i], u, b, pdf[i], cdf[i]))
                }
            }
        }
    }
    write.table(format(Y, digits=15), "/tmp/gumbel.dat", row.names=FALSE, col.names=c("x","location","scale","pdf","cdf"), quote=FALSE)
    print("file </tmp/gumbel.dat> written")
    */

    #[test]
    fn gumbel_works() -> Result<(), StrError> {
        #[rustfmt::skip]
        // x location scale pdf cdf
        let data = [
            [ -3.00000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01, 5.01069574040119e-173, 6.21013648656614e-176],
            [ -2.50000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01,  1.04108542169913e-62,  3.50738919646464e-65],
            [ -2.00000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01,  2.12096079940856e-22,  1.94233760495641e-24],
            [ -1.50000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01,  7.60108500808873e-08,  1.89217869483829e-09],
            [ -1.00000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01,  9.13256284025583e-03,  6.17978989331093e-04],
            [ -5.00000000000000e-01,  0.00000000000000e+00,  5.00000000000000e-01,  3.58748157468034e-01,  6.59880358453125e-02],
            [  0.00000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01,  7.35758882342885e-01,  3.67879441171442e-01],
            [  5.00000000000000e-01,  0.00000000000000e+00,  5.00000000000000e-01,  5.09292760087165e-01,  6.92200627555346e-01],
            [  1.00000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01,  2.36409903186286e-01,  8.73423018493117e-01],
            [  1.50000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01,  9.47380193558158e-02,  9.51431992900453e-01],
            [  2.00000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01,  3.59664593934273e-02,  9.81851073061667e-01],
            [  2.50000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01,  1.33853993550710e-02,  9.93284702067841e-01],
            [  3.00000000000000e+00,  0.00000000000000e+00,  5.00000000000000e-01,  4.94523114602982e-03,  9.97524317392752e-01],
            [ -3.00000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  3.80054250404436e-08,  1.89217869483829e-09],
            [ -2.50000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  6.23657718766199e-05,  5.11929429867073e-06],
            [ -2.00000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  4.56628142012792e-03,  6.17978989331093e-04],
            [ -1.50000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  5.07071136099807e-02,  1.13142863804596e-02],
            [ -1.00000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  1.79374078734017e-01,  6.59880358453125e-02],
            [ -5.00000000000000e-01,  0.00000000000000e+00,  1.00000000000000e+00,  3.17041921077942e-01,  1.92295645547965e-01],
            [  0.00000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  3.67879441171442e-01,  3.67879441171442e-01],
            [  5.00000000000000e-01,  0.00000000000000e+00,  1.00000000000000e+00,  3.30704298890418e-01,  5.45239211892605e-01],
            [  1.00000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  2.54646380043582e-01,  6.92200627555346e-01],
            [  1.50000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  1.78506518513121e-01,  8.00010713004354e-01],
            [  2.00000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  1.18204951593143e-01,  8.73423018493117e-01],
            [  2.50000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  7.56161799174265e-02,  9.21193655175516e-01],
            [  3.00000000000000e+00,  0.00000000000000e+00,  1.00000000000000e+00,  4.73690096779079e-02,  9.51431992900453e-01],
            [ -3.00000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  2.53535568049904e-02,  1.13142863804596e-02],
            [ -2.50000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  5.32109999504495e-02,  3.04904134630622e-02],
            [ -2.00000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  8.96870393670086e-02,  6.59880358453125e-02],
            [ -1.50000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  1.27435210411518e-01,  1.20392262079830e-01],
            [ -1.00000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  1.58520960538971e-01,  1.92295645547965e-01],
            [ -5.00000000000000e-01,  0.00000000000000e+00,  2.00000000000000e+00,  1.77786373690972e-01,  2.76920334099909e-01],
            [  0.00000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  1.83939720585721e-01,  3.67879441171442e-01],
            [  5.00000000000000e-01,  0.00000000000000e+00,  2.00000000000000e+00,  1.78717673086091e-01,  4.58956069307664e-01],
            [  1.00000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  1.65352149445209e-01,  5.45239211892605e-01],
            [  1.50000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  1.47266157620177e-01,  6.23524916256800e-01],
            [  2.00000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  1.27323190021791e-01,  6.92200627555346e-01],
            [  2.50000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  1.07565858970122e-01,  7.50883476639395e-01],
            [  3.00000000000000e+00,  0.00000000000000e+00,  2.00000000000000e+00,  8.92532592565605e-02,  8.00010713004354e-01],
            [ -3.00000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01,  0.00000000000000e+00,  0.00000000000000e+00],
            [ -2.50000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01, 5.01069574040119e-173, 6.21013648656614e-176],
            [ -2.00000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01,  1.04108542169913e-62,  3.50738919646464e-65],
            [ -1.50000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01,  2.12096079940856e-22,  1.94233760495641e-24],
            [ -1.00000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01,  7.60108500808873e-08,  1.89217869483829e-09],
            [ -5.00000000000000e-01,  5.00000000000000e-01,  5.00000000000000e-01,  9.13256284025583e-03,  6.17978989331093e-04],
            [  0.00000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01,  3.58748157468034e-01,  6.59880358453125e-02],
            [  5.00000000000000e-01,  5.00000000000000e-01,  5.00000000000000e-01,  7.35758882342885e-01,  3.67879441171442e-01],
            [  1.00000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01,  5.09292760087165e-01,  6.92200627555346e-01],
            [  1.50000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01,  2.36409903186286e-01,  8.73423018493117e-01],
            [  2.00000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01,  9.47380193558158e-02,  9.51431992900453e-01],
            [  2.50000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01,  3.59664593934273e-02,  9.81851073061667e-01],
            [  3.00000000000000e+00,  5.00000000000000e-01,  5.00000000000000e-01,  1.33853993550710e-02,  9.93284702067841e-01],
            [ -3.00000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  1.37458827543355e-13,  4.15089692010905e-15],
            [ -2.50000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  3.80054250404436e-08,  1.89217869483829e-09],
            [ -2.00000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  6.23657718766199e-05,  5.11929429867073e-06],
            [ -1.50000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  4.56628142012792e-03,  6.17978989331093e-04],
            [ -1.00000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  5.07071136099807e-02,  1.13142863804596e-02],
            [ -5.00000000000000e-01,  5.00000000000000e-01,  1.00000000000000e+00,  1.79374078734017e-01,  6.59880358453125e-02],
            [  0.00000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  3.17041921077942e-01,  1.92295645547965e-01],
            [  5.00000000000000e-01,  5.00000000000000e-01,  1.00000000000000e+00,  3.67879441171442e-01,  3.67879441171442e-01],
            [  1.00000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  3.30704298890418e-01,  5.45239211892605e-01],
            [  1.50000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  2.54646380043582e-01,  6.92200627555346e-01],
            [  2.00000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  1.78506518513121e-01,  8.00010713004354e-01],
            [  2.50000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  1.18204951593143e-01,  8.73423018493117e-01],
            [  3.00000000000000e+00,  5.00000000000000e-01,  1.00000000000000e+00,  7.56161799174265e-02,  9.21193655175516e-01],
            [ -3.00000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  9.11576582238494e-03,  3.16816514905324e-03],
            [ -2.50000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  2.53535568049904e-02,  1.13142863804596e-02],
            [ -2.00000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  5.32109999504495e-02,  3.04904134630622e-02],
            [ -1.50000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  8.96870393670086e-02,  6.59880358453125e-02],
            [ -1.00000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  1.27435210411518e-01,  1.20392262079830e-01],
            [ -5.00000000000000e-01,  5.00000000000000e-01,  2.00000000000000e+00,  1.58520960538971e-01,  1.92295645547965e-01],
            [  0.00000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  1.77786373690972e-01,  2.76920334099909e-01],
            [  5.00000000000000e-01,  5.00000000000000e-01,  2.00000000000000e+00,  1.83939720585721e-01,  3.67879441171442e-01],
            [  1.00000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  1.78717673086091e-01,  4.58956069307664e-01],
            [  1.50000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  1.65352149445209e-01,  5.45239211892605e-01],
            [  2.00000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  1.47266157620177e-01,  6.23524916256800e-01],
            [  2.50000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  1.27323190021791e-01,  6.92200627555346e-01],
            [  3.00000000000000e+00,  5.00000000000000e-01,  2.00000000000000e+00,  1.07565858970122e-01,  7.50883476639395e-01],
            [ -3.00000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01,  0.00000000000000e+00,  0.00000000000000e+00],
            [ -2.50000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01,  0.00000000000000e+00,  0.00000000000000e+00],
            [ -2.00000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01, 5.01069574040119e-173, 6.21013648656614e-176],
            [ -1.50000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01,  1.04108542169913e-62,  3.50738919646464e-65],
            [ -1.00000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01,  2.12096079940856e-22,  1.94233760495641e-24],
            [ -5.00000000000000e-01,  1.00000000000000e+00,  5.00000000000000e-01,  7.60108500808873e-08,  1.89217869483829e-09],
            [  0.00000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01,  9.13256284025583e-03,  6.17978989331093e-04],
            [  5.00000000000000e-01,  1.00000000000000e+00,  5.00000000000000e-01,  3.58748157468034e-01,  6.59880358453125e-02],
            [  1.00000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01,  7.35758882342885e-01,  3.67879441171442e-01],
            [  1.50000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01,  5.09292760087165e-01,  6.92200627555346e-01],
            [  2.00000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01,  2.36409903186286e-01,  8.73423018493117e-01],
            [  2.50000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01,  9.47380193558158e-02,  9.51431992900453e-01],
            [  3.00000000000000e+00,  1.00000000000000e+00,  5.00000000000000e-01,  3.59664593934273e-02,  9.81851073061667e-01],
            [ -3.00000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  1.06048039970428e-22,  1.94233760495641e-24],
            [ -2.50000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  1.37458827543355e-13,  4.15089692010905e-15],
            [ -2.00000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  3.80054250404436e-08,  1.89217869483829e-09],
            [ -1.50000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  6.23657718766199e-05,  5.11929429867073e-06],
            [ -1.00000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  4.56628142012792e-03,  6.17978989331093e-04],
            [ -5.00000000000000e-01,  1.00000000000000e+00,  1.00000000000000e+00,  5.07071136099807e-02,  1.13142863804596e-02],
            [  0.00000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  1.79374078734017e-01,  6.59880358453125e-02],
            [  5.00000000000000e-01,  1.00000000000000e+00,  1.00000000000000e+00,  3.17041921077942e-01,  1.92295645547965e-01],
            [  1.00000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  3.67879441171442e-01,  3.67879441171442e-01],
            [  1.50000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  3.30704298890418e-01,  5.45239211892605e-01],
            [  2.00000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  2.54646380043582e-01,  6.92200627555346e-01],
            [  2.50000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  1.78506518513121e-01,  8.00010713004354e-01],
            [  3.00000000000000e+00,  1.00000000000000e+00,  1.00000000000000e+00,  1.18204951593143e-01,  8.73423018493117e-01],
            [ -3.00000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  2.28314071006396e-03,  6.17978989331093e-04],
            [ -2.50000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  9.11576582238494e-03,  3.16816514905324e-03],
            [ -2.00000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  2.53535568049904e-02,  1.13142863804596e-02],
            [ -1.50000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  5.32109999504495e-02,  3.04904134630622e-02],
            [ -1.00000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  8.96870393670086e-02,  6.59880358453125e-02],
            [ -5.00000000000000e-01,  1.00000000000000e+00,  2.00000000000000e+00,  1.27435210411518e-01,  1.20392262079830e-01],
            [  0.00000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  1.58520960538971e-01,  1.92295645547965e-01],
            [  5.00000000000000e-01,  1.00000000000000e+00,  2.00000000000000e+00,  1.77786373690972e-01,  2.76920334099909e-01],
            [  1.00000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  1.83939720585721e-01,  3.67879441171442e-01],
            [  1.50000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  1.78717673086091e-01,  4.58956069307664e-01],
            [  2.00000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  1.65352149445209e-01,  5.45239211892605e-01],
            [  2.50000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  1.47266157620177e-01,  6.23524916256800e-01],
            [  3.00000000000000e+00,  1.00000000000000e+00,  2.00000000000000e+00,  1.27323190021791e-01,  6.92200627555346e-01],
        ];
        for row in data {
            let [x, location, scale, pdf, cdf] = row;
            let d = DistributionGumbel::new(location, scale)?;
            assert_approx_eq!(d.pdf(x), pdf, 1e-14);
            assert_approx_eq!(d.cdf(x), cdf, 1e-14);
        }
        Ok(())
    }

    #[test]
    fn new_from_mu_sig_works() -> Result<(), StrError> {
        // from Haldar & Mahadevan page 90
        let d = DistributionGumbel::new_from_mu_sig(61.3, 7.52)?;
        assert_approx_eq!(d.location, 57.9157, 0.00011);
        assert_approx_eq!(d.scale, 1.0 / 0.17055, 1e-4);
        Ok(())
    }

    #[test]
    fn mean_and_variance_work() -> Result<(), StrError> {
        let (mu, sig) = (1.0, 0.25);
        let d = DistributionGumbel::new_from_mu_sig(mu, sig)?;
        assert_approx_eq!(d.mean(), mu, 1e-14);
        assert_approx_eq!(d.variance(), sig * sig, 1e-14);
        Ok(())
    }
}