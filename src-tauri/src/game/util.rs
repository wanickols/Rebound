pub struct Util();

impl Util {
    pub fn two_mut<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
        assert!(i != j);
        if i < j {
            let (left, right) = slice.split_at_mut(j);
            (&mut left[i], &mut right[0])
        } else {
            let (left, right) = slice.split_at_mut(i);
            (&mut right[0], &mut left[j])
        }
    }
}
