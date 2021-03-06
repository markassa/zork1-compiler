use std::fs::File;

use crate::zil::contracts::*;
use crate::js::handlers::generic_tokens::*;
use crate::js::helpers::is_int;
use crate::js::contracts::*;
use crate::js::custom_buf_writer::*;

pub struct OBJECT {}

impl HandleJS for OBJECT {
    fn validate (root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_routine() ||
           root.children.len() < 2 ||
           !root.children[0].is_word() ||
           root.children[0].tokens[0].value != "OBJECT" {
            return Err(HandlerErr::origin(format!("Invalid OBJECT: {}", root)));
        }
        Ok(())
    }
  
    fn print(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        Self::validate(root)?;
      
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
        wrap!(writer.w(format!("{}let ", spacer)));
        wrap!(W::print(&root.children[1], 0, &mut writer));
        wrap!(writer.w(" = {\n"));

        for i in 2..root.children.len() {
            Self::validate_sub_grouping(&root.children[i])?;

            match &root.children[i].children[0].tokens[0].value[..] {
                "TEXT" | "DESC" | "LDESC" | "FDESC" | "DESCFCN" => wrap!(Self::return_string(&root.children[i], indent+1, &mut writer)),
                "ACTION" => wrap!(Self::return_fn(&root.children[i], indent+1, &mut writer)),
                "CAPACITY" | "SIZE" | "VALUE" | "TVALUE" => wrap!(Self::return_int(&root.children[i], indent+1, &mut writer)),
                "SYNONYM" | "ADJECTIVE" => wrap!(Self::return_string_array(&root.children[i], indent+1, &mut writer)),
                "FLAGS" | "VTYPE" => wrap!(Self::mut_bools(&root.children[i], indent+1, &mut writer)), // not sure if should be mutable
                "STRENGTH" => wrap!(Self::mut_int(&root.children[i], indent+1, &mut writer)), // not sure if should be mutable
                "IN" => wrap!(crate::js::handlers::subgrouping_IN::SubgroupingIN::print(&root.children[i], indent+1, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Unknown sub grouping in OBJECT"))),
            };
        }

        wrap!(writer.w(format!("{}}};\n\n", spacer)));

        Ok(())
    }
}

impl OBJECT {
    pub fn validate_sub_grouping(root: &ZilNode) -> Result<(), HandlerErr> {
        if !root.is_grouping() ||
           root.children.len() < 2 ||
           !root.children[0].is_word() {
            return Err(HandlerErr::origin(format!("Invalid OBJECT sub grouping: {}", root)));
        }
        Ok(())
    }

    pub fn return_string(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(": () => "));
    
        match root.children[1].kind() {
            ZilNodeType::Text => wrap!(T::print(&root.children[1], 0, &mut writer)),
            ZilNodeType::Word => wrap!(W::print_with_quotes(&root.children[1], 0, &mut writer)),
            _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown ZilNodeType in OBJECT sub group 'return_string'"))),
        };
    
        wrap!(writer.w(",\n"));
    
        Ok(())
    }

    pub fn return_fn(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(": () => "));
        wrap!(W::print(&root.children[1], 0, &mut writer));
        wrap!(writer.w(",\n"));
    
        Ok(())
    }

    pub fn return_string_array(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(format!(": () => [")));
    
        for i in 1..root.children.len() {
            match root.children[i].kind() {
                ZilNodeType::Text => wrap!(T::print(&root.children[1], 0, &mut writer)),
                ZilNodeType::Word => wrap!(W::print_with_quotes(&root.children[i], 0, &mut writer)),
                _ => return Err(OutputErr::from(HandlerErr::origin("Cannot handle unknown ZilNodeType in OBJECT sub group 'return_string_array'"))),
            };
    
            if i+1 < root.children.len() {
                wrap!(writer.w(", "));
            }
        }
    
        wrap!(writer.w("],\n"));
    
        Ok(()) 
    }

    pub fn return_int(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(": () => "));
        match is_int(&root.children[1]) {
            true => wrap!(W::print(&root.children[1], 0, &mut writer)),
            false => return Err(OutputErr::from(HandlerErr::origin(format!("Trying to parse not-an-int in OBJECT sub group 'return_int': {}", root.children[1]))))
        };
        wrap!(writer.w(",\n"));
    
        Ok(())
    }
    
    pub fn mut_int(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(": "));
        match is_int(&root.children[1]) {
            true => wrap!(W::print(&root.children[1], 0, &mut writer)),
            false => return Err(OutputErr::from(HandlerErr::origin(format!("Trying to parse not-an-int in OBJECT sub group 'mut_int': {}", root.children[1]))))
        };
        wrap!(writer.w(",\n"));
    
        Ok(())
    }
    
    pub fn mut_bools(root: &ZilNode, indent: u64, mut writer: &mut CustomBufWriter<File>) -> Result<(), OutputErr> {
        let spacer = (0..indent).map(|_| "  ").collect::<String>();
    
        wrap!(writer.w(format!("{}", spacer)));
        wrap!(W::print(&root.children[0], 0, &mut writer));
        wrap!(writer.w(format!(": {{ ")));
    
        for i in 1..root.children.len() {
            wrap!(W::print(&root.children[i], 0, &mut writer));
            wrap!(writer.w(": true"));
            if i+1 < root.children.len() {
                wrap!(writer.w(", "));
            }
        }
    
        wrap!(writer.w(" },\n"));
    
        Ok(()) 
    }
}