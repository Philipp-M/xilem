// Copyright 2023 the Druid Authors.
// SPDX-License-Identifier: Apache-2.0

pub struct VecSplice<'a, 'b, T> {
    v: &'a mut Vec<T>,
    scratch: &'b mut Vec<T>,
    ix: usize,
}

impl<'a, 'b, T> VecSplice<'a, 'b, T> {
    pub fn new(v: &'a mut Vec<T>, scratch: &'b mut Vec<T>) -> Self {
        let ix = 0;
        VecSplice { v, scratch, ix }
    }

    fn move_rest_at_ix_to_scratch(&mut self, ix: usize) {
        self.v.extend(self.scratch.splice(ix.., []).rev());
    }

    pub fn skip(&mut self, n: usize) {
        if self.v.len() < self.ix + n {
            let l = self.scratch.len().saturating_sub(n);
            self.move_rest_at_ix_to_scratch(l);
        }
        self.ix += n;
    }

    pub fn delete(&mut self, n: usize) {
        let len = self.v.len();
        let del_end_ix = self.ix + n;
        #[allow(clippy::comparison_chain)]
        if len < del_end_ix {
            let v_rest_len = len.saturating_sub(self.ix);
            let d = n.saturating_sub(v_rest_len);
            let scratch_start_ix = self.scratch.len().saturating_sub(d);
            self.scratch.truncate(scratch_start_ix);
        } else if len > del_end_ix {
            self.scratch.extend(self.v.splice(del_end_ix.., []).rev());
        }
        self.v.truncate(self.ix);
    }

    pub fn push(&mut self, value: T) {
        self.clear_tail();
        self.v.push(value);
        self.ix += 1;
    }

    pub fn mutate(&mut self) -> &mut T {
        if self.v.len() == self.ix {
            self.v.push(self.scratch.pop().unwrap());
        }
        let ix = self.ix;
        self.ix += 1;
        &mut self.v[ix]
    }

    pub fn len(&self) -> usize {
        self.ix
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_vec<R, F: FnOnce(&mut Vec<T>) -> R>(&mut self, f: F) -> R {
        self.clear_tail();
        let ret = f(self.v);
        self.ix = self.v.len();
        ret
    }

    fn clear_tail(&mut self) {
        if self.v.len() > self.ix {
            self.scratch.extend(self.v.splice(self.ix.., []).rev());
        }
    }
}
