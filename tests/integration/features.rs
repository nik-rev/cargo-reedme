use std::{collections::HashMap, iter};

use itertools::Itertools;

use super::*;

/// Note: This is only temporary because the `auto_default`
/// crate is unable to parse types with a comma in them
type HashMapStrBool<'a> = HashMap<&'a str, bool>;

#[auto_default]
struct Case<'a> {
    #[auto_default(skip)]
    expected_features: HashMapStrBool<'a>,
    cargo_default_features: Vec<&'a str>,

    cli_features: Vec<&'a str>,
    cli_no_default_features: bool,
    cli_all_features: bool,

    config_features: Option<Vec<&'a str>>,
    config_no_default_features: Option<bool>,
    config_all_features: Option<bool>,
}

fn value_to_string<T: serde::Serialize>(value: T) -> String {
    let mut out = String::new();
    serde::Serialize::serialize(&value, toml::ser::ValueSerializer::new(&mut out)).unwrap();
    out
}

#[track_caller]
fn test(
    Case {
        expected_features,
        cargo_default_features,

        cli_features,
        cli_no_default_features,
        cli_all_features,

        config_features,
        config_no_default_features,
        config_all_features,
    }: Case,
) -> Result<()> {
    let lib_rs = expected_features
        .keys()
        .map(|feature| {
            docstr!(format!
                /// #![cfg_attr(feature = "{feature}", doc = "{feature}")]
                /// #![cfg_attr(not(feature = "{feature}"), doc = "!{feature}")]
            )
        })
        .join("\n");

    let cargo_features_definition = expected_features
        .keys()
        .map(|feature| format!("{feature} = []"))
        .join("\n");
    let cargo_features_definition = docstr!(format!
        /// default = {}
        /// {cargo_features_definition}
        value_to_string(&cargo_default_features)
    );

    let config = {
        let config_features =
            config_features.map(|value| format!("features = {}", value_to_string(value)));
        let config_no_default_features = config_no_default_features
            .map(|value| format!("no-default-features = {}", value_to_string(value)));
        let config_all_features =
            config_all_features.map(|value| format!("all-features = {}", value_to_string(value)));

        [
            config_features,
            config_no_default_features,
            config_all_features,
        ]
        .iter()
        .flatten()
        .join("\n")
    };

    let args = {
        let cli_features = if cli_features.is_empty() {
            Vec::new()
        } else {
            iter::once("--features").chain(cli_features).collect()
        };

        let cli_all_features = if cli_all_features {
            Vec::from(["--all-features"])
        } else {
            Vec::new()
        };

        let cli_no_default_features = if cli_no_default_features {
            Vec::from(["--no-default-features"])
        } else {
            Vec::new()
        };

        [cli_features, cli_all_features, cli_no_default_features]
            .into_iter()
            .flatten()
            .collect()
    };

    let generated_readme = expected_features
        .iter()
        .map(|(feature, is_enabled)| {
            if *is_enabled {
                feature.to_string()
            } else {
                format!("!{feature}")
            }
        })
        .join("\n");

    super::test(super::Case {
        lib_rs: Some(&lib_rs),
        features: Some(&cargo_features_definition),
        config: Some(&config),
        args,
        generated: &generated_readme,
        ..
    })
}

#[test]
fn disabled_feature() -> Result<()> {
    test(Case {
        expected_features: HashMap::from([("a", false)]),
        ..
    })?;
    Ok(())
}

#[test]
fn enabled_feature_via_cli() -> Result<()> {
    test(Case {
        expected_features: HashMap::from([("a", true)]),
        cli_features: vec!["a"],
        ..
    })?;
    Ok(())
}

#[test]
fn enabled_feature_via_config() -> Result<()> {
    test(Case {
        expected_features: HashMap::from([("a", true)]),
        config_features: Some(vec!["a"]),
        ..
    })?;
    Ok(())
}

/// CLI takes precedence over settings defined in Config
#[test]
fn enabled_feature_via_config_and_cli() -> Result<()> {
    test(Case {
        // Now 'b' wins because CLI overrides Config's 'a'
        expected_features: HashMap::from([("a", false), ("b", true)]),
        config_features: Some(vec!["a"]),
        cli_features: vec!["b"],
        ..
    })?;
    Ok(())
}

#[test]
fn default_feature_enabled_by_default() -> Result<()> {
    test(Case {
        expected_features: HashMap::from([("a", true)]),
        cargo_default_features: vec!["a"],
        ..
    })?;
    Ok(())
}

#[test]
fn disable_default_features_via_cli() -> Result<()> {
    test(Case {
        expected_features: HashMap::from([("a", false)]),
        cargo_default_features: vec!["a"],
        cli_no_default_features: true,
        ..
    })?;
    Ok(())
}

// Config says "use defaults", but CLI says "no defaults", which takes priority
#[test]
fn cli_overrides_config_no_default_features() -> Result<()> {
    test(Case {
        expected_features: HashMap::from([("a", false)]),
        cargo_default_features: vec!["a"],
        cli_no_default_features: true,
        config_no_default_features: Some(false),
        ..
    })?;
    Ok(())
}

#[test]
fn all_features_via_cli() -> Result<()> {
    test(Case {
        expected_features: HashMap::from([("a", true), ("b", true)]),
        cli_all_features: true,
        ..
    })?;
    Ok(())
}

#[test]
fn all_features_is_separate() -> Result<()> {
    test(Case {
        expected_features: HashMap::from([("a", true), ("b", true)]),
        cli_features: vec!["a"],
        config_all_features: Some(true),
        ..
    })?;
    Ok(())
}

/// Config says no to all-features, but CLI wants everything.
#[test]
fn cli_all_features_overrides_config_disabling() -> Result<()> {
    test(Case {
        expected_features: HashMap::from([("a", true), ("b", true)]),
        cli_all_features: true,
        config_all_features: Some(false),
        ..
    })?;
    Ok(())
}
