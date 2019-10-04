error_chain! {
    links {
        Scraper(::overgg_scraper::error::Error, ::overgg_scraper::error::ErrorKind);
    }

    foreign_links {
        Nul(::std::ffi::NulError);
    }
}
