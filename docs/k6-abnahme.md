# K6 — Alltag, Auslieferung und Abnahme

Stand: 9. September 2026 · **technischer Teil durchgeführt, persönliche Abnahme offen**

## Ergebnis

Der korrigierte Momentum-Stand lässt sich als Windows-App bauen, pro Benutzer installieren und starten. Der vollständige synthetische Kernablauf, das Studio und die Datenhoheitsansicht wurden visuell durchlaufen. Speicherung, echter Datei-Export und validierte Wiederherstellung sind automatisiert geprüft.

K6 ist trotzdem noch nicht vollständig abgenommen. Zwei Grenzen sind dafür ausschlaggebend:

1. Ob die Bedienung für Dennis ohne Erklärung verständlich, hochwertig und tatsächlich entlastend ist, kann nur Dennis im installierten Programm beurteilen.
2. Die lokale deterministische Beurteilung besteht mehrere zurückgehaltene Kontrastfälle, aber noch nicht den vollständigen fremden Fall „Rezeptideen“. Sie ist weiterhin ein schmaler Produktnachweis und noch keine allgemeine kontextuelle Assistenz.

Momentum wird deshalb nicht als fertige Version 1 bezeichnet.

## Während K6 behobene Fehler

- Der Studiowechsel rief in der React-Oberfläche einen nicht vorhandenen Tauri-Befehl auf. Der Name stimmt nun mit dem Rust-Kern überein.
- Der Release-Prozess war als Konsolenprogramm gebaut und öffnete neben Momentum ein unnötiges Terminalfenster. Das Windows-Paket verwendet nun das GUI-Subsystem.
- Ohne bestätigte Datumsbedeutung konnte die lokale Formulierungsschicht dennoch eine Rückgabefrist behaupten. Dieser Zweig bleibt jetzt ausdrücklich ohne Planwirkung und erfindet weder Frist noch Aufgabe.
- Ein sichtbarer Text war unnötig an die Bezeichnung „K5“ gekoppelt. Er beschreibt nun zeitlos den Entwicklungsstand.
- Die Aussage zur Netzwerkfreiheit war zu absolut. Momentum selbst hat keinen Onlinedienst; die verwendete Windows-WebView2-Laufzeit kann jedoch unabhängig davon technische Herstellerverbindungen aufbauen. Die Oberfläche benennt diese Grenze jetzt.

## Technische Abnahme

| Bereich | Tatsächlich geprüft | Ergebnis |
| --- | --- | --- |
| Windows-Installation | NSIS-Installer still pro Benutzer installiert; Programm und Deinstallation unter `%LOCALAPPDATA%\Momentum` vorhanden | bestanden |
| Start | installierte `momentum.exe` gestartet; Hauptfenster „Momentum“ reagiert | bestanden |
| Fensterart | installierte und gebaute EXE als `Windows GUI` geprüft | bestanden; kein zusätzliches Konsolenfenster vorgesehen |
| Produktiv-Build | TypeScript-Prüfung, optimierter Vite-Build und Tauri-Release-Build | bestanden |
| Nativer Kern | Formatprüfung, Clippy ohne Warnungen und alle Rust-Ziele | bestanden |
| Speicherung | verschlüsselte SQLCipher-Datei, DPAPI-Schlüssel und Zustand über Neustart | bestanden |
| Export/Wiederherstellung | Export in eine echte JSON-Datei, Zurücksetzen und Wiederherstellen; ungültige Datei ersetzt den lokalen Stand nicht | bestanden |
| Oberfläche | Briefing, relevante Antwort, Änderungsvertrag, Bestätigung, Studio, Sammlungseditor und Datenhoheit durchlaufen | bestanden in isolierter lokaler WebView-Vorschau |
| Layout | 1265 Pixel nutzbare Breite, kein horizontaler Überlauf, keine unbeschrifteten Schaltflächen | bestanden |
| Installierte Dateidialoge | Export-/Importdialog der installierten nativen App | noch nicht persönlich durchgeklickt |

Die native Bildautomatisierung konnte das Tauri-Fenster auf diesem Windows-System zweimal nicht erfassen (`SetIsBorderRequired`, `0x80004002`). Nach der vorgeschriebenen Wiederholung wurde sie nicht weiter erzwungen. Die visuelle Prüfung verwendete deshalb dieselbe React-/WebView-Oberfläche mit isoliertem synthetischem Zustand; Kern und Speicher wurden separat durch echte Rust- und Dateiprüfungen abgedeckt.

