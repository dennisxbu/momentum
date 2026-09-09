# Entwicklung mit Git und GitHub

Momentum wird als Git-Repository auf GitHub geführt. Das Remote ist die gemeinsame, dauerhafte Ablage für Code, Regeln und Kontext. Ein Chatverlauf ersetzt das nicht.

Remote: `https://github.com/dennisxbu/momentum.git`  
Standardbranch: `main`

Die verbindliche technische Grundlage für die spätere Implementierung steht in [architektur-k4.md](architektur-k4.md). Dieses Dokument regelt Git und Projektgedächtnis, nicht die Laufzeitarchitektur.

Der ausführbare Stand und seine lokale Startanleitung stehen in [k5-implementierung.md](k5-implementierung.md). Für den Build der gebündelten SQLCipher-Kryptografie ist auf Windows zusätzlich Strawberry Perl erforderlich; die fertige App benötigt es nicht.

## Was ins Repository gehört

- Quellcode und Projektdokumentation, sobald ein Schritt sie erzeugt
- Regeln (`AGENTS.md`, `.cursor/rules/`)
- Status, Entscheidungen, Umfang, Abnahme
- Synthetische Beispieldaten, klar als Demo gekennzeichnet
- Ursprüngliche Kontextdokumente unter `docs/context/`

## Was nicht ins Repository gehört

- API-Schlüssel, Tokens, `.env`-Dateien mit Geheimnissen
- Echte persönliche Nutzdaten, Exporte aus dem späteren Alltag, Gesundheits- oder Kontodaten
- Build-Artefakte, lokale Caches, der verschachtelte Ordner `momentum/` (Clone im Clone)

GitHub als Codeablage war selbst kein Beschluss über den Speicherort der App-Daten (D4). D26 wählt inzwischen den lokalen Anwendungsdatenordner des aktuellen Windows-Benutzers; persönliche App-Daten bleiben weiterhin außerhalb des Repositorys.

## Ablauf nach einem beauftragten Schritt

1. Nur die zum Schritt gehörenden Dateien ändern.
2. `docs/status.md` und bei Bedarf `docs/entscheidungen.md` aktualisieren.
3. `git status` und `git diff` prüfen. Keine Geheimnisse stagen.
4. Commit mit kurzer Begründung (warum dieser Schritt, nicht eine Dateiliste).
5. Nach `origin/main` pushen, sofern Dennis nicht ausdrücklich etwas anderes sagt.

Nicht committen oder pushen, wenn Dennis das für den jeweiligen Stand ausdrücklich untersagt. Kein Force-Push auf `main`. Git-Hooks nicht mit `--no-verify` umgehen.

## Commits

- Ein abgeschlossener Schritt soll nachvollziehbar in der Historie liegen.
- Keine leeren Commits. Keine Secrets. Keine nachträgliche Umfangserweiterung „weil der Commit sonst zu klein wirkt“.
- `git config` nicht ändern.

## Neuer Chat

Zuerst README, AGENTS.md, `docs/produktvision.md`, Status, Entscheidungen, `docs/arbeitsplan.md`, den ausgewählten V1-Nachweis und den K2-Machbarkeitscheck lesen; danach bei Bedarf die historische Arbeitsnotiz. Vorhandenen Code und ungesicherte Änderungen prüfen, bevor etwas geplant wird. Dokumente beschreiben Absichten; vorhandene Funktionen nur behaupten, wenn sie im Projekt nachvollziehbar sind. Die ursprüngliche Prompt-Sammlung unter `docs/context/` darf seit der Grundkorrektur vom 9.9.2026 nicht mechanisch fortgesetzt werden.
