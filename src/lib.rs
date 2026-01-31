//! # Qrate
//!
//! `Qrate` is a library for managing question banks and student data.
//! It provides functionalities to load, manipulate, and store questions and student information,
//! primarily from and to SQLite database files and Excel files for storage.

/// The `database` module provides the `SQLiteDB` for database interactions.
pub mod database;
/// The `excel` module provides functionalities for Excel file operations.
pub mod excel;
/// The `header` module contains the `Header` structure for parsing file headers.
pub mod header;
// /// The `loader` module provides functions to load data from files.
// pub mod loader;
/// The `qbank` module contains the `QBank` structure and its associated methods.
pub mod qbank;
/// The `qbdb` module handles the question bank database operations.
pub mod qbdb;
/// The `question` module defines the `Question` structure.
pub mod question;
/// The `sbank` module contains the `SBank` structure and related functionalities.
pub mod sbank;
/// The `sbdb` module handles the student bank database operations.
pub mod sbdb;
/// The `student` module defines the `Student` structure.
pub mod student;

pub use database::SQLiteDB;
pub use excel::Excel;
pub use header::Header;
// pub use loader::load_question_bank_from_csv;
pub use qbank::QBank;
pub use qbdb::QBDB;
pub use question::{ChoiceAnswer, Choices, Question};
pub use sbank::{SBank, get_bank};
pub use sbdb::SBDB;
pub use student::Student;
