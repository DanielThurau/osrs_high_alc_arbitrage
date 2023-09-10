pub const WIKI_ENDPOINT_URL: &str = "https://prices.runescape.wiki/api/v1/osrs/";

#[allow(dead_code)]
pub enum Route {
    Latest,
    Mapping,
    FiveMin,
    OneHour,
    TimeSeries,
}

impl Route {
    pub fn endpoint(&self) -> String {
        match self {
            Route::Latest => "latest".to_string(),
            Route::Mapping => "mapping".to_string(),
            Route::FiveMin => "5m".to_string(),
            Route::OneHour => "1h".to_string(),
            Route::TimeSeries => "timeseries".to_string(),
        }
    }
}
