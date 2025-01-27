use crate::{matrix_data_t, Matrix};
use stdint::uint_fast8_t;

/// Kalman Filter measurement structure.
#[allow(non_snake_case, unused)]
pub struct Measurement<'a> {
    /// The number of states.
    pub num_states: uint_fast8_t,
    /// The number of measurements.
    pub num_measurements: uint_fast8_t,
    /// Measurement vector.
    pub(crate) z: Matrix<'a>,
    /// Measurement transformation matrix.
    ///
    /// See also [`R`].
    pub(crate) H: Matrix<'a>,
    /// Process noise covariance matrix.
    ///
    /// See also [`A`].
    pub(crate) R: Matrix<'a>,
    /// Innovation vector.
    pub(crate) y: Matrix<'a>,
    /// Residual covariance matrix.
    pub(crate) S: Matrix<'a>,
    /// Kalman gain matrix.
    pub(crate) K: Matrix<'a>,

    /// Temporary storage.
    pub(crate) temporary: MeasurementTemporary<'a>,
}

#[allow(non_snake_case)]
pub(crate) struct MeasurementTemporary<'a> {
    /// S-Sized temporary matrix  (number of measurements × number of measurements).
    ///
    /// - The backing field for this temporary MAY be aliased with temporary [`KHP`].
    /// - The backing field for this temporary MAY be aliased with temporary [`HP`] (if it is not aliased with [`PHt`]).
    /// - The backing field for this temporary MUST NOT be aliased with temporary [`PHt`].
    pub(crate) S_inv: Matrix<'a>,
    /// H-Sized temporary matrix  (number of measurements × number of states).
    ///
    /// - The backing field for this temporary MAY be aliased with temporary [`S_inv`].
    /// - The backing field for this temporary MAY be aliased with temporary [`PHt`].
    /// - The backing field for this temporary MUST NOT be aliased with temporary [`KHP`].
    pub(crate) HP: Matrix<'a>,
    /// P-Sized temporary matrix  (number of states × number of states).
    ///
    /// - The backing field for this temporary MAY be aliased with temporary [`S_inv`].
    /// - The backing field for this temporary MAY be aliased with temporary [`PHt`].
    /// - The backing field for this temporary MUST NOT be aliased with temporary [`HP`].
    pub(crate) KHP: Matrix<'a>,
    /// P×H'-Sized (H'-Sized) temporary matrix  (number of states × number of measurements).
    ///
    /// - The backing field for this temporary MAY be aliased with temporary [`HP`].
    /// - The backing field for this temporary MAY be aliased with temporary [`KHP`].
    /// - The backing field for this temporary MUST NOT be aliased with temporary [`S_inv`].
    pub(crate) PHt: Matrix<'a>,
}

impl<'a> Measurement<'a> {
    /// Initializes a measurement.
    ///
    /// ## Arguments
    /// * `num_states` - The number of states tracked by the filter.
    /// * `num_measurements` - The number of measurements available to the filter.
    /// * `H` - The measurement transformation matrix (`num_measurements` × `num_states`).
    /// * `z` - The measurement vector (`num_measurements` × `1`).
    /// * `R` - The process noise / measurement uncertainty (`num_measurements` × `num_measurements`).
    /// * `v` - The innovation (`num_measurements` × `1`).
    /// * `S` - The residual covariance (`num_measurements` × `num_measurements`).
    /// * `K` - The Kalman gain (`num_states` × `num_measurements`).
    /// * `S_inv` - The temporary vector for predicted states (`num_states` × `1`).
    /// * `temp_HP` - The temporary matrix for H×P calculation (`num_measurements` × `num_states`).
    /// * `temp_PHt` - The temporary matrix for P×H' calculation (`num_states` × `num_measurements`).
    /// * `temp_KHP` - The temporary matrix for K×H×P calculation (`num_states` × `num_states`).
    #[allow(non_snake_case)]
    #[doc(alias = "kalman_measurement_initialize")]
    pub fn new_direct(
        num_states: uint_fast8_t,
        num_measurements: uint_fast8_t,
        H: &'a mut [matrix_data_t],
        z: &'a mut [matrix_data_t],
        R: &'a mut [matrix_data_t],
        y: &'a mut [matrix_data_t],
        S: &'a mut [matrix_data_t],
        K: &'a mut [matrix_data_t],
        S_inv: &'a mut [matrix_data_t],
        temp_HP: &'a mut [matrix_data_t],
        temp_PHt: &'a mut [matrix_data_t],
        temp_KHP: &'a mut [matrix_data_t],
    ) -> Self {
        Self {
            num_states,
            num_measurements,
            H: Matrix::new(num_measurements, num_states, H),
            R: Matrix::new(num_measurements, num_measurements, R),
            z: Matrix::new(num_measurements, 1, z),
            K: Matrix::new(num_states, num_measurements, K),
            S: Matrix::new(num_measurements, num_measurements, S),
            y: Matrix::new(num_measurements, 1, y),
            temporary: MeasurementTemporary {
                S_inv: Matrix::new(num_measurements, num_measurements, S_inv),
                HP: Matrix::new(num_measurements, num_states, temp_HP),
                PHt: Matrix::new(num_states, num_measurements, temp_PHt),
                KHP: Matrix::new(num_states, num_states, temp_KHP),
            },
        }
    }

