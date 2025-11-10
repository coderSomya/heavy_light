/// Trait for combining two segment tree values
pub trait CombineFn<T> {
    fn combine(&self, a: T, b: T) -> T;
}

/// Trait for applying lazy updates
pub trait LazyApplyFn<T> {
    fn apply(&self, lazy_val: T, new_val: T) -> T;
}

/// Trait for applying lazy value to current value
pub trait LazyFunc<T> {
    fn apply(&self, cur_val: T, lazy_val: T, l: usize, r: usize) -> T;
}

/// Default lazy apply function - simply replaces with new value
#[derive(Clone, Copy, Default)]
pub struct DefaultLazyApply;

impl<T> LazyApplyFn<T> for DefaultLazyApply {
    fn apply(&self, _lazy_val: T, new_val: T) -> T {
        new_val
    }
}

/// Default lazy function - replaces current value with lazy value
#[derive(Clone, Copy, Default)]
pub struct DefaultLazyFunc;

impl<T> LazyFunc<T> for DefaultLazyFunc {
    fn apply(&self, _cur_val: T, lazy_val: T, _l: usize, _r: usize) -> T {
        lazy_val
    }
}

/// Generic segment tree with lazy propagation
pub struct SegmentTree<T, C, LA, LF> 
where
    T: Clone + Copy + Default,
    C: CombineFn<T>,
    LA: LazyApplyFn<T>,
    LF: LazyFunc<T>,
{
    n: usize,
    seg_tree: Vec<T>,
    seg_lazy: Vec<Option<T>>,
    combine_fn: C,
    lazy_apply_fn: LA,
    lazy_func: LF,
    sentinel: T,
    lazy_sentinel: Option<T>,
}

impl<T, C, LA, LF> SegmentTree<T, C, LA, LF>
where
    T: Clone + Copy + Default + PartialEq,
    C: CombineFn<T>,
    LA: LazyApplyFn<T>,
    LF: LazyFunc<T>,
{
    pub fn new(n: usize, combine_fn: C, lazy_apply_fn: LA, lazy_func: LF, sentinel: T, lazy_sentinel: Option<T>) -> Self {
        let size = 4 * n;
        Self {
            n,
            seg_tree: vec![T::default(); size],
            seg_lazy: vec![lazy_sentinel; size],
            combine_fn,
            lazy_apply_fn,
            lazy_func,
            sentinel,
            lazy_sentinel,
        }
    }

    pub fn query(&mut self, l: usize, r: usize) -> T {
        self.query_rec(0, 0, self.n - 1, l, r)
    }

    fn query_rec(&mut self, i: usize, tl: usize, tr: usize, ql: usize, qr: usize) -> T {
        self.eval_lazy(i, tl, tr);

        if ql <= tl && tr <= qr {
            return self.seg_tree[i];
        }

        if tl > tr || tr < ql || qr < tl {
            return self.sentinel;
        }

        let mid = (tl + tr) / 2;
        let a = self.query_rec(2 * i + 1, tl, mid, ql, qr);
        let b = self.query_rec(2 * i + 2, mid + 1, tr, ql, qr);
        self.combine_fn.combine(a, b)
    }

    pub fn update(&mut self, l: usize, r: usize, val: T) {
        self.update_rec(0, 0, self.n - 1, l, r, val);
    }

    fn update_rec(&mut self, i: usize, tl: usize, tr: usize, ql: usize, qr: usize, val: T) -> T {
        self.eval_lazy(i, tl, tr);

        if tl > tr || tr < ql || qr < tl {
            return self.seg_tree[i];
        }

        if ql <= tl && tr <= qr {
            self.seg_lazy[i] = Some(self.lazy_apply_fn.apply(
                self.seg_lazy[i].unwrap_or(val),
                val
            ));
            self.eval_lazy(i, tl, tr);
            return self.seg_tree[i];
        }

        if tl == tr {
            return self.seg_tree[i];
        }

        let mid = (tl + tr) / 2;
        let a = self.update_rec(2 * i + 1, tl, mid, ql, qr, val);
        let b = self.update_rec(2 * i + 2, mid + 1, tr, ql, qr, val);
        self.seg_tree[i] = self.combine_fn.combine(a, b);
        self.seg_tree[i]
    }

    fn eval_lazy(&mut self, i: usize, l: usize, r: usize) {
        if self.seg_lazy[i] == self.lazy_sentinel {
            return;
        }

        if let Some(lazy_val) = self.seg_lazy[i] {
            self.seg_tree[i] = self.lazy_func.apply(self.seg_tree[i], lazy_val, l, r);

            if l != r {
                let left_idx = 2 * i + 1;
                let right_idx = 2 * i + 2;
                
                self.seg_lazy[left_idx] = Some(self.lazy_apply_fn.apply(
                    self.seg_lazy[left_idx].unwrap_or(lazy_val),
                    lazy_val
                ));
                self.seg_lazy[right_idx] = Some(self.lazy_apply_fn.apply(
                    self.seg_lazy[right_idx].unwrap_or(lazy_val),
                    lazy_val
                ));
            }

            self.seg_lazy[i] = self.lazy_sentinel;
        }
    }

    pub fn get_sentinel(&self) -> T {
        self.sentinel
    }

    pub fn point_update(&mut self, idx: usize, val: T) {
        self.update(idx, idx, val);
    }
}
