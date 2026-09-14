use super::*;

#[derive(Clone, Debug)]
pub struct ReferenceStage {
    rust_path: String,
    filter: String,
    force_external: bool,
}

impl ReferenceStage {
    pub fn implicit(rust_path: &str, filter: &str) -> Self {
        Self {
            rust_path: rust_path.to_string(),
            filter: filter.to_string(),
            force_external: false,
        }
    }

    pub fn external(rust_path: &str, filter: &str) -> Self {
        Self {
            rust_path: rust_path.to_string(),
            filter: filter.to_string(),
            force_external: true,
        }
    }
}

#[derive(Debug)]
pub struct Reference {
    pub rust_path: String,
    pub filter: Filter,
    pub source_filter: String,
    pub force_external: bool,
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
                    }
                })
                .collect(),
        );

        for namespace in reader.keys() {
            for name in reader[namespace].keys() {
                let type_name = TypeName(namespace, name);
                let matching = references
                    .0
                    .iter()
                    .filter(|reference| reference.filter.includes_type_name(type_name).is_some())
                    .collect::<Vec<_>>();

                assert!(
                    matching.len() <= 1,
                    "ambiguous external reference routes for `{namespace}.{name}`: {}",
                    matching
                        .iter()
                        .map(|reference| format!(
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
        self.0
            .iter()
            .find(|reference| reference.filter.includes_type_name(name).is_some())
    }

    pub fn matching_rule(&self, name: TypeName) -> Option<(&str, bool)> {
        self.0.iter().find_map(|reference| {
            reference
                .filter
                .includes_type_name(name)
                .map(|rule| (rule, reference.force_external))
        })
    }
}
