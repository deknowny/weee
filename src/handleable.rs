use crate::error::CLIError;
use crate::context::RTContext;

pub type CmdResult<S = ()> = Result<S, CLIError>;

pub trait Handleable {
    fn handle(&self, ctx: &mut RTContext) -> CmdResult;
}
