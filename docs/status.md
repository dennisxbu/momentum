# Projektstatus

Stand: 9. September 2026

## Schritt und Dokumentationsstand

| Ebene | Stand |
| --- | --- |
| Abgeschlossener Schritt | **K1 — kleinsten ehrlichen V1-Nachweis auswählen.** Keine App-Implementierung. |
| Auftrag | Dennis delegiert die Wahl des fachlich nächsten Schritts an die Software-Expertise; K1 war laut Arbeitsplan als Nächstes erforderlich. |
| Ausgewählter Kern | Vollständiger Assistenzkreislauf von vorbereiteter Lage über Korrektur und Tagesänderung bis Abend/Folgetag, verbunden mit einer selbst angelegten Sammlung. |
| Freiheitsprüfung | Synthetische Sammlung „Stuhl-Kandidaten“ mit eigenen Eigenschaften, Beziehung, Sicht und tatsächlichem Einfluss auf den Tageskontext. |
| Produktentscheidungen | D18 und D19 wählen V1-Nachweis, Anfangskontext und vorsichtige Befugnisse. Bedienung, Technik und KI bleiben offen. |

Die bisherige Empfehlung „A — Tag als Heimat“ bleibt zurückgezogen. Der ausgewählte V1-Nachweis ist ein Produktumfang, noch kein Bedienkonzept und keine Architektur.

## Tatsächlich vorhandene Funktionen

Keine App. Kein Framework, keine produktive UI, keine Datenbank, keine KI-Integration, keine Installation.

Vorhanden sind:

- Projektdokumentation und GitHub als Projektgedächtnis;
- historische Kontextquellen;
- verworfene Konzeptentwürfe ausschließlich in der Git-/PR-Historie;
- ausgewählter, aber noch nicht auf Machbarkeit geprüfter V1-Nachweis;
- noch keine validierte Bedienrichtung.

## Aktuelle Quellenreihenfolge

1. aktuelle direkte Aussagen von Dennis;
2. [produktvision.md](produktvision.md) und [entscheidungen.md](entscheidungen.md);
3. dieser Status und der ausgewählte [V1-Nachweis](umfang-v1.md);
4. [nutzungskonzept.md](nutzungskonzept.md);
5. historische Arbeitsnotiz und Prompt-Sammlung unter `docs/context/`.

Historische Beispiele oder frühere Vorschläge dürfen eine neuere Korrektur nicht überschreiben.

## Offene Grenzen

- **O1:** Der V1-Nachweis ist ausgewählt; seine Machbarkeit und spätere Umsetzung sind noch nicht belegt.
- **O2:** Kein Bedienkonzept ist gewählt. A/B/C aus PR #5 sind verworfen.
- **O3:** Technik-Stack, Offline-Verhalten und Speicherort sind offen.
- **O4:** KI, Anbieter oder lokaler Betrieb, erlaubte Daten und Budget unter 50 Euro sind offen.
- **O5:** Für V1 sind Analyse ohne Bestätigung, Datenänderung nach konkreter Bestätigung und keine externen Aktionen gewählt. Spätere Befugnisse bleiben offen.
- **O6/O11:** Die V1-Tiefe der Freiheit ist gewählt; eine weitergehende langfristige Tiefe bleibt offen.
- **O7–O10:** Diät/Training, Abendrhythmus, Terminquellen und berufliche Entwicklung bleiben fachlich offen.

GitHub ist Codeablage. Speicherort persönlicher App-Daten bleibt offen.

## Umfang der ersten Version

Der frühere Vorschlag V9/V10 bleibt als alleinige Grundlage **zurückgezogen**. Der ausgewählte [V1-Nachweis](umfang-v1.md) verbindet zwei Leistungen:

1. Die App übernimmt in einem vollständigen Tagesablauf sichtbar Denk-, Ordnungs- und Auswertungsarbeit.
2. Ein vorher nicht fest eingebautes persönliches Thema lässt sich als eigene Sammlung mit fünf Eigenschaftsformen, einer Beziehung und einer passenden Sicht ohne Code abbilden und vom Assistenten verwenden.

Als synthetischer Prüffall dient „Arbeitsplatz verbessern“ mit „Stuhl-Kandidaten“. Eine vollständige Notion-Kopie, Plugin-Plattform oder automatische Lebenssteuerung ist dadurch nicht beauftragt.

## Nächste inhaltliche Arbeit

Vor Architektur oder App-Code folgt als nächster möglicher Auftrag **K2 — der frühe Machbarkeitscheck**. Er prüft mit getrennten synthetischen Fällen Entscheidungsqualität, Stabilität, gezielte Rückfragen, notwendigen Datenbedarf und die Einbeziehung des nicht vorgebauten Themas. Erst seine Ergebnisse begrenzen den Auftrag für neue Assistenzkonzepte in K3.

K2 beginnt nicht automatisch durch den Abschluss von K1.

## Abnahmeprotokoll

| Schritt | Was geprüft wurde | Ergebnis |
| --- | --- | --- |
| Initialprompt | Kontextdateien, Git-Remote, keine App-Dateien | Dokumentationsarbeit erfüllt |
| Schritt 1 | Nutzungskonzept und Vision-Abgleich | historisch dokumentiert; Tagesbeispiel bleibt, ist aber kein UI-Bauplan |
| Schritt 2 | V1-Umfang und spätere Korrektur | frühere Fassung reicht nach neuer Klarstellung nicht als Produktgrundlage |
| Schritt 3, erster Versuch | A/B/C und HTML-Demo | von Dennis als falsche Grundrichtung abgelehnt |
| Aktuelle Korrektur | vollständiger Chat, Repo-Dokumente und vier Screenshots gegen die ursprüngliche Vision geprüft | falsche Annahme identifiziert; Projektgedächtnis neu ausgerichtet |
| K1 | V1-Nachweis, freie Struktur, Assistenzfluss, Datenumfang, Befugnisse und beobachtbare Abnahme | schmaler vertikaler Kern ausgewählt; keine Technik oder UI vorweggenommen |

Noch keine App-Abnahme und kein ausgewähltes Bedienkonzept.

## GitHub

Remote `origin`: `https://github.com/dennisxbu/momentum.git`. PR #5 wird als durch die aktuelle Korrektur überholt behandelt; seine Entwürfe dürfen nicht später unbeabsichtigt als Produktentscheidung zurückkehren.
