# Projektstatus

Stand: 9. September 2026

## Schritt und Dokumentationsstand

| Ebene | Stand |
| --- | --- |
| Beauftragter Schritt | **Korrektur des bisherigen Grundkonzepts und vollständige Neuordnung des Projektgedächtnisses.** Keine App-Implementierung. |
| Auslöser | Dennis lehnt die Entwürfe A/B/C aus PR #5 ab: zu viel Karten-, Status- und Checkbox-Verwaltung; zu wenig erlebbare Assistenz und Freiheit. |
| Neue Leitplanke | Hochgradig hilfreicher persönlicher Assistent; flexible, Notion-artige Grundlage ohne Notion-Kopie; fokussierte und sofort brauchbare Oberfläche. |
| Dokumentationsarbeit | Aktuelle Produktvision, korrigierter V1-Rahmen und neuer Auftrag für Bedienkonzepte dokumentiert. Arbeitsregeln und Prompt-Reihe werden daran ausgerichtet. |
| Produktentscheidungen | D15–D17 halten die direkte Korrektur fest. Die konkrete Umsetzung der Flexibilität, der V1-Umfang und das Bedienkonzept bleiben offen. |

Die bisherige Empfehlung „A — Tag als Heimat“ ist zurückgezogen. Schritt 4 ist nicht beauftragt und darf auf dieser Grundlage nicht beginnen.

## Tatsächlich vorhandene Funktionen

Keine App. Kein Framework, keine produktive UI, keine Datenbank, keine KI-Integration, keine Installation.

Vorhanden sind:

- Projektdokumentation und GitHub als Projektgedächtnis;
- historische Kontextquellen;
- verworfene Konzeptentwürfe ausschließlich in der Git-/PR-Historie;
- noch keine validierte Bedienrichtung.

## Aktuelle Quellenreihenfolge

1. aktuelle direkte Aussagen von Dennis;
2. [produktvision.md](produktvision.md) und [entscheidungen.md](entscheidungen.md);
3. dieser Status und der korrigierte [V1-Rahmen](umfang-v1.md);
4. [nutzungskonzept.md](nutzungskonzept.md);
5. historische Arbeitsnotiz und Prompt-Sammlung unter `docs/context/`.

Historische Beispiele oder frühere Vorschläge dürfen eine neuere Korrektur nicht überschreiben.

## Offene Grenzen

- **O1:** Der korrigierte V1-Rahmen ist ein Vorschlag und noch nicht ausgewählt.
- **O2:** Kein Bedienkonzept ist gewählt. A/B/C aus PR #5 sind verworfen.
- **O3:** Technik-Stack, Offline-Verhalten und Speicherort sind offen.
- **O4:** KI, Anbieter oder lokaler Betrieb, erlaubte Daten und Budget unter 50 Euro sind offen.
- **O5:** Befugnisse des Assistenten für Eintragen, Umplanen und externe Aktionen sind offen.
- **O6/O11:** Form und V1-Tiefe der Notion-artigen Freiheit sind offen.
- **O7–O10:** Diät/Training, Abendrhythmus, Terminquellen und berufliche Entwicklung bleiben fachlich offen.

GitHub ist Codeablage. Speicherort persönlicher App-Daten bleibt offen.

## Umfang der ersten Version

Der frühere Vorschlag V9/V10 ist als alleinige Grundlage **zurückgezogen**. Eine selbst benannte Messgröße mit Einheit beweist die gewünschte Freiheit nicht. Der neue [V1-Rahmen](umfang-v1.md) fordert zwei zusammenhängende Nachweise:

1. Die App übernimmt in einem vollständigen Tagesablauf sichtbar Denk-, Ordnungs- und Auswertungsarbeit.
2. Ein vorher nicht fest eingebautes persönliches Thema lässt sich ohne Code sinnvoll abbilden und in Alltag sowie Auswertung verwenden.

Die genaue Tiefe bleibt auszuwählen. Eine vollständige Notion-Kopie, Plugin-Plattform oder automatische Lebenssteuerung ist dadurch nicht beauftragt.

## Nächste inhaltliche Arbeit

Vor Architektur oder App-Code:

1. den korrigierten V1-Nachweis auswählen oder begrenzt korrigieren;
2. zwei bis drei neue **Assistenzkonzepte** vergleichen, die sich in Arbeitsteilung und Offenheit unterscheiden — nicht nur in Kartenanordnung;
3. den frühen Machbarkeitscheck für Assistenzqualität und notwendigen Datenbedarf ausführen.

Keine der drei Arbeiten beginnt automatisch durch diese Dokumentationskorrektur.

## Abnahmeprotokoll

| Schritt | Was geprüft wurde | Ergebnis |
| --- | --- | --- |
| Initialprompt | Kontextdateien, Git-Remote, keine App-Dateien | Dokumentationsarbeit erfüllt |
| Schritt 1 | Nutzungskonzept und Vision-Abgleich | historisch dokumentiert; Tagesbeispiel bleibt, ist aber kein UI-Bauplan |
| Schritt 2 | V1-Umfang und spätere Korrektur | frühere Fassung reicht nach neuer Klarstellung nicht als Produktgrundlage |
| Schritt 3, erster Versuch | A/B/C und HTML-Demo | von Dennis als falsche Grundrichtung abgelehnt |
| Aktuelle Korrektur | vollständiger Chat, Repo-Dokumente und vier Screenshots gegen die ursprüngliche Vision geprüft | falsche Annahme identifiziert; Projektgedächtnis neu ausgerichtet |

Noch keine App-Abnahme und kein ausgewähltes Bedienkonzept.

## GitHub

Remote `origin`: `https://github.com/dennisxbu/momentum.git`. `main` war vor dieser Korrektur bei `9a574a9`. PR #5 wird als durch die aktuelle Korrektur überholt behandelt; seine Entwürfe dürfen nicht später unbeabsichtigt als Produktentscheidung zurückkehren.
