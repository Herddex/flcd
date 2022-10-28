use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::{mem, slice};
use std::fmt::{Display, Formatter};

struct HashNode {
    key: String,
    value: u32,
    next_node: Option<Box<HashNode>>
}
type PtrHashNode = Option<Box<HashNode>>;

pub struct SymbolTable<'a> {
    table: &'a mut [Option<Box<HashNode>>],
    table_size: usize,
    next_code: u32
}

const INITIAL_TABLE_SIZE: usize = 7;
const LOAD_FACTOR: f64 = 0.7;

impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        let layout = Layout::array::<PtrHashNode>(INITIAL_TABLE_SIZE).unwrap();
        let table = unsafe {
            slice::from_raw_parts_mut(alloc_zeroed(layout) as *mut PtrHashNode, INITIAL_TABLE_SIZE)
        };

        let new_symbol_table = SymbolTable {
            table,
            table_size: INITIAL_TABLE_SIZE,
            next_code: 0
        };

        new_symbol_table
    }

    pub fn search(&self, key: &str) -> Option<u32> {
        let hash = self.hash(key);
        let mut current_node_ref = &self.table[hash];

        while let Some(ref current_node) = current_node_ref {
            if current_node.key.eq(key) {
                return Some(current_node.value)
            }
            current_node_ref = &current_node.next_node;
        }

        None
    }

    pub fn insert(&mut self, key: &str) {
        if (self.next_code + 1) as f64 / self.table_size as f64 > LOAD_FACTOR {
            self.resize();
        }

        let hash = self.hash(key);

        self.table[hash] = Some(Box::new(HashNode {
            key: String::from(key),
            value: self.next_code,
            next_node: mem::replace(&mut self.table[hash], None)
        }));

        self.next_code += 1;
    }

    fn hash(&self, key: &str) -> usize {
        key.bytes()
            .map(|byte| byte as usize)
            .sum::<usize>() % self.table_size
    }

    fn resize(&mut self) {
        let old_layout = Layout::array::<PtrHashNode>(self.table_size).unwrap();

        self.table_size = self.table_size * 2 + 1;
        let new_layout = Layout::array::<PtrHashNode>(self.table_size).unwrap();
        let new_table = unsafe {
            slice::from_raw_parts_mut(alloc_zeroed(new_layout) as *mut PtrHashNode, self.table_size)
        };

        let old_table = mem::replace(&mut self.table, new_table);
        let ptr_old_table = old_table.as_mut_ptr() as *mut u8;

        for list in old_table {
            let mut current_node_optional = mem::replace(list, None);
            while let Some(mut node_to_move) = current_node_optional {
                current_node_optional = mem::replace(&mut node_to_move.next_node, None);

                let new_hash = self.hash(&node_to_move.key);
                node_to_move.next_node = mem::replace(&mut self.table[new_hash], None);
                self.table[new_hash] = Some(node_to_move);
            }
        }

        unsafe { dealloc(ptr_old_table, old_layout) };
    }
}

impl<'a> Drop for SymbolTable<'a> {
    fn drop(&mut self) {
        let layout= Layout::array::<PtrHashNode>(self.table_size).unwrap();
        unsafe {
            dealloc(self.table.as_mut_ptr() as *mut u8, layout)
        }
    }
}

impl<'a> Display for SymbolTable<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for current_list in self.table.iter() {
            let mut current_node_ptr = current_list;
            while let Some(current_node) = current_node_ptr.as_ref() {
                write!(f, "-> [{}: {}]", current_node.key, current_node.value)?;
                current_node_ptr = &current_node.next_node;
            }
            write!(f, "-> NIL\n")?
        }
        Ok(())
    }
}