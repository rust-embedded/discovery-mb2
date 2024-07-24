#!/usr/bin/python3

# How to use this tool:
# 1. Set up
# 1.1 make sure to be on a clean branch so that undo is easy
# 1.2 cd to mdbook/src
# 1.3 ensure that all chapters are in directories named ##-name
# 1.4 ensure that SUMMARY.md contains all chapters in correct order
# 2. python3 ../../rechapter.py
# 3. inspect rechapter.sh, rechapter.sed and SUMMARY.md.new
# 4. mv -f SUMMARY.md.new SUMMARY.md
# 5. sh rechapter.sh
# 6. rm rechapter.sh rechapter.sed
# 7. git commit -a

import re, sys
from pathlib import Path

CHAPTER=re.compile(r"([0-9]+)-([^/]*)")
p = Path('.')
chapters = dict()
for c in p.iterdir():
    if not c.is_dir():
        continue
    parts = CHAPTER.fullmatch(str(c))
    if not parts:
        continue
    chapters[parts[2]] = (c, parts[1])
nch = len(chapters)
ch_digits = len(str(nch))

def next_line(lines):
    try:
        return next(lines)
    except StopIteration:
        return None

CH_HEAD = re.compile(r"- \[([^]]*)]\(([0-9]+)-([^/]+)/README\.md\)")
CH_TOPIC = re.compile(r"( +)- \[([^]]*)]\(([0-9]+)-([^/]+)/(.*\.md)\)")
chs = []
summary_file = Path("SUMMARY.md")
new_summary_file = Path("SUMMARY.md.new")
renames = []
with summary_file.open(mode="r") as s:
    with new_summary_file.open(mode="w") as ns:
        summary = s.read()
        ch = 1
        lines = iter(summary.splitlines())
        line = next_line(lines)
        while line is not None:
            parts = CH_HEAD.fullmatch(line)
            if parts:
                name = parts[3]
                old = parts[2]
                new = f"{ch:0{ch_digits}}"
                if old != new:
                    renames.append((name, old, new))
                line = f"- [{parts[1]}]({new}-{parts[3]}/README.md)"
                print(line, file=ns)
                line = next_line(lines)
                while line is not None:
                    subparts = CH_TOPIC.fullmatch(line)
                    if not subparts:
                        break
                    old_sub = subparts[3]
                    if old_sub != old:
                        print(
                            f"chapter mismatch: old chapter {old}, old subchapter {old_sub}",
                            file=sys.stderr,
                        )
                    line = f"{subparts[1]}- [{subparts[2]}]({new}-{subparts[4]}/{subparts[5]})"
                    print(line, file=ns)
                    line = next_line(lines)
                ch += 1
                continue
            print(line, file=ns)
            line = next_line(lines)
        if line is not None:
            print(line, file=ns)
        if ch - 1 != nch:
            print("chapter count mismatch: summary {ch - 1}, dir {nch}", file=sys.stderr)
            exit(1)

sed_script = Path("rechapter.sed")
with sed_script.open(mode="w") as s:
    for name, old, new in renames:
        print(f's={old}-{name}={new}-{name}=g', file=s)

shell_script = Path("rechapter.sh")
with shell_script.open(mode="w") as s:
    for name, old, new in renames:
        print(f'git mv "{old}-{name}" "{new}-{name}"', file=s)
    print('find . -type f -name "*.md" -print |', file=s)
    print('while read i; do sed -i -f rechapter.sed "$i"; done', file=s)
