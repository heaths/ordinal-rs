// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

// cspell:ignore xmldoc xsldoc xrust

use clap::Parser;
use std::path::Path;
use xrust::{
    item::{Item, Node as _, SequenceTrait},
    parser::xml::parse,
    transform::context::StaticContextBuilder,
    trees::smite::RNode,
    xslt::from_document,
    ErrorKind,
};

#[derive(Debug, Clone, Parser)]
#[command(about = "Generate documentation for the repository")]
pub struct Command {}

impl Command {
    pub fn run(&self, config: &super::Config) -> Result<(), Box<dyn std::error::Error>> {
        let docs_dir = config.repo_dir.join("docs");
        let report_dir = config
            .repo_dir
            .join("target")
            .join("criterion")
            .join("fmt")
            .join("report");
        let xmldoc = Item::Node(load_xml(&report_dir, "violin.svg")?);
        let xsldoc = load_xml(&docs_dir, "style-svg.xslt")?;

        let mut static_content = StaticContextBuilder::new()
            .fetcher(|_| {
                Err(xrust::Error::new(
                    ErrorKind::NotImplemented,
                    "not implemented",
                ))
            })
            .message(|_| Ok(()))
            .parser(|_| {
                Err(xrust::Error::new(
                    ErrorKind::NotImplemented,
                    "not implemented",
                ))
            })
            .build();

        let mut ctx = from_document(xsldoc, None, make_from_str, |_| Ok(String::new()))?;
        ctx.context(vec![xmldoc], 0);
        ctx.result_document(RNode::new_document());

        let xml = ctx.evaluate(&mut static_content)?;
        println!("{}", xml.to_xml());

        Ok(())
    }
}

fn load_xml(dir: &Path, name: &'static str) -> Result<RNode, Box<dyn std::error::Error>> {
    let path = dir.join(name);
    let content = std::fs::read_to_string(path)?;
    Ok(make_from_str(&content)?)
}

fn make_from_str(s: &str) -> Result<RNode, xrust::Error> {
    let doc = RNode::new_document();
    let _ = parse(doc.clone(), s, None)?;
    Ok(doc)
}
