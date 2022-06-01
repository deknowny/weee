use crate::context::RTContext;
use crate::error::CLIError;

pub type CmdResult<S = ()> = Result<S, CLIError>;

pub trait Handleable {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult;
}
