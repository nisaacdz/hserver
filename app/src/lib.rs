pub mod settings;
pub use settings::*;

pub mod api;
pub mod auth;
pub mod bookings;
pub mod interval;
pub mod rooms;
pub mod users;

pub use actix_web;
