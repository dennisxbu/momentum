# K5 — Ausführbarer vertikaler Kern

Stand: 9. September 2026 · **abgeschlossen**

## Ergebnis

K5 liefert erstmals eine startbare Momentum-Windows-App. Der gebaute Kern beweist nicht möglichst viele Funktionen, sondern die zwei tragenden V1-Annahmen gemeinsam:

1. Momentum bereitet eine Lage vor, erklärt Bedeutung, macht einen begründeten Vorschlag und verändert einen geltenden Plan erst nach einer konkreten Bestätigung.
2. Derselbe Ablauf verwendet eine frei strukturierte Sammlung mit typisierten Eigenschaften und Beziehungen, ohne ein Möbel-, Einkaufs- oder Tagesplaner-Modul fest einzubauen.

Der Stand verwendet ausschließlich klar gekennzeichnete synthetische Daten. Er ist ein Entwicklungs- und Produktnachweis, keine persönliche Nutzenabnahme und keine fertige Version 1.

## Erlebbarer Ablauf

Das primäre Briefing folgt durchgängig der vereinbarten Grammatik **bekannt — Bedeutung — Vorschlag — Wirkung**. Im synthetischen Donnerstag geschieht Folgendes:

- Momentum verbindet Uni-Zeit, Testat, geplanten Lernblock und eine bestätigte Rückgabefrist aus der freien Sammlung.
- Statt einen Weg zu erfinden, fragt es genau nach der für die Reihenfolge entscheidenden Rückkehrzeit.
- Danach entsteht ein konkreter Vorschlag mit Grund, Alternative und erkennbarem Nachteil.
- Vor der Übernahme zeigt ein Änderungsvertrag ausschließlich die geplanten Änderungen. Erst die Bestätigung schreibt den geltenden Plan und ein Änderungsereignis.
- Eine 90 Minuten längere Uni führt zu einer begrenzten Neuplanung; bewusst unveränderte Teile werden ebenfalls genannt.
- Die belanglose, semantisch unbestätigte Stuhlfarbe ändert Empfehlung und Begründung nicht.
- Der Abend verwendet Plan und Korrektur wieder und fragt nur nach der für morgen nützlichen Lerninformation.
- Eine ausgelassene Antwort bleibt unbekannt. Am Folgetag wird nur eine bestätigte Durchführung weiterverwendet, ohne daraus Kompetenz zu erfinden.

Das Studio ist ein tieferer Raum auf Nachfrage. Dort lassen sich Sammlungen und Einträge ohne Code anlegen. Unterstützt werden genau die fünf für V1 beschlossenen Formen Text, Zahl mit Einheit, Datum, Auswahl und Beziehung. Eine assistenzrelevante Bedeutung muss ausdrücklich beschrieben und bestätigt werden. Eigenschaftsumbenennungen erzeugen eine neue Version; vorhandene Werte behalten ihren damaligen Namen und ihre damalige Bedeutung.

## Technische Umsetzung

- Tauri 2.11.5 als Windows-Desktophülle;
- React 19.3.0, TypeScript 7.0.2 und Vite 8.2.2 für die Oberfläche;
- Rust-Anwendungskern mit getrennten Domänen-, Anwendungs-, Infrastruktur-, Werkzeug- und Desktopgrenzen;
- `rusqlite` 0.40.2 mit gebündeltem SQLCipher und vendored OpenSSL;
- zufälliger 256-Bit-Datenbankschlüssel, geschützt durch Windows DPAPI im Bereich des aktuellen Benutzers;
- schmale fachliche Tauri-Commands; React besitzt keine Datei-, SQL- oder Shell-Berechtigung;
- providerneutrale `Reasoner`-Schnittstelle mit lokaler deterministischer K5-Implementierung ohne Netzwerk und ohne KI-Anbieter;
- versionierter lesbarer JSON-Export und Wiederherstellung nach vollständiger Prüfung in einem temporären leeren Datenbestand;
- NSIS-Installer im Modus pro Benutzer.

Die laufenden Daten liegen unter `%LOCALAPPDATA%\de.dennis.momentum`. Datenbank, Schlüssel und Exporte werden nicht im Repository gespeichert. Der Export ist bewusst lesbar und deshalb nur an einem sicheren Ort abzulegen.

