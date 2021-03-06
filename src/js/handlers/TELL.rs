use std::fs::File;

use crate::zil::contracts::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct TELL {}

impl HandleJS for TELL {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_routine() ||
           root.children.len() < 2 ||
           !root.children[0].is_word() ||
           root.children[0].tokens[0].value != "TELL" {
            return Err(HandlerErr::origin(format!("Invalid TELL: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}print(", spacer)));
      
        for i in 1..root.children.len() {
            match root.children[i].kind() {
                ZilNodeType::Routine => wrap!(R::print(&root.children[i], 0, &mut writer)),
                ZilNodeType::Text => wrap!(T::print(&root.children[i], 0, &mut writer)),
                ZilNodeType::Word => wrap!(W::print(&root.children[i], 0, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot print unknown ZilNodeType in TELL"))),
            };
            if i+1 < root.children.len() {
                wrap!(writer.w(" + "));
            }
        }
      
        wrap!(writer.w(")"));
      
        Ok(())
    }
}