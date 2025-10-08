// add fields for your implementation to use
struct PaginationHelper<T> {
    values: Vec<T>,
    pagination: usize,
}

impl<T> PaginationHelper<T> {
    fn new(collection: Vec<T>, items_per_page: usize) -> Self {
        Self {
            values: collection,
            pagination: items_per_page
        }
    }
    
    fn item_count(&self) -> usize {
        self.values.iter().count()
    }    
    
    fn page_count(&self) -> usize {
        self.values.iter().step_by(self.pagination).count()
    }
    
    fn page_item_count(&self, page_index: usize) -> Option<usize> {
        match self.values.iter().skip(page_index * self.pagination).count() {
            x if x == 0 => None,
            x if x <= self.pagination => Some(x),
            _ => Some(self.pagination),
        }
    }
    
    fn page_index(&self, item_index: usize) -> Option<usize> {
        if (item_index > self.values.iter().count()) | (self.values.iter().count() == 0) {
            return None;
        } else {
            return Some(item_index / self.pagination)
        }

    }
}

#[cfg(test)]
mod sample_tests {
    use super::PaginationHelper;

    #[test]
    fn test_item_count() {
        let helper = PaginationHelper::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 3);
        assert_eq!(helper.item_count(), 11);
    }

    #[test]
    fn test_page_count() {
        let helper = PaginationHelper::new(vec!['a', 'b', 'c', 'd', 'e', 'f'], 4);
        assert_eq!(helper.page_count(), 2);
    }

    #[test]
    fn test_page_item_count() {
        let helper = PaginationHelper::new((1..=24).collect(), 10);
        assert_eq!(helper.page_item_count(1), Some(10));
        assert_eq!(helper.page_item_count(2), Some(4));
        assert_eq!(helper.page_item_count(3), None);
    }

    #[test]
    fn test_page_index() {
        let helper = PaginationHelper::new((1..=24).collect(), 10);
        assert_eq!(helper.page_index(40), None);
        assert_eq!(helper.page_index(22), Some(2));
    }

    #[test]
    fn test_empty_collection() {
        let helper: PaginationHelper<i32> = PaginationHelper::new(vec![], 10);
        assert_eq!(helper.page_index(0), None);
    }
}