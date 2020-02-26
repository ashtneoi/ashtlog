import sys


class LogReceiver:
    def receive(self, entry, node):
        raise Exception("unimplemented")


class LogNode:
    locked = False

    def __init__(self, receiver, parent=None, name=None):
        self.receiver = receiver
        self.parent = parent
        self.name = name

    def __enter__(self):
        if self.parent:
            self.parent.locked = True
        return self

    def __exit__(self, _exc_type, _exc_val, _exc_tb):
        if self.parent:
            self.parent.locked = False

    def put(self, entry):
        if self.locked:
            raise Exception("locked")
        self.receiver.receive(entry, self)

    def child(self, entry):
        if self.locked:
            raise Exception("locked")
        self.put(entry)
        return LogNode(self.receiver, parent=self)

    def child_shared(self, name):
        if self.locked:
            raise Exception("locked")
        return LogNode(self.receiver, parent=self, name=name)


# TODO: Implement this in other reasonable ways and compare them.
class Standard(LogReceiver):
    def __init__(self, target):
        self.target = target

    def receive(self, entry, node):
        indentation = -1
        n = node
        nodes = []  # leaf to root
        while n:
            indentation += 1
            nodes.append(n)
            n = n.parent

        s = []
        s.append('  ' * indentation)
        s.append('[')
        for n in reversed(nodes):  # root to leaf
            if n.parent and n.parent.name and n.name:
                s.append('/')
            if n.name:
                s.append(n.name)  # FIXME: Escape '/' and '>'.
            else:
                s.append('>')
        s.append('] ')
        s.append(entry)
        s.append('\n')
        self.target.write("".join(s))


if __name__ == '__main__':
    rec = Standard(sys.stdout)
    log = LogNode(rec)
    log.put("hi")
    with log.child("child 1") as log_child1:
        log_child1.put("okay")
        log_child1.put("yeah")
    with log.child("child 2") as log_child2:
        log_child2.put("idk")
    out = log.child_shared('stdout')
    err = log.child_shared('stderr')
    out.put("everything is fine")
    out_more = out.child_shared("doing-stuff")
    out_more.put("okay doing stuff")
    err.put("oh no")
    out.put("it's fine")
    log.put("bye")
