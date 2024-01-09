// I Language iterator tools.
// Version: 1.0.0

// Copyright (c) 2023-present I Language Development.

// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the 'Software'),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

/////////////
// IMPORTS //
/////////////

use std;

//////////////////////////
// CONDITIONAL PEEKING //
//////////////////////////

pub trait ConditionalPeeking<I: std::iter::Iterator> {
    fn peek_while(&mut self, closure: impl Fn(&I::Item) -> bool) -> Vec<I::Item>
    where
        <I as Iterator>::Item: Clone;

    fn peek_until(&mut self, closure: impl Fn(&I::Item) -> bool) -> Vec<I::Item>
    where
        <I as Iterator>::Item: Clone;
}

impl<I: std::iter::Iterator + Clone> ConditionalPeeking<I> for std::iter::Peekable<I> {
    fn peek_while(&mut self, closure: impl Fn(&I::Item) -> bool) -> Vec<I::Item>
    where
        <I as Iterator>::Item: Clone,
    {
        let mut result: Vec<I::Item> = vec![];
        while let Some(value) = self.peek().cloned() {
            if closure(&value) {
                result.push(value);
                self.next();
            } else {
                break;
            }
        }

        result
    }

    fn peek_until(&mut self, closure: impl Fn(&<I as Iterator>::Item) -> bool) -> Vec<I::Item>
    where
        <I as Iterator>::Item: Clone,
    {
        self.peek_while(|item| !closure(item))
    }
}
