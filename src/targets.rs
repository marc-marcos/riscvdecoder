use crate::Extension;
use std::collections::HashSet;

pub struct Target(
    pub &'static str,         // Name
    pub &'static [Extension], // Supported extensions
);

pub const ALL_TARGETS: &[Target] = &[WITH_F, WITHOUT_F];

pub const WITH_F: Target = Target(
    "Core with F extension",
    &[Extension::I, Extension::M, Extension::A, Extension::F],
);

pub const WITHOUT_F: Target = Target(
    "Core without F extension",
    &[Extension::I, Extension::M, Extension::A],
);

/// Checks if a Target contains **at least** all the extensions from the specified set of [`Extension`].
pub fn is_able_to_run(extensions: &HashSet<Extension>, core: &Target) -> bool {
    extensions.iter().all(|req| core.1.contains(req))
}
