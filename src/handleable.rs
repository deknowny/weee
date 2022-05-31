use core;
use std;

use crate::error::CLIError;
use crate::rtcontext::RTContext;
use crate::show_err;

pub type CmdResult = Result<(), CLIError>;

pub trait Handleable {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult;
}
