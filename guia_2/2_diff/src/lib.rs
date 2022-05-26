///App module that consumes all the other modules in order to run the diff.
pub mod app;
///Actual diff module.
pub mod diff;
///Module that contains the core function lcs for the proper generation of the diff.
pub mod lcs;
///General utilities module that contains helper functions that are useful to the diff.
pub mod utils;
