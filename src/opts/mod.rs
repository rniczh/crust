use std::path;

mod arg_groups;

#[derive(structopt::StructOpt, Clone, Debug)]
pub struct Opts {
    #[structopt(flatten)]
    stop_stage: StopStage,
    /// The output file.
    #[structopt(short = "-o", parse(from_os_str))]
    output: path::PathBuf,
    /// The input file(s).
    #[structopt(parse(from_os_str))]
    input: Vec<path::PathBuf>,
}

#[derive(structopt::StructOpt, Clone, Debug)]
#[structopt(raw(group = "self::arg_groups::stop_stage_conflict_resolver_arg_group()"))]
pub struct StopStage {
    /// Stop after the assembly stage.
    #[structopt(group = "stop_stage_conflict_resolver", short = "-c")]
    assemble_only: bool,
    /// Stop after the compilation stage.
    #[structopt(group = "stop_stage_conflict_resolver", short = "-S")]
    compile_only: bool,
    /// Stop after the preprocessing stage.
    #[structopt(group = "stop_stage_conflict_resolver", short = "-E")]
    preprocess_only: bool,
}
