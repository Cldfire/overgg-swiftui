use crate::error::*;
use crate::utils::vec_to_ffi;
use overgg_scraper::http_client::Downloader;
use overgg_scraper::scrapers::main_page::MainPageScraper;
use overgg_scraper::data_structs::{
    MatchBriefType,
    MatchBriefInfo,
    TeamCompletedMatchBriefInfo,
    EventInfo
};
use std::ops::{Deref, DerefMut};
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

#[derive(Debug)]
#[repr(C)]
pub struct LameOption<T> {
    pub val: T,
    pub is_present: bool
}

pub struct downloaderWrapper(pub Downloader);

impl Deref for downloaderWrapper {
    type Target = Downloader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for downloaderWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct mainPageScraperWrapper(pub MainPageScraper);

impl Deref for mainPageScraperWrapper {
    type Target = MainPageScraper;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for mainPageScraperWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(u8)]
pub enum matchBriefTypeWrapper {
    IN_FUTURE,
    LIVE,
    COMPLETED
}

impl From<matchBriefTypeWrapper> for MatchBriefType {
    fn from(_enum: matchBriefTypeWrapper) -> Self {
        match _enum {
            matchBriefTypeWrapper::IN_FUTURE => MatchBriefType::InFuture,
            matchBriefTypeWrapper::LIVE => MatchBriefType::Live,
            matchBriefTypeWrapper::COMPLETED => MatchBriefType::Completed
        }
    }
}

impl From<MatchBriefType> for matchBriefTypeWrapper {
    fn from(_enum: MatchBriefType) -> Self {
        match _enum {
            MatchBriefType::InFuture => matchBriefTypeWrapper::IN_FUTURE,
            MatchBriefType::Live => matchBriefTypeWrapper::LIVE,
            MatchBriefType::Completed => matchBriefTypeWrapper::COMPLETED
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct matchBriefInfoWrapper {
    pub event: eventInfoWrapper,
    pub teams_ptr: *mut teamCompletedMatchBriefInfoWrapper,
    pub teams_len: c_int,
    pub scheduled_timestamp: LameOption<i64>
}

impl matchBriefInfoWrapper {
    pub fn try_from(_struct: MatchBriefInfo) -> Result<Self> {
        let teams = _struct.teams
            .iter()
            .map(|t| teamCompletedMatchBriefInfoWrapper::try_from(t.clone()))
            .collect::<Result<Vec<_>>>()?;

        let (teams_ptr, teams_len) = vec_to_ffi(teams);

        Ok(Self {
            event: eventInfoWrapper::try_from(_struct.event)?,
            teams_ptr,
            teams_len,
            scheduled_timestamp: match _struct.scheduled_time {
                Some(t) => LameOption {
                    val: t.timestamp(),
                    is_present: true
                },
                None => LameOption {
                    val: 0,
                    is_present: false
                }
            }
        })
    }
}

impl Drop for matchBriefInfoWrapper {
    fn drop(&mut self) {
        if self.teams_ptr.is_null() { return }

        let len = self.teams_len as usize;
        unsafe {
            let _ = Vec::from_raw_parts(self.teams_ptr, len, len);
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct eventInfoWrapper {
    pub name: *mut c_char,
    pub series: *mut c_char
}

impl eventInfoWrapper {
    pub fn try_from(_struct: EventInfo) -> Result<Self> {
        let name_c_str = CString::new(_struct.name)?;
        let series_c_str = CString::new(_struct.series)?;

        Ok(Self {
            name: name_c_str.into_raw(),
            series: series_c_str.into_raw()
        })
    }
}

impl Drop for eventInfoWrapper {
    fn drop(&mut self) {
        if self.name.is_null() || self.series.is_null() { return }

        unsafe {
            let _ = CString::from_raw(self.name);
            let _ = CString::from_raw(self.series);
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct teamCompletedMatchBriefInfoWrapper {
    pub name: *mut c_char,
    pub maps_won: LameOption<u8>
}

impl teamCompletedMatchBriefInfoWrapper {
    pub fn try_from(_struct: TeamCompletedMatchBriefInfo) -> Result<Self> {
        let c_str = CString::new(_struct.name)?;

        Ok(Self {
            name: c_str.into_raw(),
            maps_won: match _struct.maps_won {
                Some(w) => LameOption {
                    val: w,
                    is_present: true
                },
                None => LameOption {
                    val: 0,
                    is_present: false
                }
            }
        })
    }
}

impl Drop for teamCompletedMatchBriefInfoWrapper {
    fn drop(&mut self) {
        if self.name.is_null() { return }

        unsafe {
            let _ = CString::from_raw(self.name);
        }
    }
}
