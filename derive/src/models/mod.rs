/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

pub mod edge;
pub mod node;
pub mod object;


#[derive(Debug, Copy, Clone)]
pub enum DataType {
    Node,
    Edge,
    Object,
}

impl DataType {
    pub fn is_node_or_edge(&self) -> bool {
        matches!(self, Self::Node | Self::Edge)
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object)
    }

    pub fn is_edge(&self) -> bool {
        matches!(self, Self::Edge)
    }
}