    /// Initializes a measurement.
    ///
    /// ## Arguments
    /// * `num_states` - The number of states tracked by the filter.
    /// * `num_measurements` - The number of measurements available to the filter.
    /// * `H` - The measurement transformation matrix (`num_measurements` × `num_states`).
    /// * `z` - The measurement vector (`num_measurements` × `1`).
    /// * `R` - The process noise / measurement uncertainty (`num_measurements` × `num_measurements`).
    /// * `y` - The innovation (`num_measurements` × `1`).
    /// * `S` - The residual covariance (`num_measurements` × `num_measurements`).
    /// * `K` - The Kalman gain (`num_states` × `num_measurements`).
    /// * `S_inv` - The temporary matrix for S-inverted (`num_measurements` × `num_measurements`).
    /// * `temp_HP` - The temporary matrix for H×P calculation (`num_measurements` × `num_states`).
    /// * `temp_PHt` - The temporary matrix for P×H' calculation (`num_states` × `num_measurements`).
    /// * `temp_KHP` - The temporary matrix for K×H×P calculation (`num_states` × `num_states`).
    #[allow(non_snake_case)]
    #[doc(alias = "kalman_measurement_initialize")]
    pub fn new(
        num_states: uint_fast8_t,
        num_measurements: uint_fast8_t,
        H: Matrix<'a>,
        z: Matrix<'a>,
        R: Matrix<'a>,
        y: Matrix<'a>,
        S: Matrix<'a>,
        K: Matrix<'a>,
        S_inv: Matrix<'a>,
        temp_HP: Matrix<'a>,
        temp_PHt: Matrix<'a>,
        temp_KHP: Matrix<'a>,
    ) -> Self {
        debug_assert_eq!(
            H.rows, num_measurements,
            "The measurement transformation matrix H requires {} rows and {} columns (i.e. measurements × states)",
            num_measurements, num_states
        );
        debug_assert_eq!(
            H.cols, num_states,
            "The measurement transformation matrix H requires {} rows and {} columns (i.e. measurements × states)",
            num_measurements, num_states
        );

        debug_assert_eq!(
            z.rows, num_measurements,
            "The measurement vector z requires {} rows and 1 column (i.e. measurements × 1)",
            num_measurements
        );
        debug_assert_eq!(
            z.cols, 1,
            "The measurement vector z requires {} rows and 1 column (i.e. measurements × 1)",
            num_measurements
        );

        debug_assert_eq!(
            R.rows, num_measurements,
            "The process noise / measurement uncertainty matrix R requires {} rows and {} columns (i.e. measurements × measurements)",
            num_measurements, num_measurements
        );
        debug_assert_eq!(
            R.cols, num_measurements,
            "The process noise / measurement uncertainty matrix R requires {} rows and {} columns (i.e. measurements × measurements)",
            num_measurements, num_measurements
        );

        debug_assert_eq!(
            y.rows, num_measurements,
            "The innovation vector y requires {} rows and 1 column (i.e. measurements × 1)",
            num_measurements
        );
        debug_assert_eq!(
            y.cols, 1,
            "The innovation vector y requires {} rows and 1 column (i.e. measurements × 1)",
            num_measurements
        );

        debug_assert_eq!(
            S.rows, num_measurements,
            "The residual covariance matrix S requires {} rows and {} columns (i.e. measurements × measurements)",
            num_measurements, num_measurements
        );
        debug_assert_eq!(
            S.cols, num_measurements,
            "The residual covariance S requires {} rows and {} columns (i.e. measurements × measurements)",
            num_measurements, num_measurements
        );

        debug_assert_eq!(
            K.rows, num_states,
            "The Kalman gain matrix S requires {} rows and {} columns (i.e. states × measurements)",
            num_states, num_measurements
        );
        debug_assert_eq!(
            K.cols, num_measurements,
            "The Kalman gain matrix K requires {} rows and {} columns (i.e. states × measurements)",
            num_states, num_measurements
        );

        debug_assert_eq!(
            S_inv.rows, num_measurements,
            "The temporary S-inverted matrix requires {} rows and {} columns (i.e. measurements × measurements)",
            num_measurements, num_measurements
        );
        debug_assert_eq!(
            S_inv.cols, num_measurements,
            "The temporary S-inverted matrix requires {} rows and {} columns (i.e. measurements × measurements)",
            num_measurements, num_measurements
        );

        debug_assert_eq!(
            temp_HP.rows, num_measurements,
            "The temporary H×P calculation matrix requires {} rows and {} columns (i.e. measurements × measurements)",
            num_measurements, num_states
        );
        debug_assert_eq!(
            temp_HP.cols, num_states,
            "The temporary H×P calculation matrix requires {} rows and {} columns (i.e. measurements × measurements)",
            num_measurements, num_states
        );

        debug_assert_eq!(
            temp_PHt.rows, num_states,
            "The temporary P×H' calculation matrix requires {} rows and {} columns (i.e. states × measurements)",
            num_states, num_measurements
        );
        debug_assert_eq!(
            temp_PHt.cols, num_measurements,
            "The temporary P×H' calculation matrix requires {} rows and {} columns (i.e. states × measurements)",
            num_states, num_measurements
        );

        debug_assert_eq!(
            temp_KHP.rows, num_states,
            "The temporary K×H×P calculation matrix requires {} rows and {} columns (i.e. states × states)",
            num_states, num_states
        );
        debug_assert_eq!(
            temp_KHP.cols, num_states,
            "The temporary K×H×P calculation matrix requires {} rows and {} columns (i.e. states × states)",
            num_states, num_states
        );

        Self {
            num_states,
            num_measurements,
            H,
            R,
            z,
            K,
            S,
            y,
            temporary: MeasurementTemporary {
                S_inv,
                HP: temp_HP,
                PHt: temp_PHt,
                KHP: temp_KHP,
            },
        }
    }

