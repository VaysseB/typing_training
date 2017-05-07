
use std::fmt::{Display, Formatter, Result};
use std::convert::Into;
use std::iter::Iterator;

pub struct DisplayablePairOfIterators<'a, A, B>(Into<Iterator<Item=&'a A>>, Into<Iterator<Item=&'a B>>)
    where A: Display + ?Sized, B: Display + ?Sized;

impl<'a, A, B> Display for DisplayablePairOfIterators<'a, &'a A, &'a B>
    where A: Display + ?Sized, B: Display + ?Sized {
    fn fmt(&self, f: &mut Formatter) -> Result {
//        for (&a, &b) in self.0.into().zip(self.1.into()) {
//            try!(write!(f, "{}{}", a, b))
