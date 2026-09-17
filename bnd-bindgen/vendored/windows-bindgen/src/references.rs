use super::*;

#[derive(Clone, Debug)]
pub struct ReferenceStage {
    rust_path: String,
    filter: String,
    force_external: bool,
    style: ReferenceStyle,
}

impl ReferenceStage {
    pub fn implicit(rust_path: &str, filter: &str, style: ReferenceStyle) -> Self {
        Self {
            rust_path: rust_path.to_string(),
            filter: filter.to_string(),
            force_external: false,
            style,
        }
    }

    pub fn external(rust_path: &str, filter: &str, style: ReferenceStyle) -> Self {
        Self {
            rust_path: rust_path.to_string(),
            filter: filter.to_string(),
            force_external: true,
            style,
        }
    }
}

#[derive(Debug)]
pub struct Reference {
    pub rust_path: String,
    pub filter: Filter,
    pub source_filter: String,
    pub force_external: bool,
    pub style: ReferenceStyle,
}

#[derive(Debug, Default)]
pub struct References(Vec<Reference>);

impl References {
    #[track_caller]
    pub fn new(reader: &Reader, stage: Vec<ReferenceStage>) -> Self {
        let references = Self(
            stage
                .into_iter()
                .map(|stage| {
                    let entries = filter_parser::parse_filter_entry(&stage.filter);
                    let resolved = filter_parser::resolve_entries(reader, &entries);
                    let filter = Filter::from_resolved(reader, &resolved);

                    Reference {
                        rust_path: stage.rust_path,
                        filter,
                        source_filter: stage.filter,
                        force_external: stage.force_external,
                        style: stage.style,
                    }
                })
                .collect(),
        );

        for namespace in reader.keys() {
            for name in reader[namespace].keys() {
                let type_name = TypeName(namespace, name);
                let matching = references.matches(type_name);
                let highest_precedence = matching
                    .iter()
                    .map(|(reference, rule)| (reference.force_external, rule.len()))
                    .max();
                let forced = matching
                    .iter()
                    .filter(|(reference, _)| reference.force_external)
                    .count();
                let ambiguous = forced > 1
                    || (forced == 0
                        && highest_precedence.is_some_and(|precedence| {
                            matching
                                .iter()
                                .filter(|(reference, rule)| {
                                    (reference.force_external, rule.len()) == precedence
                                })
                                .count()
                                > 1
                        }));

                assert!(
                    !ambiguous,
                    "ambiguous external reference routes for `{namespace}.{name}`: {}",
                    matching
                        .iter()
                        .map(|(reference, _)| format!(
                            "`{}` -> `{}`",
                            reference.source_filter, reference.rust_path
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }

        references
    }

    pub fn contains(&self, name: TypeName) -> Option<&Reference> {
        self.best_match(name).map(|(reference, _)| reference)
    }

    pub fn matching_rule(&self, name: TypeName) -> Option<(&str, bool)> {
        self.best_match(name)
            .map(|(reference, rule)| (rule, reference.force_external))
    }

    fn matches(&self, name: TypeName) -> Vec<(&Reference, &str)> {
        self.0
            .iter()
            .filter_map(|reference| {
                reference
                    .filter
                    .includes_type_name(name)
                    .map(|rule| (reference, rule))
            })
            .collect()
    }

    fn best_match(&self, name: TypeName) -> Option<(&Reference, &str)> {
        self.matches(name)
            .into_iter()
            .max_by_key(|(reference, rule)| (reference.force_external, rule.len()))
    }
}
