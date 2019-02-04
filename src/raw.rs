extern crate libc;

use std::os::raw::{c_char, c_int};

#[link(name = "libsolv")]
extern "C" {
    pub fn pool_create() -> *mut Pool;
    pub fn pool_free(pool: *mut Pool);

    pub fn repo_create(pool: *mut Pool, name: *const c_char) -> *mut Repo;
    pub fn repo_free(repo: *mut Repo, reuseids: c_int);
}

pub enum Pool {}

pub enum Repo {}
