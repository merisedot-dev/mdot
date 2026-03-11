use crate::entity::Cardinality;

// cardinality constants
pub const DEFAULT_CARDINALITY: (String, Cardinality, Cardinality) =
    (String::new(), Cardinality::ZERO, Cardinality::MANY);

// script snippets
pub const MK_DB: &'static str = "create database";
pub const MK_ENTITY: &'static str = "create table if not exists";
pub const EDT_ENTITY: &'static str = "alter table";
pub const MK_CSTR: &'static str = "add constraint";
