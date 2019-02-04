mod raw;

use std::error;
use std::ffi::CString;
use std::fmt;
use std::marker::PhantomData;
use std::result;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        // Displaying an `Error` simply displays the pool's error string.
        self.message.fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(e: std::ffi::NulError) -> Error {
        Error {
            message: format!("interior nul byte found at {}", e.nul_position()).to_owned(),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub struct Pool {
    ptr: *mut raw::Pool,
}

impl Pool {
    pub fn new() -> Pool {
        unsafe {
            Pool {
                ptr: raw::pool_create(),
            }
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe { raw::pool_free(self.ptr) }
    }
}

pub struct Repository<'pool> {
    ptr: *mut raw::Repo,
    _marker: PhantomData<&'pool Pool>,
}

impl<'pool> Repository<'pool> {
    pub fn new(pool: &mut Pool, name: &str) -> Result<Repository<'pool>> {
        let name = CString::new(name)?;
        unsafe {
            let repo = raw::repo_create(pool.ptr, name.as_ptr());
            Ok(Repository {
                ptr: repo,
                _marker: PhantomData,
            })
        }
    }
}

impl<'pool> Drop for Repository<'pool> {
    fn drop(&mut self) {
        // Free the repo from the pool and don't reuse the IDs of the solvables.
        unsafe { raw::repo_free(self.ptr, 0) }
    }
}

pub enum DistType {
    Rpm = 0,
    Deb = 1,
    Arch = 2,
    Haiku = 3,
}
