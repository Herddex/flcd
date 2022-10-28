use std::fmt::{Display, Formatter};
use std::ptr::{null_mut};

#[derive(Debug)]
pub struct ProgramInternalForm {
    node_list: PtrPIFNode,
    tail: *mut PtrPIFNode
}
type PtrPIFNode = Option<Box<PIFNode>>;

#[derive(Debug)]
struct PIFNode {
    token: String,
    class: u32,
    symbol_table_id: Option<u32>,
    next_node: PtrPIFNode
}

impl ProgramInternalForm {
    pub fn new() -> ProgramInternalForm {
        ProgramInternalForm {
            node_list: None,
            tail: null_mut()
        }
    }

    pub fn add(&mut self, token: &str, class: u32, symbol_table_id: Option<u32>) {
        let new_node = Some(Box::new(PIFNode {
            token: String::from(token),
            class,
            symbol_table_id,
            next_node: None
        }));
        if self.node_list.is_none() {
            self.node_list = new_node;
            self.tail = &mut self.node_list as *mut PtrPIFNode;
        }
        else {
            unsafe {
                (*self.tail).as_mut().unwrap().next_node = new_node;
                self.tail = &mut (*self.tail).as_mut().unwrap().next_node as *mut PtrPIFNode;
            }
        }
    }
}

impl Display for ProgramInternalForm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut current_node_ptr = &self.node_list;
        while let Some(current_node) = current_node_ptr.as_ref() {
            match current_node.symbol_table_id.as_ref() {
                Some(symbol_table_id) =>
                    writeln!(f, "{} {} {}", current_node.token, current_node.class, *symbol_table_id),
                None => writeln!(f, "{} {}", current_node.token, current_node.class),
            }?;
            current_node_ptr = &current_node.next_node;
        }
        Ok(())
    }
}