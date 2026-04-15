#!/usr/bin/env python3
import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Sequence, Set

FN_RE = re.compile(
    r"^\s*(?:pub\s+)?(?:(?:open|closed)\s+)?(?:(?:uninterp)\s+)?(?:(spec|proof)\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)"
)


@dataclass(frozen=True)
class Function:
    name: str
    kind: str  # exec | spec | proof
    path: Path
    line: int
    body: str


@dataclass(frozen=True)
class QualifiedCallViolation:
    caller: Function
    callee_name: str
    line: int
    snippet: str


@dataclass(frozen=True)
class DuplicateNameViolation:
    path: Path
    name: str
    lines: tuple[int, ...]


@dataclass
class ModuleAudit:
    path: Path
    exec_functions: List[Function]
    edges: Dict[Function, Set[Function]]
    qualified_violations: List[QualifiedCallViolation]
    duplicate_violations: List[DuplicateNameViolation]
    cycles: List[List[Function]]


def parse_args(argv: Sequence[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Audit scheduler/fault-schedule exec helpers for module-local bare-call cycles."
        )
    )
    parser.add_argument("files", nargs="+", help="Rust source files to audit")
    return parser.parse_args(argv[1:])



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
        brace_depth = 0
        saw_open_brace = False
        j = i
        while j < len(lines):
            current = lines[j]
            for ch in current:
                if ch == "{":
                    brace_depth += 1
                    saw_open_brace = True
                elif ch == "}":
                    brace_depth -= 1
            j += 1
            if saw_open_brace and brace_depth == 0:
                break
        body = "\n".join(lines[start:j])
        out.append(Function(name=name, kind=kind, path=path, line=start + 1, body=body))
        i = j
    return out



def function_body_text(function: Function) -> str:
    idx = function.body.find("{")
    if idx < 0:
        return ""
    return function.body[idx + 1 :]



def line_for_offset(function: Function, text: str, offset: int) -> int:
    return function.line + text[:offset].count("\n")



def find_duplicate_exec_names(exec_functions: Sequence[Function]) -> List[DuplicateNameViolation]:
    by_name: Dict[str, List[Function]] = {}
    for function in exec_functions:
        by_name.setdefault(function.name, []).append(function)

    duplicates: List[DuplicateNameViolation] = []
    for name, defs in sorted(by_name.items()):
        if len(defs) > 1:
            duplicates.append(
                DuplicateNameViolation(
                    path=defs[0].path,
                    name=name,
                    lines=tuple(sorted(fn.line for fn in defs)),
                )
            )
    return duplicates



def bare_call_pattern(name: str) -> re.Pattern[str]:
    return re.compile(rf"(?<![.:A-Za-z0-9_]){re.escape(name)}\s*\(")



def qualified_call_pattern(name: str) -> re.Pattern[str]:
    return re.compile(
        rf"(?<![A-Za-z0-9_])(?:self|Self|super|crate|[A-Za-z_][A-Za-z0-9_]*)(?:::[A-Za-z_][A-Za-z0-9_]*)*::{re.escape(name)}\s*\("
    )



def build_module_audit(path: Path) -> ModuleAudit:
    functions = extract_functions(path)
    exec_functions = [fn for fn in functions if fn.kind == "exec"]
    duplicate_violations = find_duplicate_exec_names(exec_functions)

    edges: Dict[Function, Set[Function]] = {fn: set() for fn in exec_functions}
    qualified_violations: List[QualifiedCallViolation] = []

    if duplicate_violations:
        return ModuleAudit(
            path=path,
            exec_functions=exec_functions,
            edges=edges,
            qualified_violations=qualified_violations,
            duplicate_violations=duplicate_violations,
            cycles=[],
        )

    by_name = {fn.name: fn for fn in exec_functions}
    bare_patterns = {name: bare_call_pattern(name) for name in by_name}
    qualified_patterns = {name: qualified_call_pattern(name) for name in by_name}

    for function in exec_functions:
        body_text = function_body_text(function)
        for name, pattern in bare_patterns.items():
            if pattern.search(body_text):
                edges[function].add(by_name[name])
        for name, pattern in qualified_patterns.items():
            for match in pattern.finditer(body_text):
                qualified_violations.append(
                    QualifiedCallViolation(
                        caller=function,
                        callee_name=name,
                        line=line_for_offset(function, body_text, match.start()),
                        snippet=match.group(0).strip(),
                    )
                )

    cycles = find_cycles(edges)
    return ModuleAudit(
        path=path,
        exec_functions=exec_functions,
        edges=edges,
        qualified_violations=qualified_violations,
        duplicate_violations=duplicate_violations,
        cycles=cycles,
    )



def find_cycles(edges: Dict[Function, Set[Function]]) -> List[List[Function]]:
    cycles: List[List[Function]] = []
    temp: Set[Function] = set()
    perm: Set[Function] = set()
    stack: List[Function] = []

    def visit(node: Function) -> None:
        if node in perm:
            return
        if node in temp:
            if node in stack:
                idx = stack.index(node)
                cycles.append(stack[idx:] + [node])
            return
        temp.add(node)
        stack.append(node)
        for nxt in sorted(edges[node], key=lambda fn: (str(fn.path), fn.line, fn.name)):
            visit(nxt)
        stack.pop()
        temp.remove(node)
        perm.add(node)

    for node in sorted(edges, key=lambda fn: (str(fn.path), fn.line, fn.name)):
        visit(node)
    return cycles



def format_function(function: Function) -> str:
    return f"{function.name} @ {function.path}:{function.line}"



def main(argv: Sequence[str]) -> int:
    args = parse_args(argv)
    files = [Path(arg) for arg in args.files]
    missing = [str(path) for path in files if not path.exists()]
    if missing:
        for path in missing:
            print(f"missing: {path}", file=sys.stderr)
        return 2

    audits = [build_module_audit(path) for path in files]

    print("iterative-exec audit")
    print(f"files: {', '.join(str(path) for path in files)}")

    failed = False
    for audit in audits:
        print(f"module: {audit.path}")
        print(f"exec functions: {len(audit.exec_functions)}")
        for function in sorted(audit.exec_functions, key=lambda fn: (fn.line, fn.name)):
            callees = ", ".join(sorted(fn.name for fn in audit.edges[function])) or "(none)"
            print(f"- {format_function(function)} -> {callees}")

        if audit.duplicate_violations:
            failed = True
            print("duplicate bare-name violations:")
            for violation in audit.duplicate_violations:
                lines = ", ".join(str(line) for line in violation.lines)
                print(
                    f"- {violation.path}: exec helper name '{violation.name}' appears on lines {lines}"
                )

        if audit.qualified_violations:
            failed = True
            print("qualified local-call violations:")
            for violation in audit.qualified_violations:
                print(
                    f"- {format_function(violation.caller)} uses qualified local call {violation.snippet!r} at line {violation.line}"
                )

        if audit.cycles:
            failed = True
            print("cycles:")
            for cycle in audit.cycles:
                print("- " + " -> ".join(format_function(fn) for fn in cycle))

    print(f"result: {'FAIL' if failed else 'PASS'}")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
