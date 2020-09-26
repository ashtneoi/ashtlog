use ashtlog::{
    log,
    log_child,
    PlainLogBackend,
    LogNode,
};

fn main() {
    let r = PlainLogBackend;
    let mut a = LogNode::new(&r);

    let mut b = log_child!(a, "b");
    log!(b, "1");
    let mut c = b.child_shared("c");
    log!(c, "2");
    let mut d = b.child_shared("d");
    log!(d, "3");
    let mut e = log_child!(a, "e");
    log!(e, "4");
    let mut f = e.child_shared("f");
    log!(f, "5");
    log!(a, "6");

    let g = log_child!(a, "g");
    let h = g.child_shared("h");
    let mut i = h.child_shared("i");
    let mut j = log_child!(i, "j");
    let mut k = log_child!(j, "k");
    log!(k, "7");
}
