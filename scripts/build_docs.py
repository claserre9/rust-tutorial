#!/usr/bin/env python3
"""Génère docs/ depuis les sources cours/ et annexes/ pour MkDocs.

Réécrit aussi les liens relatifs pour qu'ils pointent vers les chemins
générés (ex: ../02_fondations/README.md → 02_fondations.md).
"""

import re
import shutil
from pathlib import Path

ROOT = Path(__file__).parent.parent
DOCS = ROOT / "docs"


def réécrire_liens(contenu: str, contexte: str) -> str:
    """Réécrit les liens relatifs vers les chemins MkDocs générés.

    contexte : "index", "cours" ou "annexes"
    """

    def remplacer(m: re.Match) -> str:
        lien = m.group(1)

        # Liens vers les chapitres : ../XX_nom/README.md  ou  cours/XX_nom/README.md
        ch = re.match(r"(?:\.\.\/|cours\/)(\d+_[^/]+)\/README\.md", lien)
        if ch:
            nom = ch.group(1)
            if contexte == "cours":
                return f"({nom}.md)"
            else:  # index ou annexes
                return f"(cours/{nom}.md)"

        # Liens vers les annexes depuis les chapitres : ../../annexes/X.md
        ann = re.match(r"\.\.\/\.\.\/annexes\/(.*\.md)", lien)
        if ann:
            return f"(../annexes/{ann.group(1)})"

        # Liens vers les annexes depuis l'index : annexes/X.md
        ann2 = re.match(r"annexes\/(.*\.md)", lien)
        if ann2 and contexte == "index":
            return f"(annexes/{ann2.group(1)})"

        return m.group(0)  # lien non reconnu → inchangé

    return re.sub(r"\(([^)]+\.md)\)", remplacer, contenu)


def copier_et_réécrire(src: Path, dst: Path, contexte: str) -> None:
    dst.parent.mkdir(parents=True, exist_ok=True)
    contenu = src.read_text(encoding="utf-8")
    contenu = réécrire_liens(contenu, contexte)
    dst.write_text(contenu, encoding="utf-8")


# Nettoyer docs/
if DOCS.exists():
    shutil.rmtree(DOCS)
DOCS.mkdir()

# index.md depuis README.md
copier_et_réécrire(ROOT / "README.md", DOCS / "index.md", "index")

# Chapitres
for readme in sorted((ROOT / "cours").glob("*/README.md")):
    nom = readme.parent.name  # ex: 01_ecosysteme
    copier_et_réécrire(readme, DOCS / "cours" / f"{nom}.md", "cours")

# Annexes
for md in sorted((ROOT / "annexes").glob("*.md")):
    copier_et_réécrire(md, DOCS / "annexes" / md.name, "annexes")

print(f"docs/ généré : {sum(1 for _ in DOCS.rglob('*.md'))} fichiers")
