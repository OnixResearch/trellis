#!/usr/bin/env python3
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Set

FN_RE = re.compile(
    r"^\s*(?:pub\s+)?(?:(?:open|closed)\s+)?(?:(?:uninterp)\s+)?(?:(spec|proof)\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)"
)

@dataclass
class Function:
    name: str
    kind: str  # exec | spec | proof
    path: Path
    line: int
    body: str


def extract_functions(path: Path) -> List[Function]:
    text = path.read_text()
    lines = text.splitlines()
    out: List[Function] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        m = FN_RE.match(line)
        if not m:
            i += 1
            continue
        kind = "exec" if m.group(1) is None else m.group(1)
        name = m.group(2)
        start = i
        brace_depth = line.count("{") - line.count("}")
        j = i + 1
        while j < len(lines) and brace_depth <= 0:
            brace_depth += lines[j].count("{") - lines[j].count("}")
            j += 1
        while j < len(lines) and brace_depth > 0:
            brace_depth += lines[j].count("{") - lines[j].count("}")
            j += 1
        body = "\n".join(lines[start:j])
        out.append(Function(name=name, kind=kind, path=path, line=start + 1, body=body))
        i = j
    return out


def bare_call_edges(exec_functions: List[Function]) -> Dict[str, Set[str]]:
    names = [fn.name for fn in exec_functions]
    edges: Dict[str, Set[str]] = {fn.name: set() for fn in exec_functions}
    patterns = {
        name: re.compile(rf"(?<![.:A-Za-z0-9_]){re.escape(name)}\s*\(")
        for name in names
    }
    for fn in exec_functions:
        body = "\n".join(fn.body.splitlines()[1:])
        for other in names:
            if patterns[other].search(body):
                edges[fn.name].add(other)
    return edges


def find_cycles(edges: Dict[str, Set[str]]) -> List[List[str]]:
    cycles: List[List[str]] = []
    temp: Set[str] = set()
    perm: Set[str] = set()
    stack: List[str] = []

    def visit(node: str):
        if node in perm:
            return
        if node in temp:
            if node in stack:
                idx = stack.index(node)
                cycles.append(stack[idx:] + [node])
            return
        temp.add(node)
        stack.append(node)
        for nxt in sorted(edges[node]):
            visit(nxt)
        stack.pop()
        temp.remove(node)
        perm.add(node)

    for node in sorted(edges):
        visit(node)
    return cycles


def main(argv: List[str]) -> int:
    if len(argv) < 2:
        print("usage: check-iterative-exec.py <file> [<file> ...]", file=sys.stderr)
        return 2

    files = [Path(arg) for arg in argv[1:]]
    missing = [str(path) for path in files if not path.exists()]
    if missing:
        for path in missing:
            print(f"missing: {path}", file=sys.stderr)
        return 2

    functions: List[Function] = []
    for path in files:
        functions.extend(extract_functions(path))

    exec_functions = [fn for fn in functions if fn.kind == "exec"]
    edges = bare_call_edges(exec_functions)
    cycles = find_cycles(edges)

    print("iterative-exec audit")
    print(f"files: {', '.join(str(path) for path in files)}")
    print(f"exec functions: {len(exec_functions)}")
    for fn in exec_functions:
        callees = ", ".join(sorted(edges[fn.name])) or "(none)"
        print(f"- {fn.name} @ {fn.path}:{fn.line} -> {callees}")

    if cycles:
        print("result: FAIL")
        for cycle in cycles:
            print("cycle: " + " -> ".join(cycle))
        return 1

    print("result: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
