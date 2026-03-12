use crate::entity::Cardi;

// cardinality constants
pub const DEFAULT_CARDINALITY: (String, Cardi, Cardi) =
    (String::new(), Cardi::ZERO, Cardi::MANY);

// script snippets
pub const MK_DB: &'static str = "create database";
pub const MK_ENTITY: &'static str = "create table if not exists";
pub const EDT_ENTITY: &'static str = "alter table";
pub const MK_CSTR: &'static str = "add constraint";
