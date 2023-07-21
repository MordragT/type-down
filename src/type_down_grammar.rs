use crate::type_down_grammar_trait::{TypeDown, TypeDownGrammarTrait};
#[allow(unused_imports)]
use parol_runtime::{Result, Token};
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our TypeDown grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct TypeDownGrammar<'t> {
    pub type_down: Option<TypeDown<'t>>,
}

impl TypeDownGrammar<'_> {
    pub fn new() -> Self {
        TypeDownGrammar::default()
    }
}

impl Display for TypeDown<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for TypeDownGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.type_down {
            Some(type_down) => writeln!(f, "{}", type_down),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> TypeDownGrammarTrait<'t> for TypeDownGrammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal 'TypeDown'
    fn type_down(&mut self, arg: &TypeDown<'t>) -> Result<()> {
        self.type_down = Some(arg.clone());
        Ok(())
    }
}
