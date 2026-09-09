# Projektstatus

Stand: 9. September 2026

## Schritt und Dokumentationsstand

| Ebene | Stand |
| --- | --- |
| Aktueller Schritt | **K6 — technischer Abnahmeteil durchgeführt; persönliche Abnahme offen.** |
| Auftrag | Dennis hat K6 am 9. September 2026 ausdrücklich beauftragt. |
| Konzepte | 1 „Briefing“, 2 „Studio“ und 3 „Delegat“ unterscheiden sich in Eröffnung, Arbeitsteilung, Korrektur und sichtbarer Freiheit. |
| Empfehlung | Verständliches Briefing als Alltag, Änderungsvertrag aus 3 für Vertrauen, Studio aus 2 und optionale Werkzeuge auf Nachfrage für Freiheit und Tiefe. |
| Produktentscheidung | D24: Konzept 1 „Briefing“ mit verständlicher Bediengrammatik, Änderungsvertrag sowie Studio und optionalen Werkzeugen auf Nachfrage ist ausgewählt. |
| Technikentscheidung | D25–D30: Tauri/React/TypeScript mit Rust-Kern, lokales verschlüsseltes SQLite, schmale Commands, providerneutrale Assistenz und interne optionale Werkzeuge. |
| Ausführbarer Stand | Korrigierter NSIS-Installer ist gebaut, pro Benutzer installiert und als reagierende Windows-App gestartet. |

Die bisherige Empfehlung „A — Tag als Heimat“ bleibt zurückgezogen. V1-Nachweis, Bedienkonzept und Architektur wurden in D18, D24 und D25–D30 jeweils getrennt entschieden.

## Tatsächlich vorhandene Funktionen

Vorhanden sind:

- Projektdokumentation und GitHub als Projektgedächtnis;
- historische Kontextquellen;
- verworfene Konzeptentwürfe ausschließlich in der Git-/PR-Historie;
- ausgewählter und konzeptionell geprüfter V1-Nachweis;
- dokumentierter K2-Trockenlauf und automatisierte K5-Vertragstests für die zentralen Kontrastfälle;
- ausgearbeiteter K3-Konzeptvergleich; die früheren wegwerfbaren Entwürfe bleiben verworfen;
- verbindliche Bediengrammatik „bekannt — Bedeutung — Vorschlag — Wirkung“ und Werkzeugmodell als K3-Korrektur;
- ausführbares Briefing mit „bekannt — Bedeutung — Vorschlag — Wirkung“, Alternative, Unsicherheit und sichtbarer Änderungswirkung;
- konkreter Änderungsvertrag: Analyse schreibt nicht, eine bestätigte Änderung schreibt Zustand und Historie gemeinsam;
- vollständiger synthetischer Verlauf von Morgen über Tagesänderung bis Abend und Folgetag;
- freies Studio mit Sammlungen, Einträgen, Text, Zahl mit Einheit, Datum, Auswahl, Beziehung und versionierter Umbenennung;
- lokaler verschlüsselter SQLCipher-Speicher mit DPAPI-geschütztem Schlüssel;
- vollständiger lesbarer JSON-Export mit validierter Wiederherstellung;
- installierte Windows-App und erzeugter NSIS-Installer ohne zusätzliches Konsolenfenster;
- dokumentierte technische K6-Abnahme mit 16 automatisierten Prüfungen, visueller Kernablaufprüfung und offengelegter WebView2-Netzwerkgrenze;
- abgeschlossene [K4-Architekturentscheidung](architektur-k4.md) und dokumentierte [K5-Implementierung](k5-implementierung.md).

Nicht vorhanden sind echte persönliche Daten, externe Konten, ein KI-Anbieter, Kalender-/Diätwerkzeuge, allgemeines semantisches Verständnis freier Themen oder eine persönliche Bedienabnahme.

## Aktuelle Quellenreihenfolge

1. aktuelle direkte Aussagen von Dennis;
2. [produktvision.md](produktvision.md) und [entscheidungen.md](entscheidungen.md);
3. dieser Status und der ausgewählte [V1-Nachweis](umfang-v1.md);
4. der abgeschlossene [Machbarkeitscheck K2](machbarkeitscheck-k2.md);
5. [nutzungskonzept.md](nutzungskonzept.md);
6. historische Arbeitsnotiz und Prompt-Sammlung unter `docs/context/`.

Historische Beispiele oder frühere Vorschläge dürfen eine neuere Korrektur nicht überschreiben.

## Offene Grenzen

