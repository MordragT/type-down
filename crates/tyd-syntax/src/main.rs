use std::fs;

use miette::Result;
use tyd_syntax::ast::Ast;

// use tyd_syntax::parse;

// fn main() -> Result<()> {
//     let cst = parse("../../examples/wip.tyd")?;

//     println!("{cst:?}");

//     Ok(())
// }

fn main() -> Result<()> {
    let src = fs::read_to_string("../../examples/wip.tyd").unwrap();

    let ast = Ast::parse(&src, "test")?;

    println!("{ast:?}");

    Ok(())
}
