# Momentum

Persönlicher, flexibel anpassbarer Windows-Assistent für den privaten Alltag von Dennis. Der erste ausführbare vertikale Kern verbindet ein vorbereitetes Briefing mit einer freien persönlichen Datengrundlage, statt eine weitere Karten- und Checkbox-Verwaltung zu sein. Er ist ein geprüfter Entwicklungsstand, noch keine abgenommene fertige App.

Code und Projektgedächtnis: [github.com/dennisxbu/momentum](https://github.com/dennisxbu/momentum). Das legt nicht fest, wo spätere persönliche App-Daten gespeichert werden.

## Lesen

- [AGENTS.md](AGENTS.md) — Arbeitsweise für Mensch und Agent
- [docs/status.md](docs/status.md) — aktueller Schritt und Grenzen
- [docs/produktvision.md](docs/produktvision.md) — aktuelle Nordrichtung: Assistenz, Freiheit und fokussierte Nutzung
- [docs/arbeitsplan.md](docs/arbeitsplan.md) — korrigierte Reihenfolge; Assistenzrisiko vor Architektur und Tracker-Ausbau prüfen
- [docs/nutzungskonzept.md](docs/nutzungskonzept.md) — Alltagsszenarien, korrigiert nach dem verworfenen Entwurf
- [docs/umfang-v1.md](docs/umfang-v1.md) — ausgewählter vertikaler Nachweis für Version 1
- [docs/machbarkeitscheck-k2.md](docs/machbarkeitscheck-k2.md) — geprüfte Machbarkeitsgrenze und Mindestkontext
- [docs/bedienkonzepte.md](docs/bedienkonzepte.md) — abgeschlossener K3-Vergleich und ausgewählte Briefing-Leitform
- [docs/architektur-k4.md](docs/architektur-k4.md) — ausgewählte technische Grundlage, Daten- und Assistenzgrenzen
- [docs/k5-implementierung.md](docs/k5-implementierung.md) — ausführbarer Kern, Prüfungen und Startanleitung
- [docs/entscheidungen.md](docs/entscheidungen.md) — Vorschlag vs. Beschluss
- [docs/entwicklung.md](docs/entwicklung.md) — Git/GitHub
- [docs/context/](docs/context/) — Arbeitsnotiz und Schritt-Prompts

## Mitwirken

Jeweils nur den ausdrücklich beauftragten Schritt aus [docs/arbeitsplan.md](docs/arbeitsplan.md) bearbeiten. Die ursprüngliche [Prompt-Sammlung](docs/context/Cursor_Prompts_App_Entwicklung.md) bleibt historischer Kontext und darf nach der Korrektur nicht mechanisch fortgesetzt werden. In einem neuen Chat zuerst Produktvision, Status und Entscheidungen lesen. Die frühere Empfehlung „A — Tag als Heimat“ ist verworfen.

## Entwicklungsstand ausprobieren

Voraussetzungen sind Node.js, Rust, die Tauri-Windows-Werkzeuge und Strawberry Perl für die gebündelte SQLCipher-Kryptografie. Danach im Projektordner:

```powershell
npm install
npm run tauri -- dev
```

Der vollständige Prüflauf ist mit `npm run build` und anschließend `cargo test --all-targets` im Ordner `src-tauri` möglich. Der erzeugte Installer liegt nach `npm run tauri -- build` unter `src-tauri/target/release/bundle/nsis/` und wird nicht in Git eingecheckt.
