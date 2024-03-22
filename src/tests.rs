use crate::bin::Bin;
use crate::block::{Block, Dimension};
use crate::error::Result;
use crate::item::Item;

mod block {
    use super::*;

    #[test]
    fn test_block_creation() -> Result<()> {
        Block::new(1, 2, 3);
        Ok(())
    }

    #[test]
    fn test_item_creation() -> Result<()> {
        Item::new(&1, [1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_bin_creation() -> Result<()> {
        Bin::new([1 as Dimension, 2 as Dimension, 3 as Dimension]);
        Ok(())
    }

    #[test]
    fn test_block_volume() -> Result<()> {
        let b = Block::new(3 as Dimension, 4 as Dimension, 5 as Dimension);
        assert_eq!(b.volume(), 60 as Dimension);
        Ok(())
    }

    #[test]
    fn test_block_volume_large_values() -> Result<()> {
        let b = Block::new(200 as Dimension, 100 as Dimension, 200 as Dimension);
        assert_eq!(b.volume(), 4_000_000 as Dimension);
        Ok(())
    }

    #[test]
    fn test_block_does_it_fit() -> Result<()> {
        // test that when an item fits, it returns true
        let item = Block::new(35, 140, 127);
        let container = Block::new(40 as Dimension, 220 as Dimension, 140 as Dimension);
        assert!(container.does_it_fit(&item));
        Ok(())
    }

    #[test]
    fn test_block_does_it_fit_false() -> Result<()> {
        // test that when a item does not fit, it returns false
        let item = Block::new(4 as Dimension, 12 as Dimension, 14 as Dimension);
        let container = Block::new(3 as Dimension, 14 as Dimension, 14 as Dimension);
        assert!(!container.does_it_fit(&item));
        Ok(())
    }

    #[test]
    fn test_best_fit_nil() -> Result<()> {
        // assert that if a item does not fit in the container,
        // we get None returned
        let item = Block::new(4 as Dimension, 12 as Dimension, 14 as Dimension);
        let container = Block::new(3 as Dimension, 14 as Dimension, 14 as Dimension);
        assert_eq!(container.best_fit(&item), None);
        Ok(())
    }

    #[test]
    fn test_best_fit_exact_size() -> Result<()> {
        // assert that if a item is the same size as the container, the remaining_dimensions comes back
        // empty

        let item = Block::new(13 as Dimension, 13 as Dimension, 31 as Dimension);
        let container = Block::new(13 as Dimension, 13 as Dimension, 31 as Dimension);
        assert_eq!(container.best_fit(&item), Some(vec![]));
        Ok(())
    }

    #[test]
    fn test_best_fit_half_size() -> Result<()> {
        // Assert that if a item is smaller than the container, but has two dimensions the same, it will
        // return the empty space

        let item = Block::new(13 as Dimension, 13 as Dimension, 31 as Dimension);
        let container = Block::new(13 as Dimension, 26 as Dimension, 31 as Dimension);
        assert_eq!(
            container.best_fit(&item),
            Some(vec![Block::new(
                13 as Dimension,
                13 as Dimension,
                31 as Dimension
            )])
        );
        Ok(())
    }

    #[test]
    fn test_best_fit_first_fit_greater_than() -> Result<()> {
        // test that the "greater than" match clause of the first fit returns the
        // correct remaining space.
        let item = Block::new(125, 700, 1000);
        let container = Block::new(350, 950, 1250);
        assert_eq!(
            container.best_fit(&item),
            Some(vec![
                Block::new(125, 250, 700),
                Block::new(250, 350, 1250),
                Block::new(225, 700, 1250)
            ])
        );
        Ok(())
    }

    #[test]
    fn test_best_fit_multiple_spaces_1_2_2() -> Result<()> {
        // test to ensure that our 2x theorum is working
        let item = Block::new(1 as Dimension, 1 as Dimension, 1 as Dimension);
        let container = Block::new(1 as Dimension, 2 as Dimension, 2 as Dimension);
        assert_eq!(
            container.best_fit(&item),
            Some(vec![
                Block::new(1 as Dimension, 1 as Dimension, 1 as Dimension),
                Block::new(1 as Dimension, 1 as Dimension, 2 as Dimension)
            ])
        );
        Ok(())
    }

    #[test]
    fn test_best_fit_multiple_spaces() -> Result<()> {
        // assert that if a item is smaller than the container, but has two dimensions
        // the same, it will return the empty space
        let item = Block::new(13 as Dimension, 13 as Dimension, 31 as Dimension);
        let (x, y, z) = (20 as Dimension, 20 as Dimension, 31 as Dimension);
        let container = Block::new(x, y, z);
        assert_eq!(
            container.best_fit(&item),
            Some(vec![
                Block::new(7 as Dimension, 13 as Dimension, 31 as Dimension),
                Block::new(7 as Dimension, 20 as Dimension, 31 as Dimension)
            ])
        );
        Ok(())
    }
}
