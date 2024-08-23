use crossterm::{
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
    cursor::MoveTo,
    ExecutableCommnd,
};
use std::io::{stdout, Write};
use std::{thread, time};
use rand::Rng;
use std::env;
