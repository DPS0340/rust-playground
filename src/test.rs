#[cfg(test)]
mod tests {
    use crate::reduce::get_min;
    use std::{option::Option, vec::Vec};

    fn test_min(xs: &Vec<i32>) -> (bool, Option<&i32>) {
        let res = get_min(xs);

        let mut copied_xs = xs.to_vec();
        copied_xs.sort();

        let expect = copied_xs.first();

        (
            match (res, expect) {
                (Some(x), Some(y)) => *x == *y,
                (_, _) => false,
            },
            res,
        )
    }

    #[test]
    fn test_min_asc() {
        let xs = vec![1, 2, 3, 4, 5];

        let (res, elem) = test_min(&xs);

        assert_eq!(res, true);
        assert_eq!(*elem.unwrap(), 1);
    }

    #[test]
    fn test_min_desc() {
        let mut xs = vec![1, 2, 3, 4, 5];
        xs.reverse();

        let (res, elem) = test_min(&xs);

        assert_eq!(res, true);
        assert_eq!(*elem.unwrap(), 1);
    }

    #[test]
    fn test_min_empty() {
        let xs = vec![];

        let (res, elem) = test_min(&xs);

        assert_eq!(res, false);
        assert_eq!(elem.is_none(), true);
    }
}
