// cardinality constants
pub const DEFAULT_CARDINALITY: (String, u8, u8) = (String::new(), 0, 0);

// script snippets
pub const MK_DB: &'static str = "create database";
pub const MK_ENTITY: &'static str = "create table if not exists";
pub const EDT_ENTITY: &'static str = "alter table";
pub const MK_CSTR: &'static str = "add constraint";
