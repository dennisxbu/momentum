# Projektstatus

Stand: 9. September 2026

## Schritt und Dokumentationsstand

| Ebene | Stand |
| --- | --- |
| Aktueller Schritt | **K4 — technische Grundlage entscheiden: abgeschlossen.** |
| Auftrag | Dennis hat K4 am 9. September 2026 ausdrücklich beauftragt. |
| Konzepte | 1 „Briefing“, 2 „Studio“ und 3 „Delegat“ unterscheiden sich in Eröffnung, Arbeitsteilung, Korrektur und sichtbarer Freiheit. |
| Empfehlung | Verständliches Briefing als Alltag, Änderungsvertrag aus 3 für Vertrauen, Studio aus 2 und optionale Werkzeuge auf Nachfrage für Freiheit und Tiefe. |
| Produktentscheidung | D24: Konzept 1 „Briefing“ mit verständlicher Bediengrammatik, Änderungsvertrag sowie Studio und optionalen Werkzeugen auf Nachfrage ist ausgewählt. |
| Technikentscheidung | D25–D30: Tauri/React/TypeScript mit Rust-Kern, lokales verschlüsseltes SQLite, schmale Commands, providerneutrale Assistenz und interne optionale Werkzeuge. |

Die bisherige Empfehlung „A — Tag als Heimat“ bleibt zurückgezogen. V1-Nachweis, Bedienkonzept und Architektur wurden in D18, D24 und D25–D30 jeweils getrennt entschieden.

## Tatsächlich vorhandene Funktionen

Keine App. Kein Framework, keine produktive UI, keine Datenbank, keine KI-Integration, keine Installation.

Vorhanden sind:

- Projektdokumentation und GitHub als Projektgedächtnis;
- historische Kontextquellen;
- verworfene Konzeptentwürfe ausschließlich in der Git-/PR-Historie;
- ausgewählter und konzeptionell geprüfter V1-Nachweis;
- dokumentierter K2-Trockenlauf, aber noch kein ausführbarer Assistenztest;
- ausgearbeiteter K3-Konzeptvergleich mit interaktivem, wegwerfbarem Entwurf;
- verbindliche Bediengrammatik „bekannt — Bedeutung — Vorschlag — Wirkung“ und Werkzeugmodell als K3-Korrektur;
- ausgewählte konzeptionelle Bedienrichtung, aber noch kein realer Benutzbarkeitstest;
- abgeschlossene [K4-Architekturentscheidung](architektur-k4.md), aber noch kein erzeugtes Projektgerüst.

## Aktuelle Quellenreihenfolge

1. aktuelle direkte Aussagen von Dennis;
2. [produktvision.md](produktvision.md) und [entscheidungen.md](entscheidungen.md);
3. dieser Status und der ausgewählte [V1-Nachweis](umfang-v1.md);
4. der abgeschlossene [Machbarkeitscheck K2](machbarkeitscheck-k2.md);
5. [nutzungskonzept.md](nutzungskonzept.md);
6. historische Arbeitsnotiz und Prompt-Sammlung unter `docs/context/`.

Historische Beispiele oder frühere Vorschläge dürfen eine neuere Korrektur nicht überschreiben.

## Offene Grenzen

- **O1:** Produktnachweis, Bedienform und technische Grundlage sind ausgewählt; ausführbare Qualität und Umsetzung sind noch nicht belegt.
- **Bedienkonzept geklärt:** D24 wählt das verständliche Briefing als primären Alltag. Die früheren A/B/C aus PR #5 bleiben verworfen; reale Benutzbarkeit wird erst an einer ausführbaren Umsetzung geprüft.
- **Technik geklärt:** D25–D30 wählen Stack, lokalen verschlüsselten Speicher, Offline-Grundsatz, Schichtgrenzen, Export und erste Windows-Verteilung.
- **O4:** KI-Einsatz, Anbieter oder lokaler Betrieb, erlaubte Daten und Budget unter 50 Euro bleiben vor einer produktiven Anbindung offen. Die Architektur bindet sich an keinen Anbieter.
- **O5:** Für V1 sind Analyse ohne Bestätigung, Datenänderung nach konkreter Bestätigung und keine externen Aktionen gewählt. Spätere Befugnisse bleiben offen.
- **O6/O11:** Die V1-Tiefe der Freiheit ist gewählt. K2 verlangt bestätigte Semantik, aber keine weitere Eigenschaftsform; langfristige Tiefe bleibt offen.
- **O7–O10:** Diätanalyse und smarter Kalender sind als optionale Werkzeugideen festgehalten; genaue Tiefe, Daten, Integrationen und Zeitpunkt sowie Abendrhythmus und berufliche Entwicklung bleiben fachlich offen.

