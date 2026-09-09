# Aktueller Arbeitsplan

Stand: 9. September 2026 · ersetzt die automatische Fortsetzung der ursprünglichen Schrittfolge

Die ursprüngliche Prompt-Sammlung unter `docs/context/` bleibt historischer und methodischer Kontext. Nach der grundlegenden Korrektur vom 9. September 2026 darf sie nicht mechanisch ab Schritt 3 fortgesetzt werden: Sie würde zu viel Tagesplaner, Dateneingabe und UI bauen, bevor der zentrale Assistenz- und Freiheitsanspruch bewiesen ist.

Jeder Abschnitt wird nur nach einem ausdrücklichen Auftrag bearbeitet. Der folgende Abschnitt beginnt nie automatisch.

## K0 — Grundrichtung korrigieren

Status: **abgeschlossen**

Ergebnis:

- [produktvision.md](produktvision.md) hält Assistenz, Notion-artige Freiheit und fokussierte Nutzung als Nordrichtung fest.
- [umfang-v1.md](umfang-v1.md) verlangt Assistenz- und Freiheits-Nachweis statt eines Karten-/Tracker-MVP.
- [bedienkonzepte.md](bedienkonzepte.md) verwirft A/B/C und definiert den neuen Konzeptauftrag.
- Status, Entscheidungen und Arbeitsregeln sind angepasst.

Nicht entstanden: App, Architektur, neue Screens oder KI-Entscheidung.

## K1 — Den kleinsten ehrlichen V1-Nachweis auswählen

Status: **abgeschlossen**

Ausgewählt ist ein schmaler, vollständiger Assistenzkreislauf auf einer begrenzt flexiblen Grundlage:

- Dennis kann eine nicht vorgebaute Sammlung mit eigenen Eigenschaften und einer Beziehung anlegen.
- Der verbindliche synthetische Prüffall „Stuhl-Kandidaten“ bringt eine eigene Frist und ein Budget in den bekannten Donnerstag ein.
- Der Assistent verdichtet die Lage, fragt nach einer entscheidenden Lücke, begründet Empfehlung und Alternative, reagiert stabil auf Änderungen und verwendet den Verlauf am Abend sowie Folgetag weiter.
- Eigene Datenänderungen brauchen eine konkrete Bestätigung. Analyse und Vorschläge darf die App selbst vorbereiten; externe Aktionen sind ausgeschlossen.
- Der Nachweis enthält dauerhafte Speicherung sowie echten Export mit Wiederherstellung.

Die vollständige Auswahl und ihre beobachtbaren Abnahmekriterien stehen in [umfang-v1.md](umfang-v1.md). Keine Technik und keine UI wurden gewählt.

## K2 — Früher Machbarkeitscheck für Assistenz und Offenheit

Status: **abgeschlossen**

Ergebnis des [Machbarkeitschecks](machbarkeitscheck-k2.md):

- Zehn getrennte synthetische Varianten prüfen relevante und belanglose Änderungen, eine entscheidende Lücke, freie Strukturen, Überlastung, Korrektur und unbekannte Durchführung.
- Der V1-Kern ist unter klaren Bedingungen konzeptionell machbar.
- Eine freie Eigenschaft wird erst planungsrelevant, wenn Bedeutung, Beziehung und persönliche Absicht bestätigt sind. Ein beliebiges Datum erzeugt keine Verpflichtung.
- Zeitfenster, Einheiten, Zustände, Herkunft und Befugnisse brauchen eine transparente Prüfung. Kontextuelle Beurteilung darf darauf aufsetzen, aber nicht direkt schreiben.
- Die fünf in K1 gewählten Eigenschaftsformen reichen für den Nachweis; zusätzliche Formen sind durch K2 nicht begründet.
- Eine rein transparente Lösung bleibt für das gewünschte Assistenzgefühl zu flach, eine rein generative Lösung ist als Wahrheits- und Schreibinstanz zu unsicher. Für K3 ist eine begrenzte Kombination die tragfähigste Annahme, ohne damit Anbieter oder KI festzulegen.

Keine App, produktive Integration oder kostenpflichtige Modellnutzung ist entstanden.

## K3 — Neue Assistenzkonzepte vergleichen

Status: **abgeschlossen**

In [bedienkonzepte.md](bedienkonzepte.md) sind drei unterschiedliche Grundmodi mit identischen K1-/K2-Fällen ausgearbeitet:

- **1 — Briefing:** Momentum eröffnet vorbereitet mit Einordnung, Empfehlung und genau dem relevanten Entscheidungsmoment.
- **2 — Studio:** Dennis arbeitet in einem verbundenen persönlichen Denkraum; Momentum schlägt Beziehungen, Bedeutungen und Sichten vor.
- **3 — Delegat:** Momentum bereitet im Hintergrund gebündelte Änderungssätze vor, die Dennis gezielt bestätigt oder korrigiert.

Die Empfehlung der Software-Expertise lautet: **Briefing als Alltag, Änderungsvertrag aus 3 für Vertrauen, Studio aus 2 nur auf Nachfrage für Freiheit und Tiefe.**