- **O1:** Der vertikale Produktnachweis ist installiert und technisch geprüft; persönliche Assistenzqualität, Bediengefühl und Nutzen sind noch nicht durch Dennis abgenommen.
- **Bedienkonzept umgesetzt:** D24 ist als verständliches Briefing, Änderungsvertrag und Studio auf Nachfrage umgesetzt. Die visuelle K6-Prüfung bestand; die persönliche Benutzbarkeit bleibt offen.
- **Technik geklärt:** D25–D30 wählen Stack, lokalen verschlüsselten Speicher, Offline-Grundsatz, Schichtgrenzen, Export und erste Windows-Verteilung.
- **O4:** KI-Einsatz, Anbieter oder lokaler Betrieb, erlaubte Daten und Budget unter 50 Euro bleiben vor einer produktiven Anbindung offen. Die Architektur bindet sich an keinen Anbieter.
- **O5:** Für V1 sind Analyse ohne Bestätigung, Datenänderung nach konkreter Bestätigung und keine externen Aktionen gewählt. Spätere Befugnisse bleiben offen.
- **O6/O11:** Die V1-Tiefe der Freiheit ist gewählt. K2 verlangt bestätigte Semantik, aber keine weitere Eigenschaftsform; langfristige Tiefe bleibt offen.
- **O7–O10:** Diätanalyse und smarter Kalender sind als optionale Werkzeugideen festgehalten; genaue Tiefe, Daten, Integrationen und Zeitpunkt sowie Abendrhythmus und berufliche Entwicklung bleiben fachlich offen.
- **O12:** Momentum selbst verwendet keinen Onlinedienst. Die Windows-WebView2-Laufzeit öffnete im K6-Test dennoch technische Herstellerverbindungen; vollständige Netzwerkstille ist nicht belegt.

GitHub ist Codeablage. D26 legt den lokalen Anwendungsdatenordner des aktuellen Windows-Benutzers als Speicherort fest; echte persönliche Daten gehören weiterhin niemals ins Repository.

## Umfang der ersten Version

Der frühere Vorschlag V9/V10 bleibt als alleinige Grundlage **zurückgezogen**. Der ausgewählte [V1-Nachweis](umfang-v1.md) verbindet zwei Leistungen:

1. Die App übernimmt in einem vollständigen Tagesablauf sichtbar Denk-, Ordnungs- und Auswertungsarbeit.
2. Ein vorher nicht fest eingebautes persönliches Thema lässt sich als eigene Sammlung mit fünf Eigenschaftsformen, einer Beziehung und einer passenden Sicht ohne Code abbilden und vom Assistenten verwenden.

Als synthetischer Prüffall dient „Arbeitsplatz verbessern“ mit „Stuhl-Kandidaten“. K2 ergänzt als zurückgehaltenen Fall „Rezeptideen“ und zeigt: Die freien Daten funktionieren im Assistenzkontext nur mit bestätigter Bedeutung und Beziehung. Eine vollständige Notion-Kopie, Plugin-Plattform oder automatische Lebenssteuerung ist dadurch nicht beauftragt.

## Nächste inhaltliche Arbeit

Der technische K6-Teil ist in [k6-abnahme.md](k6-abnahme.md) festgehalten. Als Nächstes ist ausschließlich Dennis' kurzer Durchlauf der installierten App mit dem Urteil **angenommen**, **angenommen mit Änderungen** oder **nicht angenommen** nötig. Vor diesem Urteil beginnt kein weiterer Abschnitt.

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
| K5, Oberfläche und Kern | vollständiger synthetischer Assistenzablauf und freies Studio auf der K4-Architektur | ausführbar; TypeScript-/Vite-Build und zwölf Rust-Prüfungen bestanden |
| K5, Schutz und Haltbarkeit | SQLCipher-Dateikopf, DPAPI-Schlüssel, Neustart, Rollback, Definitionshistorie und Export/Wiederherstellung | technisch bestanden mit synthetischen Daten |
| K5, Windows-Paket | Desktopstart und optimierter NSIS-Build | `momentum.exe` reagierte; Installer erzeugt, aber noch nicht installiert oder persönlich abgenommen |
| K6, Installation und Start | korrigierten NSIS-Installer pro Benutzer installiert; Hauptfenster, GUI-Subsystem und Deinstallation geprüft | technisch bestanden |
| K6, Oberfläche | Briefing, Antwort, Änderungsvertrag, Bestätigung, Studio, Sammlungseditor und Datenhoheit visuell durchlaufen | bestanden in lokaler WebView-Vorschau; native Bilderfassung war auf diesem System technisch nicht möglich |
| K6, Assistenzvarianten | relevante und belanglose Änderungen, unbestätigtes Datum, Zustandsfortführung und unbekannte Durchführung | sechs Kontrastfälle bestanden; „Rezeptideen“ und Freitext-Deduplizierung noch nicht bestanden |
| K6, Daten | Neustart, echter Datei-Export, Rücksetzung, validierte Wiederherstellung und ungültige Sicherung | technisch bestanden |
| K6, Netzwerk und Kosten | Quellcode, Paket und laufende Prozessgruppe geprüft | keine Momentum-Dienste oder Kosten; WebView2-Herstellerverbindungen beobachtet und offengelegt |

Noch keine persönliche App-Abnahme. Der K6-Stand ist installiert und technisch geprüft; ob er sich für Dennis wirklich intuitiv, entlastend und hochwertig genug anfühlt, bleibt sein ausstehender Nutzungstest.

## GitHub

Remote `origin`: `https://github.com/dennisxbu/momentum.git`. PR #5 wird als durch die aktuelle Korrektur überholt behandelt; seine Entwürfe dürfen nicht später unbeabsichtigt als Produktentscheidung zurückkehren.
