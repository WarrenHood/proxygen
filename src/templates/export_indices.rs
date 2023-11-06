// After making changes in this file, you should run `proxify update .`` in the root of this project to update exports

#![allow(non_upper_case_globals)]

pub const TOTAL_EXPORTS: usize = {{ total_exports }};
{{ export_indices }}
