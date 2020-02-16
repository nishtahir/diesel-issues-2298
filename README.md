# Sample project for diesel/issues/2298

Attempting to `grouped_by` panics if the parent array contains keys
not present in the child array.

The issue seems to be as a result of a `[]` map access here
https://docs.diesel.rs/src/diesel/associations/belongs_to.rs.html#128

