use toml;
use url::Url;

use config::error::{Error};

pub type Notifiers = Vec<Notifier>;
pub type NotifierUrl = Url;

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct Notifier {
    url: NotifierUrl
}

#[derive(Debug)]
pub struct Group {
    pub notifiers: Notifiers
}

impl Group {
    pub fn new(input: &toml::Value) -> Result<Group, Error> {
        input.lookup("notifiers").ok_or(Error::NotifiersMissing).and_then(|notifiers| {
            Self::read_notifiers(notifiers)
                .map(|notifiers| Group{notifiers:notifiers})
                .map_err(|err| {
                    // todo: figure out how to pass in err
                    println!("{:?}", err);
                    Error::NotifiersInvalid
                })
        })
    }

    fn read_notifiers(notifiers: &toml::Value) -> Result<Notifiers, Error> {
        notifiers.as_slice().ok_or(Error::NotifiersInvalid).and_then(|notifiers| {
            // todo: make this use .iter().map instead of a loop withs side effects
            let mut all = Notifiers::new();
            for notifier in notifiers {
                match Self::read_notifier(notifier) {
                    Ok(notifier) => all.push(notifier),
                    Err(msg) => return Err(msg)
                }
            }
            Ok(all)
        })
    }

    fn read_notifier(notifier: &toml::Value) -> Result<Notifier, Error> {
        notifier.as_table().ok_or(Error::NotifierInvalid).and_then(|table| {
            table.get("url").ok_or(Error::NotifierUrlMissing).and_then(|url| {
                Self::read_url(url).map(|url| Notifier { url: url })
            })
        })
    }

    fn read_url(url: &toml::Value) -> Result<NotifierUrl, Error> {
        url.as_str().ok_or(Error::NotifierUrlMissing).and_then(|url| {
            Url::parse(url).map_err(|err| {
                // todo figure out how to pass in err
                println!("{:?}", err);
                Error::NotifierUrlInvalid
            })
        })
    }

}
