extern crate libc;

use libc::*;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CblasOrder {
  RowMajor      = 101,
  ColMajor      = 102,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CblasTranspose {
  NoTrans       = 111,
  Trans         = 112,
  ConjTrans     = 113,
  ConjNoTrans   = 114,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CblasUpLo {
  Upper         = 121,
  Lower         = 122,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CblasDiag {
  NonUnit       = 131,
  Unit          = 132,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CblasSide {
  Left          = 141,
  Right         = 142,
}

// XXX: No explicit linkage.
extern "C" {
  pub fn cblas_snrm2(
      n:        c_int,
      x:        *const c_float,
      incx:     c_int,
  ) -> c_float;
  pub fn cblas_sdot(
      n:        c_int,
      alpha:    c_float,
      x:        *const c_float,
      incx:     c_int,
      y:        *const c_float,
      incy:     c_int,
  ) -> c_float;
  pub fn cblas_sscal(
      n:        c_int,
      alpha:    c_float,
      x:        *mut c_float,
      incx:     c_int,
  );
  pub fn cblas_saxpy(
      n:        c_int,
      alpha:    c_float,
      x:        *const c_float,
      incx:     c_int,
      y:        *mut c_float,
      incy:     c_int,
  );
  pub fn cblas_sgemv(
      order:    CblasOrder,
      trans:    CblasTranspose,
      m:        c_int,
      n:        c_int,
      alpha:    c_float,
      a:        *const c_float,
      lda:      c_int,
      x:        *const c_float,
      incx:     c_int,
      beta:     c_float,
      y:        *mut c_float,
      incy:     c_int,
  );
  pub fn cblas_sgemm(
      order:    CblasOrder,
      trans_a:  CblasTranspose,
      trans_b:  CblasTranspose,
      m:        c_int,
      n:        c_int,
      k:        c_int,
      alpha:    c_float,
      a:        *const c_float,
      lda:      c_int,
      b:        *const c_float,
      ldb:      c_int,
      beta:     c_float,
      c:        *mut c_float,
      ldc:      c_int,
  );
}
