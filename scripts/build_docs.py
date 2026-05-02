#!/usr/bin/env python3
"""Génère docs/ depuis les sources cours/ et annexes/ pour MkDocs."""

import shutil
from pathlib import Path

ROOT = Path(__file__).parent.parent
DOCS = ROOT / "docs"

def copier(src: Path, dst: Path) -> None:
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)

# Nettoyer docs/
if DOCS.exists():
    shutil.rmtree(DOCS)
DOCS.mkdir()

# index.md depuis README.md
copier(ROOT / "README.md", DOCS / "index.md")

# Chapitres
for readme in sorted((ROOT / "cours").glob("*/README.md")):
    nom = readme.parent.name  # ex: 01_ecosysteme
    copier(readme, DOCS / "cours" / f"{nom}.md")

# Annexes
for md in sorted((ROOT / "annexes").glob("*.md")):
    copier(md, DOCS / "annexes" / md.name)

print(f"docs/ généré : {sum(1 for _ in DOCS.rglob('*.md'))} fichiers")
