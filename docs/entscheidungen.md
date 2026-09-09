# Entscheidungen

Vorschlag und Zustimmung bleiben getrennt. Eine Idee aus der Arbeitsnotiz oder aus einem Schritt ist kein Beschluss, solange der Status nicht „beschlossen“ oder **Fakt** ist.

Eine Zusammenführung in Git ist keine automatische Zustimmung zu jeder Produktannahme in den zusammengeführten Dateien.

Legende: **beschlossen** · **Fakt** (von Dennis genannt, keine fertige Umsetzung) · **Vorschlag** · **überholt** · **offen**

„Offen“ und „nicht beschlossen“ sind keine dauerhaften Verbote.

Die Arbeitsnotiz unter `docs/context/` bleibt historische Quelle. Neue Festlegungen stehen hier.

## Beschlossen oder als Fakt festgehalten

| ID | Thema | Status | Inhalt | Grundlage |
| --- | --- | --- | --- | --- |
| D1 | Zielplattform und Nutzung | Fakt | Persönliche Windows-App für Dennis, privater Alltag, ein Nutzer. Kein kommerzielles Produkt, keine Mehrbenutzer-App, kein öffentliches Angebot. Windows ist die Zielplattform. Daraus folgt kein Auftrag, jetzt mobil zu entwickeln, und kein endgültiges Verbot einer späteren mobilen Begleitung. | Original 1 und 6; Korrektur Dennis 8.9.2026 |
| D2 | 50-Euro-Grenze | beschlossen | Monatliche KI-API-Kosten von etwa 50 Euro bei täglicher Nutzung sind nicht akzeptabel. | Original 6 |
| D3 | Codeablage | Fakt | Projektdateien liegen in `https://github.com/dennisxbu/momentum`. | Dennis, 8.9.2026 |
| D4 | GitHub ≠ App-Daten | beschlossen | GitHub entscheidet nicht, wo persönliche App-Daten gespeichert werden. | Prompt-Sammlung Schritt 4; Initialprompt |
| D5 | Synthetische Entwicklungsdaten | beschlossen | Entwicklung und Vorführungen verwenden eindeutig synthetische Daten. | Initialprompt |
| D6 | Schrittarbeit | beschlossen | Jeweils nur der ausdrücklich beauftragte Schritt. Keine vollständige Software aus der Vision ableiten. | Prompt-Sammlung; AGENTS.md |
| D7 | iOS-Kopie | beschlossen | iPhone ist Bediengefühl-Referenz, kein Auftrag zu einer iOS-Optik. | Original 3 |
| D8 | Visuelle und erfreuliche Nutzung | Fakt | Detailliert, visuell anregend und hochwertig; Benutzen soll Spaß machen. Geringer Pflegeaufwand allein reicht nicht. | Original 1, 2 und 4; Korrektur 8.9.2026 |
| D9 | Intuitive Anpassbarkeit | Fakt | Modular, möglichst wenig fest verdrahtet, Platz für Ziel- und Bereichsänderungen; Konfigurierbarkeit so intuitiv wie möglich. | Original 1 und 3 |
| D10 | Daten werden verarbeitet | Fakt | Mit Daten und Informationen soll etwas passieren: Statistiken, Trends, Analysen — nicht triviale Anzeigen oder bloß rotierende Sprüche. | Original 2 |
| D11 | Kontextbezogene Unterstützung | Fakt | Vorschläge sollen in der Lage sinnvoll sein, nicht weil fest `A + B = C` gilt. | Original 5 |
| D12 | Wissenschaftlich begründete Prozesse | Fakt | Tools und Prozesse sollen einen klaren wissenschaftlichen oder psychologischen Ansatz haben, auf Dennis abgestimmt, als investierte Zeit. Die Wirkung *dieser* App ist nicht bewiesen. | Original 2 |
| D13 | Ausgelassene Reflexion | Fakt | Eine ausgelassene Reflexion löscht keine vorhandenen Informationen und beweist keine unterlassene Aktivität. Unbekannt bleibt unbekannt. | Korrektur Dennis 8.9.2026 |
| D14 | Offenheit für Motivation | Fakt | Grundsätzlich nichts gegen Motivationssprüche; Ablehnung gilt einer billigen App ohne Substanz. Keine Pflicht zu Gamification, Sprüchen oder Animationen. | Original 2; Korrektur 8.9.2026 |
| D15 | Assistenz statt Verwaltungsoberfläche | Fakt | Momentum soll sich wie ein hochgradig hilfreicher persönlicher Assistent anfühlen, der Denk-, Ordnungs- und Auswertungsarbeit übernimmt. Eine App, in der Dennis hauptsächlich Karten liest, Checkboxen anklickt und Zustände pflegt, verfehlt die Richtung. | Direkte Korrektur Dennis 9.9.2026; vier Screenshots aus Schritt 3 |
| D16 | Notion-artige Freiheit | Fakt | Dennis will weg von starren, fest programmierten Zielen und Strukturen. „Notion-Stil“ beschreibt die Freiheit, grundsätzlich sehr unterschiedliche eigene Gedanken und Zusammenhänge abbilden zu können — abgespeckt, fokussiert und nicht als Notion-Kopie. | Direkte Präzisierung Dennis 9.9.2026; ursprüngliche Aussagen zu wenig Hardcoding |
| D17 | Bedienkonzepte A/B/C verworfen | Fakt | Die Konzepte aus PR #5 sind keine auswählbaren Produktgrundlagen mehr. Das gemeinsame Karten-/Statusmodell und die dünne Assistenz gehen in die falsche Richtung; Empfehlung A ist zurückgezogen. | Direkte Bewertung Dennis 9.9.2026 |
| D18 | V1-Nachweis ausgewählt | beschlossen | V1 beweist einen vollständigen Assistenzkreislauf zusammen mit einer frei angelegten Sammlung. Der synthetische Fall „Stuhl-Kandidaten“ verwendet eigene Eigenschaften, eine Beziehung und eine passende Sicht tatsächlich im Tageskontext. Dauerhafte Speicherung und echte Wiederherstellung gehören zum Nachweis. | Dennis delegiert nächste fachliche Entscheidung 9.9.2026; K1; [umfang-v1.md](umfang-v1.md) |
| D19 | Befugnisse und Datenquellen in V1 | beschlossen | Die App darf intern analysieren und Vorschläge vorbereiten. Nutzerdaten oder geltende Pläne ändert sie nur aufgrund einer konkreten Aussage oder Bestätigung. Externe Aktionen sind ausgeschlossen. Der V1-Nachweis verwendet lokal angelegte beziehungsweise synthetische Daten und keine externen Kontenanbindungen. | K1; [umfang-v1.md](umfang-v1.md) |
| D20 | Semantische Brücke für freie Daten | beschlossen | Eine freie Eigenschaft beeinflusst Empfehlungen erst, wenn ihre Bedeutung und ihr Bezug zu einer aktiven Absicht bestätigt sind. Ein Datums- oder Zahlenfeld erzeugt allein keine Frist, Priorität oder Handlung. Die App darf eine Interpretation vorschlagen, aber nicht stillschweigend als Tatsache behandeln. | Ergebnis K2; [machbarkeitscheck-k2.md](machbarkeitscheck-k2.md) |
| D21 | Verlässliche Assistenzgrenze | beschlossen | Faktenherkunft, Zustand, Einheiten, Zeitkonflikte und Schreibbefugnisse werden unabhängig von einer möglichen kontextuellen Komponente transparent geprüft. Eine generative oder anderweitig kontextuelle Komponente erhält keine direkte Schreibbefugnis. | Ergebnis K2; [machbarkeitscheck-k2.md](machbarkeitscheck-k2.md) |
| D22 | Verständlichkeit erzeugt Nutzengefühl | Fakt | Dennis möchte jederzeit intuitiv verstehen, was Momentum weiß, warum etwas relevant ist, was es empfiehlt und was sich dadurch ändern würde. Aus „Ich verstehe, was hier passiert“ soll das Gefühl entstehen: „Das bringt mich weiter.“ | Direkte Präzisierung Dennis 9.9.2026 |
| D23 | Fachfähigkeiten sind optionale Werkzeuge | Fakt | Ein smarter Kalender und ein Werkzeug zum Erfassen und Analysieren einer Diät sind gewünschte Beispiele, aber keine verpflichtenden, fest verdrahteten Lebensbereiche. Solche Werkzeuge müssen veränderbar, verschiebbar, pausierbar und vollständig weglassbar sein. | Direkte Präzisierung Dennis 9.9.2026 |