    /// Gets a reference to the measurement vector z.
    #[inline(always)]
    pub fn measurement_vector_ref(&self) -> &Matrix {
        &self.z
    }

    /// Gets a mutable reference to the measurement vector z.
    #[inline(always)]
    #[doc(alias = "kalman_get_measurement_vector")]
    pub fn measurement_vector_mut(&'a mut self) -> &'a mut Matrix {
        &mut self.z
    }

    /// Applies a function to the measurement vector z.
    #[inline(always)]
    pub fn measurement_vector_apply<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Matrix<'a>) -> (),
    {
        f(&mut self.z)
    }

    /// Gets a reference to the measurement transformation matrix H.
    #[inline(always)]
    pub fn measurement_transformation_ref(&self) -> &Matrix {
        &self.H
    }

    /// Gets a mutable reference to the measurement transformation matrix H.
    #[inline(always)]
    #[doc(alias = "kalman_get_measurement_transformation")]
    pub fn measurement_transformation_mut(&'a mut self) -> &'a mut Matrix {
        &mut self.H
    }

    /// Applies a function to the measurement transformation matrix H.
    #[inline(always)]
    pub fn measurement_transformation_apply<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Matrix<'a>) -> (),
    {
        f(&mut self.H)
    }

    /// Gets a reference to the process noise matrix R.
    #[inline(always)]
    pub fn process_noise_ref(&self) -> &Matrix {
        &self.R
    }

    /// Gets a mutable reference to the process noise matrix R.
    #[inline(always)]
    #[doc(alias = "kalman_get_process_noise")]
    pub fn process_noise_mut(&'a mut self) -> &'a mut Matrix {
        &mut self.R
    }

    /// Applies a function to the process noise matrix R.
    #[inline(always)]
    pub fn process_noise_apply<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Matrix<'a>) -> (),
    {
        f(&mut self.R)
    }
}