Dennis' Korrektur ergänzt zwei verbindliche Bedingungen: Die Zusammenarbeit folgt immer der verständlichen Folge **bekannt — Bedeutung — Vorschlag — Wirkung**. Smarter Kalender, Diätanalyse und spätere Fachfähigkeiten sind optionale, anpassbare Werkzeuge auf der gemeinsamen Grundlage, keine vorgeschriebenen Lebensbereiche oder V1-Zusagen.

Alle Konzepte zeigen Morgen, Rückfrage, Tagesänderung, Abend, Entwicklung, freies Thema und Wiedereinstieg. Zusätzlich werden die K2-Kontraste harte Rückgabe, belanglose Stuhlfarbe und zurückgehaltene Rezeptideen verwendet. Kein produktiver Stack und keine endgültige Oberfläche sind gewählt.

Dennis hat die überarbeitete Empfehlung am 9. September 2026 ausgewählt. D24 legt Konzept 1 „Briefing“ als primären Alltag fest, ergänzt um die verbindliche Bediengrammatik, den begrenzten Änderungsvertrag sowie Studio und optionale Werkzeuge auf Nachfrage.

## K4 — Technische Grundlage entscheiden

Status: **abgeschlossen**

Die ausgewählte [K4-Architektur](architektur-k4.md) trägt den Freiheits-Nachweis, ohne ein unbegrenztes No-Code-System zu bauen. Sie trennt:

- flexible, versionierbare persönliche Strukturen und Daten;
- verlässliche Berechnungen und Auswertungen;
- Assistenzabläufe und Befugnisse;
- Oberfläche und konkrete Nutzung;
- Speicherung, Export und Wiederherstellung;
- optionale externe Modell- oder Datenanbindungen.

Entschieden sind Tauri 2 mit React/TypeScript und Rust-Kern, lokal verschlüsseltes SQLite mit DPAPI-Schlüssel, vollständiger Offline-Kern, schmale Tauri-Commands, providerneutrale Beurteilung, interne optionale Werkzeuge, versionierter Export und NSIS-Verteilung pro Benutzer. Ein KI-Anbieter sowie echte externe Konten bleiben getrennte spätere Entscheidungen. In K4 entstanden keine Installation und kein App-Code.

## K5 — Vertikalen Kern statt Tracker-Unterbau bauen

Status: **abgeschlossen**

Die Implementierungsreihenfolge wird nach K4 konkretisiert. Sie muss möglichst früh einen kleinen vollständigen Nutzenfluss liefern:

1. startbares Gerüst;
2. kleiner flexibler Datenkern mit Speicherung;
3. nicht vorgebautes Thema als Freiheits-Nachweis;
4. vollständiger Assistenzfluss von Morgen über Korrektur bis Abend/Folgetag;
5. erst dann breitere Fachfunktionen, Auswertungen und visuelle Verfeinerung innerhalb des gewählten V1-Umfangs.

Eine große Aufgaben-/Routinenverwaltung darf nicht vorgezogen werden, nur weil sie technisch leichter ist.

Ergebnis:

- startbare Tauri-Windows-App mit React-Oberfläche und Rust-Anwendungskern;
- verschlüsselter lokaler SQLCipher-Speicher mit einem durch Windows DPAPI geschützten Zufallsschlüssel;
- freies Studio mit den fünf V1-Eigenschaftsformen, Beziehungen, typisierter Prüfung und versionierter Umbenennung;
- vollständiger synthetischer Ablauf von Morgen über Bestätigung und relevante Tagesänderung bis Abend und Folgetag;
- sichtbare Folge „bekannt — Bedeutung — Vorschlag — Wirkung“ und begrenzter Änderungsvertrag vor Planänderungen;
- vollständiger lesbarer JSON-Export mit validierter Wiederherstellung;
- bestandene automatisierte Kernprüfungen, echter App-Start und erzeugter NSIS-Installer.

Details und reproduzierbare Prüfungen stehen in [k5-implementierung.md](k5-implementierung.md). Die persönliche Bedien- und Nutzenabnahme ist nicht vorweggenommen.

## K6 — Alltag, Auslieferung und Abnahme

Status: **technischer Teil am 9. September 2026 durchgeführt; persönliche Abnahme offen.** Details stehen in [k6-abnahme.md](k6-abnahme.md).

Wie bisher getrennt prüfen:

- technische Funktion;
- Assistenzqualität mit zurückgehaltenen Varianten;
- Bediengefühl und persönlicher Nutzen durch Dennis;
- Datenbeständigkeit, Export und echte Wiederherstellung;
- Windows-Paket und Verhalten ohne optionale Dienste;
- Kosten und Datenverwendung, falls externe Dienste gewählt wurden.

„Fertig“ gilt erst nach dem vereinbarten Nutzungstest und der dokumentierten Abnahme. Danach keine automatische Version 2.

## Aktuelle Grenze

K0 bis K5 sind abgeschlossen. K6 ist ausdrücklich beauftragt und technisch durchgeführt, aber erst nach Dennis' persönlichem Nutzungstest abgeschlossen. Bis zu seinem Urteil beginnt kein weiterer Abschnitt und es gilt weiterhin keine fertige Version 1 als abgenommen.
