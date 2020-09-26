def escape(x, escaped, escape_char):
    xx = []
    for c in x:
        if c in escaped:
           xx.append(escape_char + c)
        else:
            xx.append(c)
    return "".join(xx)


class LogBackend:
    def put(self, entry):
        pass

    def child(self, entry):
        pass

    def child_shared(self, name):
        pass


class NullLogBackend(LogBackend):
    def put(self, entry):
        pass

    def child(self, entry):
        pass

    def child_shared(self, name):
        pass


class PlainLogBackend(LogBackend):
    def __init__(self, parent=None, name=None):
        self.parent = parent
        self.name = name
        super().__init__()

    def _get_path(self):
        segments = []
        here = self
        while here is not None:
            if here.name is None:
                segments.append(">")
            else:
                segments.append(escape(here.name, "/>\\", "\\"))
            here = here.parent
        segments.reverse()
        for i in range(1, len(segments)):
            if segments[i - 1] != ">" and segments[i] != ">":
                segments[i] = "/" + segments[i]

        return "".join(segments), len(segments)

    def put(self, entry):
        path, indent = self._get_path()
        print(f"{'  ' * indent}[{path}] {entry}")

    def child(self, entry):
        self.put(entry)
        return type(self)(self)

    def child_shared(self, name):
        return type(self)(self, name=name)