GitHub ist Codeablage. D26 legt den lokalen Anwendungsdatenordner des aktuellen Windows-Benutzers als Speicherort fest; echte persönliche Daten gehören weiterhin niemals ins Repository.

## Umfang der ersten Version

Der frühere Vorschlag V9/V10 bleibt als alleinige Grundlage **zurückgezogen**. Der ausgewählte [V1-Nachweis](umfang-v1.md) verbindet zwei Leistungen:

1. Die App übernimmt in einem vollständigen Tagesablauf sichtbar Denk-, Ordnungs- und Auswertungsarbeit.
2. Ein vorher nicht fest eingebautes persönliches Thema lässt sich als eigene Sammlung mit fünf Eigenschaftsformen, einer Beziehung und einer passenden Sicht ohne Code abbilden und vom Assistenten verwenden.

Als synthetischer Prüffall dient „Arbeitsplatz verbessern“ mit „Stuhl-Kandidaten“. K2 ergänzt als zurückgehaltenen Fall „Rezeptideen“ und zeigt: Die freien Daten funktionieren im Assistenzkontext nur mit bestätigter Bedeutung und Beziehung. Eine vollständige Notion-Kopie, Plugin-Plattform oder automatische Lebenssteuerung ist dadurch nicht beauftragt.

## Nächste inhaltliche Arbeit

K4 ist mit [architektur-k4.md](architektur-k4.md) und D25–D30 abgeschlossen. Der nächste mögliche Abschnitt ist K5: den kleinsten vollständigen vertikalen Kern auf dieser Grundlage bauen. K5 beginnt erst nach einem ausdrücklichen Auftrag.

## Abnahmeprotokoll

| Schritt | Was geprüft wurde | Ergebnis |
| --- | --- | --- |
| Initialprompt | Kontextdateien, Git-Remote, keine App-Dateien | Dokumentationsarbeit erfüllt |
| Schritt 1 | Nutzungskonzept und Vision-Abgleich | historisch dokumentiert; Tagesbeispiel bleibt, ist aber kein UI-Bauplan |
| Schritt 2 | V1-Umfang und spätere Korrektur | frühere Fassung reicht nach neuer Klarstellung nicht als Produktgrundlage |
| Schritt 3, erster Versuch | A/B/C und HTML-Demo | von Dennis als falsche Grundrichtung abgelehnt |
| Aktuelle Korrektur | vollständiger Chat, Repo-Dokumente und vier Screenshots gegen die ursprüngliche Vision geprüft | falsche Annahme identifiziert; Projektgedächtnis neu ausgerichtet |
| K1 | V1-Nachweis, freie Struktur, Assistenzfluss, Datenumfang, Befugnisse und beobachtbare Abnahme | schmaler vertikaler Kern ausgewählt; keine Technik oder UI vorweggenommen |
| K2 | zehn synthetische Kontrastfälle, drei Lösungsrichtungen, Mindestkontext und Pflegeaufwand | konzeptionell bestanden mit Bedingungen; reale Modell- und Bedienqualität ausdrücklich noch nicht geprüft |
| K3, Vergleich | drei Arten der Zusammenarbeit gegen dieselben K1-/K2-Fälle ausgearbeitet | fachliche Empfehlung V17 vorhanden |
| K3, Korrektur | intuitive Nachvollziehbarkeit und Kalender/Diätanalyse als optionale, anpassbare Werkzeuge eingeordnet | Empfehlung geschärft; keine Fachfunktion in V1 aufgenommen |
| K3, Auswahl | überarbeitete Leitform von Dennis bestätigt | D24 beschlossen; K3 abgeschlossen |
| K4 | Desktop-Stack, Datenkern, Vertrauensgrenzen, Offline, Schutz, Erweiterung, Export und Verteilung gegen K1–K3 geprüft | D25–D30 beschlossen; auf dem Entwicklungsrechner sind Tauri-Voraussetzungen vorhanden; keine App erzeugt |

Noch keine App-Abnahme. Bedienkonzept und Architektur sind ausgewählt, aber noch nicht ausführbar auf Funktion, Assistenzqualität oder Benutzbarkeit geprüft.

## GitHub

Remote `origin`: `https://github.com/dennisxbu/momentum.git`. PR #5 wird als durch die aktuelle Korrektur überholt behandelt; seine Entwürfe dürfen nicht später unbeabsichtigt als Produktentscheidung zurückkehren.
