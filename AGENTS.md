# Arbeitsweise für Momentum

Diese Datei gilt für jede Arbeit in diesem Repository. Sie ist keine Freigabe, die App zu bauen.

GitHub (`https://github.com/dennisxbu/momentum`) ist die Codeablage und das Projektgedächtnis. Der Chat allein hält den Stand nicht fest. GitHub legt nicht fest, wo spätere persönliche App-Daten gespeichert werden.

## Auftrag

Persönlicher, flexibel anpassbarer Windows-Assistent für Dennis. Ziel ist eine anhand vereinbarter Kriterien geprüfte erste Version für den eigenen Windows-Alltag. Die langfristige Vision ist damit nicht abgeschlossen.

Die Arbeitsnotiz und die ursprüngliche Prompt-Sammlung unter `docs/context/` sind historischer Kontext, kein Pflichtenheft und kein Auftrag, sofort die vollständige Software zu bauen. Der nach der Grundkorrektur gültige Ablauf steht in `docs/arbeitsplan.md`. Jeweils nur der ausdrücklich beauftragte Abschnitt.

## Verbindliche Arbeitsweise

1. Bearbeite ausschließlich den aktuell ausdrücklich beauftragten Schritt. Führe ihn einschließlich seiner notwendigen Prüfung vollständig aus. Beginne danach keinen weiteren Schritt, auch nicht als Bonus, Vorbereitung oder offensichtliche Fortsetzung.
2. Die Arbeitsnotiz enthält Nutzeranforderungen, Vorschläge, Beispiele, Forschungsgrundlagen und offene Entscheidungen. Unterscheide diese Kategorien. Ein Vorschlag der Assistenz ist keine angenommene Produkteigenschaft. Ein Beispieltagesablauf ist keine fest zu programmierende Entscheidungsregel. Eine Git-Zusammenführung ist keine Zustimmung zu jeder darin stehenden Produktannahme. „Nicht beschlossen“ ist kein dauerhaftes Produktverbot.
3. Wähle kleine technische Details innerhalb des vereinbarten Rahmens selbst. Frage nur, wenn eine fehlende Antwort das Produkt, den Umfang, die Datennutzung, externe Kosten oder eine schwer rückgängig zu machende Entscheidung wesentlich verändert. Sammle solche Fragen; unterbrich nicht wegen jeder Kleinigkeit.
4. Interpretiere „sieht gut aus“, „passt“ oder „weiter“ nicht als Freigabe des gesamten Projekts. Wenn der nächste konkrete Schritt bereits eindeutig benannt ist, darf dieser bearbeitet werden; andernfalls kurz klären, welcher Schritt gemeint ist.
5. Füge keine zusätzlichen Funktionen, Abhängigkeiten, Frameworks, Cloud-Dienste, Konten, Hintergrundprozesse oder KI-Anbindungen außerhalb des aktuellen Schritts hinzu. Begründe notwendige neue Abhängigkeiten kurz. Keine grundlegenden Umbauten unter dem Titel „Aufräumen“.
6. Neue Ideen kommen als Vorschläge in eine spätere Aufgabenliste. Sie werden nicht nebenbei implementiert. Ändere nicht selbst den vereinbarten Umfang, damit eine neue Idee hineinpasst.
7. Aktuelle direkte Anweisungen von Dennis haben Vorrang. Wenn eine Grundannahme korrigiert wird, prüfe ihre Auswirkungen auf das Konzept und bereits gebaute Teile. Verteidige eine falsche Grundlage nicht allein deshalb, weil schon Code existiert.

## Produktleitplanken

