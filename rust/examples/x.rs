use ashtlog::{
    PlainLogReceiver,
    LogNode,
};

fn main() {
    let r = PlainLogReceiver;
    let mut a = LogNode::new(&r);

    let mut b = a.child(format_args!("b"));
    b.put(format_args!("1"));
    let mut c = b.child_shared("c");
    c.put(format_args!("2"));
    let mut d = b.child_shared("d");
    d.put(format_args!("3"));
    let mut e = a.child(format_args!("e"));
    e.put(format_args!("4"));
    let mut f = e.child_shared("f");
    f.put(format_args!("5"));
    a.put(format_args!("6"));

    let g = a.child(format_args!("g"));
    let h = g.child_shared("h");
    let mut i = h.child_shared("i");
    let mut j = i.child(format_args!("j"));
    let mut k = j.child(format_args!("k"));
    k.put(format_args!("7"));
}
