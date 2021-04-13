use structopt::StructOpt;

#[wasm_run::main("yewprint-doc")]
#[derive(StructOpt, Debug)]
enum Cli {}
