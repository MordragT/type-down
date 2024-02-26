use miette::Result;
use tyd_syntax::{ast::Ast, cst::Cst};

// use tyd_syntax::parse;

// fn main() -> Result<()> {
//     let cst = parse("../../examples/wip.tyd")?;

//     println!("{cst:?}");

//     Ok(())
// }

fn main() -> Result<()> {
    let src = "

[ warning {label}
some text within div

and a link <example.com>[click here]
]
";

    let ast = Ast::parse(&src, "test")?;

    println!("{ast:?}");

    Ok(())
}
