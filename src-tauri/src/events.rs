//! Event name constants shared between the Tauri backend and the frontend
//! (mirrored in `src/lib/ipc.ts`, Task 4.3).

pub const REFRESH_STARTED: &str = "metagrid://refresh-started";
pub const REFRESH_DONE: &str = "metagrid://refresh-done";
pub const REFRESH_ERROR: &str = "metagrid://refresh-error";
pub const STATUS: &str = "metagrid://status";