## Reproduzierbarer Start

Auf dem Entwicklungsrechner sind Node.js, Rust, Visual Studio C++/Windows SDK, WebView2 und Strawberry Perl vorhanden. Strawberry Perl wird nur zum Bauen der eingebetteten Kryptobibliothek benötigt.

Im Projektordner:

```powershell
npm install
npm run tauri -- dev
```

Produktionspaket erzeugen:

```powershell
npm run tauri -- build
```

Der nicht eingecheckte Installer entsteht unter:

```text
src-tauri/target/release/bundle/nsis/Momentum_0.1.0_x64-setup.exe
```

## Durchgeführte Prüfungen

| Bereich | Tatsächlich geprüft | Ergebnis |
| --- | --- | --- |
| Oberfläche | TypeScript-Prüfung und optimierter Vite-Build | bestanden |
| Nativer Kern | alle Rust-Ziele kompiliert | bestanden |
| Verschlüsselung | echte erzeugte Datenbank beginnt nicht mit dem SQLite-Klartextkopf | bestanden |
| Windows-Schutz | Schlüsseldatei wird durch DPAPI erzeugt und beim Neustart wieder geöffnet | bestanden |
| Freie Struktur | alle fünf Typen und Beziehung im generischen Modell | bestanden |
| Semantik | nur bestätigte Datumsbedeutung mit bestätigtem Vorhabensbezug gelangt in den Briefing-Kontext | bestanden |
| Befugnis | Vorschlag schreibt nicht; konkrete Bestätigung schreibt Plan und Historie | bestanden |
| Stabilität | unbestätigte Farbangabe ändert Empfehlung und Begründung nicht | bestanden |
| Tagesänderung | bestätigte Uni-Verlängerung erzeugt begrenzten neuen Änderungssatz | bestanden |
| Abend/Folgetag | vollständiger Ablauf einschließlich bewusst unbekannter Durchführung | bestanden |
| Historie | ungültiger typisierter Eintrag rollt zusammen mit seinem Ereignis zurück | bestanden |
| Umbenennung | neue Definitionsversion, alte Werte bleiben historisch verständlich | bestanden |
| Neustart | bestätigter Zustand bleibt nach erneutem Öffnen erhalten | bestanden |
| Export/Wiederherstellung | vollständiger semantisch gleicher JSON-Rundlauf | bestanden |
| Optionales Werkzeug | ausgeblendetes Beispielwerkzeug trägt nichts zum Kontext bei | bestanden |
| Desktopstart | erzeugte `momentum.exe` gestartet, reagierender Prozess und lokale Daten angelegt | bestanden |
| Installer | optimierter Build und NSIS-Paket erzeugt | bestanden |

Insgesamt bestehen zwölf automatisierte Rust-Prüfungen. Der Installer wurde in K5 gebaut, aber noch nicht installiert; die echte Installations-, Offline- und persönliche Bedienabnahme gehört getrennt zu K6.

## Bewusste Grenzen

- Keine Kalender-, E-Mail-, Uni-, Fitness- oder Gesundheitsanbindung.
- Kein Diät-, Trainings- oder Kalenderwerkzeug.
- Kein KI-Anbieter, keine Modellkosten und keine externen Datenflüsse.
- Keine umfassende Notion-Kopie, Formeln, Skripte oder Laufzeit-Plugins.
- Noch keine persönliche Aussage darüber, ob sich das Briefing für Dennis wirklich entlastend, intuitiv und hochwertig genug anfühlt.
- Der synthetische Ablauf ist absichtlich schmal. Weitere Assistenzqualität und zurückgehaltene Varianten werden erst in K6 geprüft.

## Grenze zum nächsten Schritt

K6 prüft Installation, Offline-Verhalten, Bediengefühl, Assistenzqualität, Datenhoheit und persönlichen Nutzen. Dieser Schritt beginnt nur nach ausdrücklichem Auftrag und darf die noch offenen KI-, Kalender- oder Diätentscheidungen nicht nebenbei treffen.
