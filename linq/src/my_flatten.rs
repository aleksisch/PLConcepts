
pub struct Flatten<I, I2> {
    pub(crate) it: I,
    pub(crate) inner_it: Option<I2>,
}

impl<I, I2> Flatten<I, I2> {
}

impl<I: Iterator> Iterator for Flatten<I, I::Item>
where I::Item: Iterator {
    type Item = <<I as Iterator>::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner_it.as_mut()?.next() {
            Some(val) => Some(val),
            None => {
                self.inner_it = self.it.next();
                self.inner_it.as_mut()?.next()
            }
        }
    }
}
