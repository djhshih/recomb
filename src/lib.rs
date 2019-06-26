extern crate bitvec;

use bitvec::prelude::*;
use std::cmp;

pub fn recombine(xs: &BitSlice, ys: &BitSlice, breaks: &[usize]) -> BitVec {
    let n = cmp::max(xs.len(), ys.len());
    let mut zs = BitVec::with_capacity(n);
    let mut l = 0;
    let mut ts = xs;
    for &r in breaks {
        if r <= ts.len() {
            for i in l..r {
                zs.push(ts[i]);
            }
            l = r;
        }
        // switch to other slice
        if ts as *const _ == xs as *const _ {
            ts = ys;
        } else {
            ts = xs;
        }
        if r > ts.len() {
            break;
        }
    }
    // finish copying the final segment
    for i in l..ts.len() {
        zs.push(ts[i]);
    }
    
    zs
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recombine_eq_len() {
        let xs = bitvec![0, 1, 1, 1, 0, 1, 0];
        let ys = bitvec![1, 1, 1, 0, 0, 1, 1];
        let zs = recombine(&xs, &ys, &[2]);
        let ss = bitvec![0, 1, 1, 0, 0, 1, 1];
        assert_eq!(&zs, &ss);
        let zs = recombine(&xs, &ys, &[0, 3, 5]);
        let ss = bitvec![1, 1, 1, 1, 0, 1, 1];
        assert_eq!(&zs, &ss);
        let zs = recombine(&xs, &ys, &[6]);
        let ss = bitvec![0, 1, 1, 1, 0, 1, 1];
        assert_eq!(&zs, &ss);
    }
    
    #[test]
    fn test_recombine_xs_short() {
        let xs = bitvec![0, 1, 1, 1, 0];
        let ys = bitvec![1, 1, 1, 0, 0, 1, 1];
        let zs = recombine(&xs, &ys, &[]);
        let ss = bitvec![0, 1, 1, 1, 0];
        assert_eq!(&zs, &ss);
        let zs = recombine(&xs, &ys, &[5]);
        let ss = bitvec![0, 1, 1, 1, 0, 1, 1];
        assert_eq!(&zs, &ss);
    }
    
    #[test]
    fn test_recombine_ys_short() {
        let xs = bitvec![0, 1, 1, 1, 0, 0, 0];
        let ys = bitvec![1, 1, 1];
        let zs = recombine(&xs, &ys, &[]);
        let ss = bitvec![0, 1, 1, 1, 0, 0, 0];
        assert_eq!(&zs, &ss);
        let zs = recombine(&xs, &ys, &[3]);
        let ss = bitvec![0, 1, 1];
        assert_eq!(&zs, &ss);
        let zs = recombine(&xs, &ys, &[0, 3]);
        let ss = bitvec![1, 1, 1, 1, 0, 0, 0];
        assert_eq!(&zs, &ss);
    }
}
