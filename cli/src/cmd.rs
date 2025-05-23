// SONIC: Standard library for formally-verifiable distributed contracts
//
// SPDX-License-Identifier: Apache-2.0
//
// Designed in 2019-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
// Written in 2024-2025 by Dr Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright (C) 2019-2024 LNP/BP Standards Association, Switzerland.
// Copyright (C) 2024-2025 Laboratories for Ubiquitous Deterministic Computing (UBIDECO),
//                         Institute for Distributed and Cognitive Systems (InDCS), Switzerland.
// Copyright (C) 2019-2025 Dr Maxim Orlovsky.
// All rights under the above copyrights are reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//        http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
// or implied. See the License for the specific language governing permissions and limitations under
// the License.

use std::fs::File;
use std::path::PathBuf;

use hypersonic::persistance::LedgerDir;
use hypersonic::{Articles, AuthToken, CallParams, IssueParams, Schema};

#[derive(Parser)]
pub enum Cmd {
    /// Issue a new HyperSONIC contract
    Issue {
        /// Schema used to issue the contract
        schema: PathBuf,

        /// Parameters and data for the contract
        params: PathBuf,

        /// Output file which will contain articles of the contract
        output: Option<PathBuf>,
    },

    /// Process contract articles into a contract directory
    Process {
        /// Contract articles to process
        articles: PathBuf,
        /// Directory to put the contract directory inside
        dir: Option<PathBuf>,
    },

    /// Print out a contract state
    State {
        /// Contract directory
        dir: PathBuf,
    },

    /// Make a contract call
    Call {
        /// Contract directory
        dir: PathBuf,
        /// Parameters and data for the call
        call: PathBuf,
    },

    /// Export contract deeds to a file
    Export {
        /// Contract directory
        dir: PathBuf,

        /// List of tokens of authority which should serve as a contract terminals.
        #[clap(short, long)]
        terminals: Vec<AuthToken>,

        /// Location to save the deeds file to
        output: PathBuf,
    },

    /// Accept deeds into a contract
    Accept {
        /// Contract directory
        dir: PathBuf,

        /// File with deeds to accept
        input: PathBuf,
    },
}

impl Cmd {
    pub fn exec(self) -> anyhow::Result<()> {
        match self {
            Cmd::Issue { schema, params, output } => issue(schema, params, output)?,
            Cmd::Process { articles, dir } => process(articles, dir)?,
            Cmd::State { dir } => state(dir)?,
            Cmd::Call { dir, call: path } => call(dir, path)?,
            Cmd::Export { dir, terminals, output } => export(dir, terminals, output)?,
            Cmd::Accept { dir, input } => accept(dir, input)?,
        }
        Ok(())
    }
}

fn issue(schema: PathBuf, form: PathBuf, output: Option<PathBuf>) -> anyhow::Result<()> {
    let schema = Schema::load(schema)?;
    let file = File::open(&form)?;
    let params = serde_yaml::from_reader::<_, IssueParams>(file)?;

    let path = output.unwrap_or(form);
    let output = path
        .with_file_name(params.name.as_str())
        .with_extension("articles");

    let articles = schema.issue(params);
    articles.save(output)?;

    Ok(())
}

fn process(articles_path: PathBuf, dir: Option<PathBuf>) -> anyhow::Result<()> {
    let articles = Articles::load(&articles_path)?;
    let path = dir
        .or_else(|| Some(articles_path.parent()?.to_path_buf()))
        .ok_or(anyhow::anyhow!("invalid path for creating the contract"))?;
    LedgerDir::new(articles, path)?;

    Ok(())
}

fn state(path: PathBuf) -> anyhow::Result<()> {
    let ledger = LedgerDir::load(path)?;
    let val = serde_yaml::to_string(&ledger.state().main)?;
    println!("{val}");
    Ok(())
}

fn call(dir: PathBuf, form: PathBuf) -> anyhow::Result<()> {
    let mut ledger = LedgerDir::load(dir)?;
    let file = File::open(form)?;
    let call = serde_yaml::from_reader::<_, CallParams>(file)?;
    let opid = ledger.call(call)?;
    println!("Operation ID: {opid}");
    Ok(())
}

fn export(dir: PathBuf, terminals: impl IntoIterator<Item = AuthToken>, output: PathBuf) -> anyhow::Result<()> {
    let mut ledger = LedgerDir::load(dir)?;
    ledger.export_to_file(terminals, output)?;
    Ok(())
}

fn accept(dir: PathBuf, input: PathBuf) -> anyhow::Result<()> {
    let mut ledger = LedgerDir::load(dir)?;
    ledger.accept_from_file(input)?;
    Ok(())
}
