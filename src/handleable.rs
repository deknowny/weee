use crate::error::CLIError;
use crate::rtcontext::RTContext;

pub type CmdResult = Result<(), CLIError>;

pub trait Handleable {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult;
}
