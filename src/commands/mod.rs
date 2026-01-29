// Declare the `add` module. This corresponds to a file named `add.rs`
// or a folder `add/mod.rs` that implements the logic for adding tasks.
pub mod add;

// Declare the `update` module, which handles updating task descriptions.
pub mod update;

// Declare the `delete` module, which handles removing tasks by ID.
pub mod delete;

// Declare the `mark` module, which handles updating task status
// (e.g., in-progress or done).
pub mod mark;

// Declare the `list` module, which handles listing tasks, optionally filtered by status.
pub mod list;