## Zurückgehaltene Assistenzvarianten

| Variante | Erwartung | Ergebnis |
| --- | --- | --- |
| Rückkehr direkt oder später | relevante Antwort muss Empfehlung und Begründung ändern | bestanden |
| belanglose Farbangabe | Empfehlungskern bleibt stabil | bestanden |
| unbestätigtes Datumsfeld | keine erfundene Frist oder Aufgabe | bestanden; Filter und Formulierung getrennt geprüft |
| bestätigte freie Datumsbedeutung mit Beziehung | Sammlung, Eintrag und Bezug gelangen in den Kontext | bestanden auf Strukturebene |
| verlängerte Uni | nur der betroffene Abschnitt wird neu vorgeschlagen; noch keine automatische Planänderung | bestanden im vollständigen Ablauf |
| ausgelassene Abendantwort | Durchführung bleibt unbekannt und wird nicht als Misserfolg gewertet | bestanden |
| fremder Fall „Rezeptideen“ | passende Abendoption aus bestätigter Verderbgrenze ableiten, ohne Ernährungsmodul | **noch nicht bestanden**; freie Struktur reicht, die lokale Beurteilung ist dafür semantisch zu schmal |
| gleichbedeutender Freitext | keine doppelte Evidenz aus bloßer Umformulierung | **noch nicht umgesetzt** |

Damit ist die Sicherheits- und Zustandslogik belastbarer als vor K6. Die allgemeine Assistenzqualität ist aber noch nicht bewiesen. Eine spätere kontextuelle Komponente bleibt erforderlich, wenn Momentum unterschiedliche freie Themen wirklich inhaltlich verstehen soll; Anbieter, Datennutzung und Budget werden dadurch nicht vorentschieden.

## Offline, Netzwerk, Kosten und Daten

- Der Momentum-Quellcode enthält keinen Laufzeit-Netzclient und keinen Aufruf eines externen Dienstes. Die Produktionsoberfläche wird aus dem Installationspaket geladen.
- Der Kern benötigt weder Konto noch Internet und hat keine Modell- oder API-Kosten.
- Beim Laufzeittest öffnete der zu Momentum gehörende WebView2-Netzwerkprozess fünf externe HTTPS-Verbindungen. Die Prozessrolle war `network.mojom.NetworkService`; es gab keinen Hinweis auf eine Übertragung von Momentum-Inhalten. Eine vollständig netzwerkstille Prozessgruppe ist damit jedoch **nicht** belegt.
- Es wurden keine experimentellen Browser-Schalter eingebaut, um diese Beobachtung nur scheinbar zu unterdrücken. Microsoft beschreibt WebView2-Browser-Schalter als Entwicklungs- und Diagnosemittel und rät davon ab, sie in Produktions-Apps auszuliefern: [WebView2 browser flags](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/webview-features-flags).
- Die Inhaltsrichtlinie der App erlaubt der React-Oberfläche nur eigene lokale Quellen. Persönliche Daten liegen verschlüsselt unter `%LOCALAPPDATA%\de.dennis.momentum`.
- Ein physischer Offline-Test durch Trennen der Windows-Netzwerkverbindung wurde nicht durchgeführt, weil K6 keine Änderung der Rechnerverbindung voraussetzt.

## Persönliche Abnahme durch Dennis

Die installierte App soll einmal ohne Entwicklungsanleitung durchlaufen werden. Für die Abnahme reichen fünf kurze Urteile:

1. Verstehst du beim ersten Briefing, was Momentum weiß und warum es fragt?
2. Ist vor jeder Übernahme eindeutig, was sich ändert und was gleich bleibt?
3. Fühlt sich der Ablauf nach hilfreicher Denk- und Ordnungsarbeit an statt nach Checkbox-Verwaltung?
4. Wirkt das Studio frei genug, ohne dass du die App erst selbst zusammenbauen musst?
5. Würdest du diesen Kern im Alltag freiwillig wieder öffnen?

Antwortformat: **angenommen**, **angenommen mit Änderungen** oder **nicht angenommen**, jeweils mit den wichtigsten Beobachtungen. Erst danach kann O1 geschlossen und K6 abgeschlossen werden.

## Paket

Der lokale Installer liegt außerhalb der Git-Historie unter:

```text
src-tauri/target/release/bundle/nsis/Momentum_0.1.0_x64-setup.exe
```

SHA-256 des finalen K6-Pakets:

```text
C577EEE44A0C78198B1E21BA057AE5674A6D0CE2F611CFBE30A4428C770CCC4C
```
