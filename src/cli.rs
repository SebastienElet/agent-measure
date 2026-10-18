use agent_measure::measure::{
    FeedbackArgs, FinishArgs, HookAgent, ListArgs, OutcomeArgs, PrVerdictArgs, ReportArgs,
};
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(version, about = "Local agent measurement")]
pub struct Cli {
    #[command(subcommand)]
    pub command: MeasureCommand,
}
#[derive(Subcommand)]
pub enum MeasureCommand {
    Hook {
        #[arg(long, value_enum)]
        agent: HookAgent,
    },
    List(ListArgs),
    Finish(FinishArgs),
    Feedback(FeedbackArgs),
    Outcome(OutcomeArgs),
    PrVerdict(PrVerdictArgs),
    Report(ReportArgs),
}
