extern crate libc;

use libc::FILE;
use std::os::raw::{c_char, c_int};

#[link(name = "solv")]
extern "C" {
    pub fn pool_create() -> *mut Pool;
    pub fn pool_free(pool: *mut Pool);

    pub fn pool_setarch(pool: *mut Pool, arch: *const c_char);

    // repo_solv.h
    pub fn repo_add_solv(repo: *mut Repo, fp: *mut FILE, flags: c_int) -> c_int;

    pub fn repo_create(pool: *mut Pool, name: *const c_char) -> *mut Repo;
    pub fn repo_free(repo: *mut Repo, reuseids: c_int);
}

pub enum Pool {}

pub enum Repo {}
