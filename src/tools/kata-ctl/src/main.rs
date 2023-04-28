// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0
//

mod arch;
mod args;
mod check;
mod ops;
mod types;
mod utils;

use anyhow::Result;
use clap::{Parser, crate_name};
use std::process::exit;
use std::io;

use args::{Commands, KataCtlCli};

use ops::check_ops::{
    handle_check, handle_env, handle_factory, handle_iptables, handle_metrics, handle_version,
};
use ops::exec_ops::handle_exec;
use ops::volume_ops::handle_direct_volume;
use slog::{error, o};

macro_rules! sl {
    () => {
        slog_scope::logger().new(o!("subsystem" => "main"))
    };
}

fn real_main() -> Result<()> {
    let args = KataCtlCli::parse();

    let log_level = match args.log_level {
        None => slog::Level::Info,
        Some(level_arg) => level_arg.get_slog_level(),
    };

    let (logger, _guard);
    if args.json_logging {
        let writer = io::stdout();
        (logger, _guard) = logging::create_logger(crate_name!(), crate_name!(), log_level, writer);
    } else {
        (logger, _guard) = logging::create_term_logger(log_level);
    }
    let _guard = slog_scope::set_global_logger(logger);

    let res = match args.command {
       Commands::Check(args) => handle_check(args),
       Commands::DirectVolume(args) => handle_direct_volume(args),
       Commands::Exec(args) => handle_exec(args),
       Commands::Env => handle_env(),
       Commands::Factory => handle_factory(),
       Commands::Iptables(args) => handle_iptables(args),
       Commands::Metrics(args) => handle_metrics(args),
       Commands::Version => handle_version(),
   };

   if let Err(ref e) = res {
      error!(sl!(), "{:#?}", e);
   }

   return res
}

fn main() {
   if let Err(_e) = real_main() {
       exit(1);
   }
}



