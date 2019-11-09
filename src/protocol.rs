use serde::{Serialize, Deserialize};
// use serde::Serialize;

// use crate::i2c;
// use std::sync::mpsc::Sender;

// gui stuff
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewWindow {
    pub id: String,
    pub content: String,
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: i32,
    pub height: i32,
}

// i2c stuff
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Button {
    B1,
    B2,
    B3,
    B4,
}

// Serial stuff... well, it's all serial stuff.
#[derive(Serialize, Deserialize, Debug)]
enum IncomingMsg {
    CreateWindow(NewWindow),
    DestroyWindow(u32),
    On(Button),
    Off(Button),
}

#[derive(Serialize, Debug)]
enum OutgoingMsg {
    Pressed(Button),
    Released(Button),
}