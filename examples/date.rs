// Copyright 2023 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

// cspell:ignore datelike
use chrono::{DateTime, Datelike as _, Local};
use ordinal::ToOrdinal as _;

fn main() {
    let now: DateTime<Local> = Local::now();
    let format = format!("%A, %B %-d{}, %Y %-I:%M:%S %p", now.day().suffix());
    println!("{}", now.format(format.as_ref()));
}
