use std::collections::BTreeMap;
use clap::{command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};

pub fn cli() -> Command {
    command!()
        .group(ArgGroup::new("directory").multiple(false))
        .next_help_heading("DIRECTORY")
        .args([
            position_sensitive_flag(Arg::new("dir"))
                .long("dir")
                .action(ArgAction::Append)
                .help("File directory you would like to search")
                .group("directory"),
        ])
        .group(ArgGroup::new("operators").multiple(true))
        .next_help_heading("OPERATORS")
        .args([
            position_sensitive_flag(Arg::new("size"))
                .short('s')
                .long("size")
                .action(ArgAction::Append)
                .help("Human readable size")
                .group("operators"),
				position_sensitive_flag(Arg::new("all"))
                .short('a')
                .long("all")
                .action(ArgAction::Append)
                .help("Show all files")
                .group("operators"),
        ])
}

fn position_sensitive_flag(arg: Arg) -> Arg {
    // Flags don't track the position of each occurrence, so we need to emulate flags with
    // value-less options to get the same result
    arg.num_args(0)
        .value_parser(value_parser!(bool))
        .default_missing_value("true")
        .default_value("false")
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Value {
    Bool(bool),
    String(String),
}

impl Value {
    pub fn from_matches(matches: &ArgMatches) -> Vec<(clap::Id, Self)> {
        let mut values = BTreeMap::new();
        for id in matches.ids() {
            if matches.try_get_many::<clap::Id>(id.as_str()).is_ok() {
                // ignore groups
                continue;
            }
            let value_source = matches
                .value_source(id.as_str())
                .expect("id came from matches");
            if value_source != clap::parser::ValueSource::CommandLine {
                // Any other source just gets tacked on at the end (like default values)
                continue;
            }
            if Self::extract::<String>(matches, id, &mut values) {
                continue;
            }
            if Self::extract::<bool>(matches, id, &mut values) {
                continue;
            }
            unimplemented!("unknown type for {id}: {matches:?}");
        }
        values.into_values().collect::<Vec<_>>()
    }

    fn extract<T: Clone + Into<Value> + Send + Sync + 'static>(
        matches: &ArgMatches,
        id: &clap::Id,
        output: &mut BTreeMap<usize, (clap::Id, Self)>,
    ) -> bool {
        match matches.try_get_many::<T>(id.as_str()) {
            Ok(Some(values)) => {
                for (value, index) in values.zip(
                    matches
                        .indices_of(id.as_str())
                        .expect("id came from matches"),
                ) {
                    output.insert(index, (id.clone(), value.clone().into()));
                }
                true
            }
            Ok(None) => {
                unreachable!("`ids` only reports what is present")
            }
            Err(clap::parser::MatchesError::UnknownArgument { .. }) => {
                unreachable!("id came from matches")
            }
            Err(clap::parser::MatchesError::Downcast { .. }) => false,
            Err(_) => {
                unreachable!("id came from matches")
            }
        }
    }
}

impl From<String> for Value {
    fn from(other: String) -> Self {
        Self::String(other)
    }
}

impl From<bool> for Value {
    fn from(other: bool) -> Self {
        Self::Bool(other)
    }
}