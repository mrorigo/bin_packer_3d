use crate::block::{Block, Dimension};
use crate::item::Item;
use std::iter::FromIterator;

/// Represents an item that a user will insert into a bin.
#[derive(Clone, Debug)]
pub struct Bin {
    /// Represents the cuboid of this Bin.
    block: Block,
}

impl Bin {
    /// Creates a new Bin from it's dimensions.
    pub fn new(dims: [Dimension; 3]) -> Self {
        Self {
            block: Block::new(dims[0], dims[1], dims[2]),
        }
    }

    /// Returns whether or not the Bin's dimensions can emcompass or match the Bin.
    pub fn does_item_fit(&self, item: &Item<'_>) -> bool {
        self.block.does_it_fit(&item.block)
    }

    /// Returns the remaining bins after the item has been added to the current bin.
    pub fn best_fit(self, item: &Item<'_>) -> Option<Vec<Bin>> {
        Some(self.block.best_fit(&item.block)?
            .into_iter()
            .map(|block| Bin::from(block))
            .collect())
    }
}

impl From<Block> for Bin {
    fn from(block: Block) -> Self {
        Bin::new([block.dims[0], block.dims[1], block.dims[2]])
    }
}
