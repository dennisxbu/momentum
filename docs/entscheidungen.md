# Entscheidungen

Vorschlag und Zustimmung bleiben getrennt. Eine Idee aus der Arbeitsnotiz oder aus einem Schritt ist kein Beschluss, solange der Status nicht „beschlossen“ oder **Fakt** ist.

Eine Zusammenführung in Git ist keine automatische Zustimmung zu jeder Produktannahme in den zusammengeführten Dateien.

Legende: **beschlossen** · **Fakt** (von Dennis genannt, keine fertige Umsetzung) · **Vorschlag** · **offen**

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

Die konkrete Umsetzung und der Umfang von Version 1 bleiben offen (O1), solange Dennis V9 nicht ausgewählt hat.

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
| V8 | Früher Machbarkeitscheck | Vorschlag | Separat beauftragbarer kleiner Check zu Datenbedarf und Entscheidungsqualität, bevor Architektur festzementiert wird. Keine festgelegten Ergebnisse, keine KI-Entscheidung. | Korrektur 8.9.2026; [umfang-v1.md](umfang-v1.md) |
| V9 | Umfang Version 1 | Vorschlag | Kleinste Version mit vollem nützlichen Tag plus Verlauf, begrenztem Kontextvorschlag und Qualitätskriterien. Siehe [umfang-v1.md](umfang-v1.md). | Schritt 2 |

## Offen (verändern Produkt, Umfang, Kosten oder Datennutzung)

| ID | Thema | Status | Warum es offen bleibt |
| --- | --- | --- | --- |
| O1 | Umfang Version 1 | offen | Vorschlag in [umfang-v1.md](umfang-v1.md). Noch keine Auswahl durch Dennis. |
| O2 | Bedienkonzept | offen | Schritt 3 vergleicht Konzepte. |
| O3 | Technik-Stack und Speicherung | offen | Schritt 4. Offline, lokale vs. externe Speicherung unklar. |
| O4 | KI ja/nein, Anbieter, Daten, Budget | offen | 50 Euro ausgeschlossen; alles darunter und lokale KI unklar. |
| O5 | Arten von Planungsvorschlägen | offen | Frage 1: bei welchen konkreten Änderungen nur anzeigen bis zur Zustimmung, bei welchen innerhalb eines gesetzten Rahmens eintragen? Keine Alles-oder-nichts-Entscheidung, kein Einstellungsapparat. |
| O6 | Konfigurierbarkeit-Tiefe | offen | Vorlagen und Felder vs. eigene Formeln oder Module. |
| O7 | Diät und Training | offen | Frage 2: welche Hilfe und welche Informationen Dennis beitragen möchte. Nicht auf tägliche Ernährung vs. nur Training reduziert. |
| O8 | Abendrückblick als Gewohnheit | offen | Frage 3: wie verbindlich die tägliche Praxis sein soll. Getrennt von D13 (Auslassen löscht nichts). |
| O9 | Herkunft und Pflege von Terminen | offen | Frage 4: woher Termine vorerst kommen und wie sie gepflegt werden. Keine bestimmte Integration ohne Bedarf. |
| O10 | Form beruflicher Entwicklung | offen | Frage 5: welche Formen und Zeitpunkte möglich sein sollen; nicht nur tägliche Aufgaben oder Warten auf einen Anlass. |

Änderungen an diesem Dokument kennzeichnen, ob Dennis zugestimmt hat oder ob nur ein Vorschlag nachgetragen wurde.
