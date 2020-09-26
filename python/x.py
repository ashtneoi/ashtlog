from ashtlog import PlainLogBackend


def main():
    a = PlainLogBackend()
    b = a.child("b")
    b.put("1")
    c = b.child_shared("c")
    c.put("2")
    d = b.child_shared("d")
    d.put("3")
    e = a.child("e")
    e.put("4")
    f = e.child_shared("f")
    f.put("5")
    a.put("6")

    g = a.child("g")
    h = g.child_shared("h")
    i = h.child_shared("i")
    j = i.child("j")
    k = j.child("k")
    k.put("7")


if __name__ == "__main__":
    main()