- Die App soll sich wie ein hochgradig hilfreicher Assistent anfühlen, der Lage, Alternativen und Entwicklungen vorbereitet und Arbeit abnimmt. Eine Karten-, Status- oder Checkbox-Oberfläche mit einem Vorschlagstext erfüllt das nicht.
- Bei Empfehlungen und vorbereiteten Änderungen muss intuitiv verständlich bleiben: Was ist bekannt oder unbekannt, warum ist es relevant, was wird vorgeschlagen und was würde sich nach Bestätigung konkret ändern. Verständlichkeit ist Teil des Nutzens, keine spätere Erklärungsschicht.
- „Notion-Stil“ bezeichnet Freiheit der abbildbaren Gedanken, Eigenschaften und Zusammenhänge, abgespeckt und fokussiert. Es ist weder ein Auftrag zur Notion-Kopie noch zu einer leeren No-Code-Baufläche. Orientierung als Standard, Freiheit auf Nachfrage.
- Studium, smarter Kalender, Diät/Training, berufliche Entwicklung und Alltag sind aktuelle Beispiele, keine fest verdrahteten Produktmodule. Fachfähigkeiten sind optionale Werkzeuge: anpassbar, verschiebbar, pausierbar und nicht verpflichtend. Sie dürfen starke Vorlagen und überprüfbare Speziallogik besitzen, aber keine Ziele, Abläufe oder tägliche Erfassung erzwingen. Ein neuer persönlicher Anwendungsfall darf nicht grundsätzlich eine neue Sonderarchitektur benötigen.
- Eine freie Eigenschaft wird nicht allein durch Name oder Datentyp zur Frist, Priorität oder Aufgabe. Assistenzrelevante Bedeutung und Bezug zu einer aktiven Absicht müssen bestätigt sein; die App darf eine Interpretation nur vorschlagen.
- Herkunft, Zustand, Einheiten, Zeitkonflikte und Schreibbefugnisse bleiben transparent prüfbar. Eine mögliche kontextuelle oder generative Komponente darf nicht unmittelbar persönliche Daten oder geltende Pläne verändern.
- Hochwertige, intuitive, modular veränderbare Windows-App für den privaten Alltag. Benutzung soll Freude machen; geringer Pflegeaufwand allein erfüllt die Vision nicht. Das iPhone ist eine Referenz für Klarheit der Bedienung, keine Aufforderung zu einer iOS-Kopie.
- Schutz vor Druck ist kein Verbot erfreulicher oder motivierender Rückmeldung. Keine Pflicht zu Gamification, Motivationssprüchen oder bestimmten Animationen.
- Windows ist die Zielplattform. Daraus folgt kein Auftrag für mobile Entwicklung und kein endgültiges Verbot einer späteren mobilen Begleitung.
- Geringer Pflegeaufwand und tatsächlicher Nutzen gehören zum Funktionsumfang. Konfigurierbarkeit darf nicht bedeuten, dass die App erst selbst zusammengebaut werden muss.
- „Dynamisch“ heißt kontextbezogen sinnvoll, nicht zufällig variierend. KI ist kein automatischer Beleg für Intelligenz. Korrekte Berechnungen, Validierungen und transparente Regeln bleiben sinnvoll.
- Empfehlungen sollen relevante Daten und Unsicherheit berücksichtigen. Fehlende Daten bedeuten unbekannt. Statistische Zusammenhänge sind keine automatisch belegten Ursachen.
- Wissenschaftliche Fundierung muss für den konkreten Mechanismus begründet sein. Keine nachgewiesene Wirkung dieser noch ungeprüften App versprechen.
- 50 Euro monatliche KI-Kosten sind nicht akzeptabel. Ein konkretes Budget und die KI-Nutzung sind noch nicht beschlossen. 5 Euro waren lediglich ein Vorschlag.
- Für Entwicklung und Vorführungen eindeutig synthetische Daten verwenden. Keine erfundenen Ergebnisse als echte persönliche Analyse ausgeben.
- Keine API-Schlüssel, Tokens oder echten persönlichen Nutzdaten ins Repository oder in Logs schreiben.

## Git und GitHub

Einzelheiten stehen in [docs/entwicklung.md](docs/entwicklung.md). Kurz:

- Nach einem abgeschlossenen Schritt Dokumentation aktualisieren, committen und nach `origin/main` pushen, sofern Dennis nicht ausdrücklich etwas anderes sagt.
- Keine Geheimnisse, keine echten Nutzdaten, kein Force-Push auf `main`, Hooks nicht umgehen.
- In einem neuen Chat zuerst den Wiederaufnahme-Prompt bzw. README, AGENTS.md, Status, Entscheidungen und den vereinbarten Umfang lesen.

## Projektgedächtnis

Vorhandene gleichwertige Dateien weiterverwenden, statt widersprüchliche Doppelstrukturen anzulegen.

- [docs/status.md](docs/status.md) — aktueller Schritt, vorhandene Funktionen, offene Grenzen, nächste Entscheidung, Umfang Version 1, Abnahmeprotokoll
- [docs/entscheidungen.md](docs/entscheidungen.md) — Status, Begründung, Grundlage; Vorschlag und Zustimmung getrennt
- [docs/produktvision.md](docs/produktvision.md) — aktuelle Nordrichtung und Bedeutung von Assistenz sowie Notion-artiger Freiheit
- [docs/arbeitsplan.md](docs/arbeitsplan.md) — aktueller Ablauf und Freigabegrenzen
- [docs/nutzungskonzept.md](docs/nutzungskonzept.md) — Alltagsszenarien mit der aktuellen Grundkorrektur
- [docs/umfang-v1.md](docs/umfang-v1.md) — ausgewählter vertikaler Produktnachweis für Version 1
- [docs/machbarkeitscheck-k2.md](docs/machbarkeitscheck-k2.md) — K2-Ergebnisse zu Semantik, Datenbedarf, Stabilität und Assistenzgrenzen
- [docs/bedienkonzepte.md](docs/bedienkonzepte.md) — K3-Vergleich; Empfehlung V17 ist noch keine Auswahl durch Dennis
- [docs/context/](docs/context/) — Arbeitsnotiz, Prompt-Sammlung, ursprüngliche Momentaufnahmen
- [.cursor/rules/](.cursor/rules/) — kurze, immer geltende Agent-Regeln

## Abschluss jeder Antwort

1. Ergebnis dieses Schritts in wenigen Sätzen.
2. Betroffene Dateien und relevante Entscheidungen.
3. Wie das Ergebnis selbst angesehen oder ausprobiert werden kann.
4. Tatsächlich durchgeführte Prüfungen und verbleibende Grenzen.
5. Höchstens die Entscheidungen, die vor dem nächsten Schritt nötig sind.

Danach stoppen. Die App nie als fertig darstellen, solange die vereinbarte Abnahme fehlt.

## Aktuell nicht tun

Keine App-Dateien, kein Framework, keine Installation, keine produktive UI, keine Datenbank und keine KI-Integration, solange der beauftragte Schritt das nicht ausdrücklich enthält. Insbesondere keine Architektur auf den verworfenen Konzepten A/B/C aufbauen.