Die konkrete Oberfläche, Datenarchitektur und technische Umsetzung bleiben offen. D18 wählt einen begrenzten Produktnachweis, keine vollständige Notion-Funktionsliste.

## Vorschläge (nicht beschlossen)

| ID | Thema | Status | Inhalt | Grundlage |
| --- | --- | --- | --- | --- |
| V1 | Navigation | Vorschlag | Ansichten „Heute“, „Überblick“, „Detail“. | Assistenz, Arbeitsnotiz |
| V2 | Produktname / Metapher | Vorschlag | „Persönliche Steuerzentrale“, Name „Momentum“. Ordner- und Reponame sind noch kein festgelegter Produktname. | Assistenz; Repo-Name |
| V3 | KI-Aufgabenteilung | Vorschlag | Berechnungen lokal; KI für Kontext und Formulierungen. | Assistenz |
| V4 | KI-Budgetziel | Vorschlag | Rund 5 Euro monatlich als Entwicklungsziel. | Assistenz; nicht von Dennis bestätigt |
| V5 | Bausteine | Vorschlag | Ziele, Projekte/Aufgaben, Routinen, Messwerte, Notizen, Ansichten. | Assistenz |
| V6 | Nutzungskonzept | Vorschlag | Tagesorientierung plus Prüfszenarien (Überlastung, Wiedereinstieg, Zielverlauf, Zieländerung). Die Verständnishilfe zu Arten von Dingen ist kein Datenmodell. | Schritt 1; [nutzungskonzept.md](nutzungskonzept.md) |
| V7 | Ausgestaltung des Tages | Vorschlag | Bestimmtes Tagesmodell; Vermeiden einer Schulnote für den ganzen Tag. Kein Verbot erfreulicher Rückmeldung. | Schritt 1; nicht mit D8–D12 verwechseln |
| V8 | Früher Machbarkeitscheck | Vorschlag | Vor verbindlicher Architektur Assistenzqualität, nötige Daten, Pflegeaufwand und flexible Abbildung eines nicht vorgebauten Themas prüfen. Eigener Auftrag, keine kostenpflichtigen Aufrufe. | Korrektur Schritt 2 und 9.9.2026; [umfang-v1.md](umfang-v1.md) |
| V9 | Frühere V1-Abgrenzung | überholt | Der frühere Vorschlag mit nützlichem Tag, Verlauf, einzelner Messgröße sowie Export/Wiederherstellung konnte dennoch zu einer manuellen Karten-/Tracker-App führen. Er wird nicht unverändert ausgewählt. | Schritt 2; Korrektur Dennis 9.9.2026 |
| V10 | Frühere kleinste Anpassbarkeit | überholt | Eine frei benannte Messgröße mit Einheit bleibt möglicherweise nützlich, beweist allein aber nicht die von Dennis gemeinte Freiheit. | Korrektur Schritt 2; Präzisierung Dennis 9.9.2026 |
| V11 | Flexible Grundlage, fokussierte Oberfläche | Vorschlag | Wenige verständliche Grundbausteine ermöglichen eigene Strukturen, Eigenschaften, Beziehungen und Sichten. Gute Vorlagen und eine kuratierte Alltagsoberfläche verhindern, dass Dennis zuerst ein leeres System bauen muss. | Synthese aus D9, D15 und D16; [produktvision.md](produktvision.md) |
| V12 | Assistenzzyklus | Vorschlag | Lage zusammenführen, entscheidende Lücke erkennen, Alternativen abwägen, begründet vorschlagen, nur mit Befugnis handeln und aus Korrektur sowie Ergebnis lernen. | Synthese aus D10–D16; [produktvision.md](produktvision.md) |
| V13 | Begrenzte Kombination für Assistenz | Vorschlag | Eine transparente Fakten- und Berechnungsschicht wird mit einer austauschbaren kontextuellen Beurteilung kombiniert. Das ist nach K2 die tragfähigste Annahme für K3, aber noch keine Wahl von KI, Modell, Anbieter oder Technik. | Vergleich in K2; [machbarkeitscheck-k2.md](machbarkeitscheck-k2.md) |
| V14 | K3 1 — Briefing | Vorschlag | Momentum eröffnet den Alltag mit einer vorbereiteten Einordnung, Empfehlung, Alternative und genau dem Entscheidungsmoment mit größter Auswirkung. Details und freie Strukturen liegen auf Nachfrage dahinter. | K3; [bedienkonzepte.md](bedienkonzepte.md) |
| V15 | K3 2 — Studio | Vorschlag | Ein verbundener persönlicher Denkraum ist der Grundmodus. Dennis beginnt bei Gegenständen und Zusammenhängen; Momentum schlägt Strukturen, Bedeutungen und Sichten vor. | K3; [bedienkonzepte.md](bedienkonzepte.md) |
| V16 | K3 3 — Delegat | Vorschlag | Momentum bereitet im erlaubten Leseraum gebündelte Entscheidungen und Änderungssätze vor. Dennis prüft nur bedeutende Freigaben, nicht alle zugrunde liegenden Daten. | K3; [bedienkonzepte.md](bedienkonzepte.md) |
| V17 | Empfohlene Leitform | Vorschlag | Verständliches Briefing als primärer Alltag, begrenzter Änderungsvertrag aus Konzept 3 für Vertrauen sowie Studio aus Konzept 2 und optionale Werkzeuge nur auf Nachfrage für Freiheit und Tiefe. Die Rollen bleiben getrennt; es entsteht keine Oberfläche mit allen Elementen gleichzeitig. | Empfehlung der Software-Expertise in K3; [bedienkonzepte.md](bedienkonzepte.md) |
| V18 | Werkzeugmodell | Vorschlag | Fachfähigkeiten liegen als aktivierbare und anpassbare Werkzeuge beziehungsweise Vorlagen auf der gemeinsamen flexiblen Grundlage. Sie dürfen starke Voreinstellungen und überprüfbare Speziallogik haben, erzwingen aber weder Ziele noch Abläufe oder tägliche Eingaben. | Synthese aus D9, D16, D22 und D23; K3-Korrektur |

