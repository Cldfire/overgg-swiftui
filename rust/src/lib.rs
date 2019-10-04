#![allow(bad_style)]

extern crate overgg_scraper;
#[macro_use]
extern crate error_chain;

pub mod types;
pub mod error;
mod utils;

use error::*;
use utils::vec_to_ffi;
use overgg_scraper::http_client::Downloader;
use types::{
    downloaderWrapper,
    mainPageScraperWrapper,
    matchBriefTypeWrapper,
    matchBriefInfoWrapper
};
use std::os::raw::c_int;

// Downloader

/// Creates a new `Downloader` for you to use opaquely.
#[no_mangle]
pub extern "C" fn downloaderNew(inout_ptr: *mut *mut downloaderWrapper) {
    unsafe {
        *inout_ptr = Box::into_raw(Box::new(downloaderWrapper(Downloader::new())));
    }
}

#[no_mangle]
pub extern "C" fn downloaderGetMainPage(
    ptr: *const downloaderWrapper,
    inout_ptr: *mut *mut mainPageScraperWrapper
) -> i32 {
    let dlr = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let main_page = match dlr.main_page() {
        Ok(v) => v,
        Err(_) => return -1
    };

    unsafe {
        *inout_ptr = Box::into_raw(Box::new(mainPageScraperWrapper(main_page)));
    }

    0
}

#[no_mangle]
pub extern "C" fn downloaderFree(ptr: *mut downloaderWrapper) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

// MainPageScraper

#[no_mangle]
pub extern "C" fn mainPageScraperGetMatchesBrief(
    ptr: *const mainPageScraperWrapper,
    _type: matchBriefTypeWrapper,
    inout_ptr: *mut *mut matchBriefInfoWrapper,
    inout_len: *mut c_int
) -> i32 {
    let scraper = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };

    let matches = match scraper.matches_brief(_type.into())
        .into_iter()
        .map(|m| matchBriefInfoWrapper::try_from(m))
        .collect::<Result<Vec<_>>>() {
        Ok(v) => v,
        Err(_) => return -1
    };

    let (ptr, len) = vec_to_ffi(matches);

    unsafe {
        *inout_ptr = ptr;
        *inout_len = len;
    }

    0
}

#[no_mangle]
pub extern "C" fn matchesBriefInfoVecFree(ptr: *mut matchBriefInfoWrapper, len: c_int) {
    let len = len as usize;

    unsafe {
        let _ = Vec::from_raw_parts(ptr, len, len);
    }
}

#[no_mangle]
pub extern "C" fn mainPageScraperFree(ptr: *mut mainPageScraperWrapper) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}
