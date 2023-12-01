/*
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Apikey(HeaderValueString);

derive_header! {
    Apikey(_),
    name: USER_AGENT
}

impl ApiKey {
    pub fn from_static(src: &'static str) -> ApiKey {
        ApiKey(HeaderValueString::from_static(src))
    }

    /// View this `Apikey` as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

error_type!(InvalidUserAgent);

impl FromStr for Apikey {
    type Err = InvalidUserAgent;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        HeaderValueString::from_str(src)
            .map(Apikey)
            .map_err(|_| InvalidUserAgent { _inner: () })
    }
}

impl fmt::Display for Apikey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
*/
