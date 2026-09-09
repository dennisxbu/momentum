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

Voraussetzung: K1 ausgewählt und Ergebnisse aus K2 berücksichtigt.

Ziel: zwei bis drei wirklich unterschiedliche Arten der Zusammenarbeit zeigen. Nicht erneut dieselben Karten anders sortieren.

Jedes Konzept zeigt:

- vorbereiteten Morgen;
- gezielte Rückfrage und Neubewertung;
- relevante Tagesänderung;
- abendliche Synthese;
- Entwicklung über Wochen;
- nicht vorgebautes persönliches Thema;
- Wiedereinstieg nach einer Pause.

Bewertet werden Arbeitsteilung, Entscheidungsqualität, Verständlichkeit, Freiheit, Pflegeaufwand und Freude an der Nutzung. Die Darstellung darf ein wegwerfbarer Entwurf sein und wählt keinen produktiven Stack.

Ergebnis: Auswahl oder konkrete Korrektur durch Dennis.

## K4 — Technische Grundlage entscheiden

Voraussetzung: K1–K3 abgeschlossen und ein Konzept ausgewählt.

Die Architektur muss den ausgewählten Freiheits-Nachweis tragen, ohne ein unbegrenztes No-Code-System zu bauen. Sie trennt mindestens:

- flexible, versionierbare persönliche Strukturen und Daten;
- verlässliche Berechnungen und Auswertungen;
- Assistenzabläufe und Befugnisse;
- Oberfläche und konkrete Nutzung;
- Speicherung, Export und Wiederherstellung;
- optionale externe Modell- oder Datenanbindungen.

Zu entscheiden: Windows-Technik, Speicherort, Offline-Anforderungen, Schutz persönlicher Daten, Erweiterungsgrenzen und Wechselkosten. Noch keine Installation und kein App-Code.

## K5 — Vertikalen Kern statt Tracker-Unterbau bauen

Die Implementierungsreihenfolge wird nach K4 konkretisiert. Sie muss möglichst früh einen kleinen vollständigen Nutzenfluss liefern:

1. startbares Gerüst;
2. kleiner flexibler Datenkern mit Speicherung;
3. nicht vorgebautes Thema als Freiheits-Nachweis;
4. vollständiger Assistenzfluss von Morgen über Korrektur bis Abend/Folgetag;
5. erst dann breitere Fachfunktionen, Auswertungen und visuelle Verfeinerung innerhalb des gewählten V1-Umfangs.

Eine große Aufgaben-/Routinenverwaltung darf nicht vorgezogen werden, nur weil sie technisch leichter ist.

## K6 — Alltag, Auslieferung und Abnahme

Wie bisher getrennt prüfen:

- technische Funktion;
- Assistenzqualität mit zurückgehaltenen Varianten;
- Bediengefühl und persönlicher Nutzen durch Dennis;
- Datenbeständigkeit, Export und echte Wiederherstellung;
- Windows-Paket und Verhalten ohne optionale Dienste;
- Kosten und Datenverwendung, falls externe Dienste gewählt wurden.

„Fertig“ gilt erst nach dem vereinbarten Nutzungstest und der dokumentierten Abnahme. Danach keine automatische Version 2.

## Aktuelle Grenze

K0, K1 und K2 sind abgeschlossen. **K3 ist der nächste mögliche Auftrag.** K4 und Implementierung sind nicht automatisch freigegeben.
