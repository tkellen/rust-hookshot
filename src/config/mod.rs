use toml;
use std::collections::BTreeMap;

mod group;
mod error;
use self::group::Group;
use self::error::Error;

pub type Config = BTreeMap<String, Group>;

pub fn parse(input: &str) -> Result<Config, Error> {
    let mut config = Config::new();
    let mut parser = toml::Parser::new(input);
    match parser.parse() {
        Some(groups) => {
            // todo: find a way to use iter/fold here
            for (key, value) in groups.into_iter() {
                match Group::new(&value) {
                    Ok(group) => { config.insert(key, group); },
                    Err(err) => {
                        // todo: figure out how to pass in err
                        println!("{:?}", err);
                        return Err(Error::InvalidGroup)
                    }
                };
            }
        },
        // todo: use parser.errors
        None => return Err(Error::Parse)
    }
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn parse_test() {
        parse("\
          [group]\n\
            [[group.notifiers]]\n\
              url = \"http://second.com\"\n\
            [[group.notifiers]]\n\
              url = \"http://first.com\"\n\
        ");
    }
}