## Offen (verändern Produkt, Umfang, Kosten oder Datennutzung)

| ID | Thema | Status | Warum es offen bleibt |
| --- | --- | --- | --- |
| O1 | Umsetzung des V1-Umfangs | offen | Der Produktnachweis ist mit D18 ausgewählt und in K2 konzeptionell unter Bedingungen als machbar bewertet. Bedienform, Architektur, ausführbare Prüfung und Implementierung folgen erst in K3–K5. |
| O2 | Bedienkonzept | offen | Die früheren A/B/C sind verworfen. Die K3-Leitform wurde um die verbindliche Bediengrammatik und optionale Werkzeuge korrigiert; V17 bleibt die fachliche Empfehlung. Vor K4 braucht es Dennis' Auswahl oder weitere konkrete Korrektur. |
| O3 | Technik-Stack und Speicherung | offen | K4. Offline, lokale vs. externe Speicherung unklar. |
| O4 | KI ja/nein, Anbieter, Daten, Budget | offen | 50 Euro ausgeschlossen; alles darunter und lokale KI unklar. |
| O5 | Spätere Assistentenbefugnisse | offen | Für V1 gilt D19. Ob spätere Versionen in gesetzten Rahmen selbst planen oder externe Aktionen ausführen dürfen, bleibt eine getrennte Entscheidung. |
| O6 | Langfristige Konfigurierbarkeit | offen | Für V1 sind Sammlung, fünf Eigenschaftsformen, Beziehung und konkrete Sicht gewählt. Welche weiteren Bausteine oder Abläufe langfristig ohne Code veränderbar werden, bleibt offen. |
| O7 | Diät und Training | offen | Eine Diätanalyse ist als mögliches optionales Werkzeug gewünscht. Welche Daten, Auswertungen, fachlichen Grenzen und welcher Umsetzungszeitpunkt sinnvoll sind, ist noch nicht entschieden. |
| O8 | Abendrückblick als Gewohnheit | offen | Frage 3: wie verbindlich die tägliche Praxis sein soll. Getrennt von D13 (Auslassen löscht nichts). |
| O9 | Smarter Kalender und Terminquellen | offen | Ein smarter Kalender ist als mögliches optionales Werkzeug gewünscht. Funktionsumfang, echte Quellen oder Integrationen und Umsetzungszeitpunkt bleiben offen; im V1-Nachweis sind Termine lokal angelegt. |
| O10 | Form beruflicher Entwicklung | offen | Frage 5: welche Formen und Zeitpunkte möglich sein sollen; nicht nur tägliche Aufgaben oder Warten auf einen Anlass. |
| O11 | Erweiterung des Freiheits-Nachweises | offen | K2 begründet keine zusätzlichen Eigenschaftsformen für V1, verlangt aber D20. Weitergehende Formen und Beziehungen bleiben eine spätere Entscheidung. |

Änderungen an diesem Dokument kennzeichnen, ob Dennis zugestimmt hat oder ob nur ein Vorschlag nachgetragen wurde.
