#!/usr/bin/env long-shebang
#!nim c -r

func escape(x: string): string =
    for c in x:
        if c == '/' or c == '>' or c == '\\':
            result.add('\\')
        result.add(c)

type LogBackend = object of RootObj

type PlainLogBackend {.final.} = object of LogBackend
    parent: ref PlainLogBackend
    name: string

proc put(self: ref PlainLogBackend, entry: string) =
    var segments: seq[string] = @[]
    var here = self
    while here != nil:
        if here.name == "":
            segments.add(">")
        else:
            segments.add(escape(here.name))
    echo entry

# echo escape(r"/h\i>>")
# let p = new PlainLogBackend(parent: nil, name: "")
# p.put("hihihi")
let x = PlainLogBackend.ee
