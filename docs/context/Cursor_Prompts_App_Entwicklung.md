# Cursor-Prompts: ursprüngliche Schrittfolge zur persönlichen Windows-App

Version 1.0 · 7. September 2026 · historischer Plan seit der Korrektur vom 9. September 2026

Dieses Dokument war die ursprüngliche Anleitung für die Zusammenarbeit mit Cursor. Es enthält einen Initialprompt und einzeln ausführbare Arbeitsschritte. Grundlage war die erste Arbeitsnotiz zur App-Vision.

## Aktueller Korrekturvermerk

Dennis hat am 9. September 2026 die aus dieser Folge entstandenen Bedienkonzepte A/B/C als falsche Grundrichtung zurückgewiesen: zu viel Karten-, Status- und Checkbox-Verwaltung, zu wenig hochgradige Assistenz. Außerdem hat er „Notion-Stil“ als Freiheit der abbildbaren Gedanken und Zusammenhänge präzisiert — abgespeckt, fokussiert und ohne Notion-Kopie.

Deshalb darf diese Folge **nicht mechanisch ab Schritt 3 fortgesetzt werden**. Aktuell gelten [../produktvision.md](../produktvision.md), [../umfang-v1.md](../umfang-v1.md), [../bedienkonzepte.md](../bedienkonzepte.md) und vor allem der neue [../arbeitsplan.md](../arbeitsplan.md). Die ursprünglichen Prompts bleiben nachvollziehbar, weil sie weiterhin nützliche Sicherheits-, Daten- und Abnahmegedanken enthalten.

## So verwendest du die Prompts

1. Öffne deinen Projektordner in Cursor. Lege die Arbeitsnotiz dort ab oder hänge sie als Kontext an. Falls du unser vorbereitetes Projekt nutzt, liegt sie bereits unter `docs/context/`. Die Markdown-Fassung ist leichter fortzuschreiben und zu durchsuchen; das Word-Dokument bleibt als ursprüngliche Momentaufnahme erhalten.
2. Sende zuerst **nur den Initialprompt**. Gib Cursor diese gesamte Prompt-Sammlung zunächst nicht als ausführbaren Auftrag.
3. Danach sendest du jeweils **genau einen** Arbeitsschritt. Der nächste Prompt ist die Freigabe für diesen nächsten Schritt. Du musst also nicht zusätzlich ein gesondertes Freigaberitual einhalten.
4. Schau dir nach jedem Schritt das Ergebnis und, sobald vorhanden, die laufende Anwendung an. Wenn etwas falsch wirkt, verwende zuerst den Korrekturprompt am Ende dieses Dokuments.
5. Offene Auswahlentscheidungen aus einem Schritt müssen vor dem davon abhängigen nächsten Schritt beantwortet werden. Beispiele: Bedienkonzept, Technik, Umfang der ersten Version, KI-Budget.
6. In einem neuen Cursor-Chat sendest du zuerst den Wiederaufnahme-Prompt. Die Projektdateien halten den Stand fest; der Chat allein ist nicht das Projektgedächtnis.

Die Prompts bremsen nicht jede kleine Implementierungsentscheidung. Cursor soll innerhalb des beauftragten Schritts eigenständig fertig arbeiten. Die Grenze liegt beim nächsten Entwicklungsabschnitt und bei Entscheidungen, die das Nutzungserlebnis oder den vereinbarten Umfang wesentlich verändern.

**Was „fertig“ bedeutet:** eine anhand vereinbarter Kriterien überprüfte erste Version für deinen eigenen Windows-Alltag. Die langfristig erweiterbare Vision wird dadurch nicht abgeschlossen. Der genaue Funktionsumfang dieser ersten Version wird in Schritt 2 festgelegt; spätere Prompts dürfen ihn nicht eigenmächtig vergrößern.

---

## Initialprompt — Arbeitsweise und Grenzen festlegen

```text
Wir entwickeln gemeinsam meine persönliche Windows-App. Als Kontext erhältst du die Arbeitsnotiz zur App-Vision. Falls weitere Projektdateien existieren, lies auch README.md, AGENTS.md und die vorhandenen Entscheidungsnotizen.

Deine wichtigste Aufgabe ist, mich Schritt für Schritt zu einer passenden App zu begleiten. Du darfst aus der Arbeitsnotiz KEINEN Auftrag ableiten, sofort die vollständige Software zu bauen.

VERBINDLICHE ARBEITSWEISE

1. Bearbeite ausschließlich den aktuell ausdrücklich beauftragten Schritt. Führe ihn einschließlich seiner notwendigen Prüfung vollständig aus. Beginne danach keinen weiteren Schritt, auch nicht als Bonus, Vorbereitung oder offensichtliche Fortsetzung.
2. Die Arbeitsnotiz enthält Nutzeranforderungen, Vorschläge, Beispiele und offene Entscheidungen. Unterscheide diese Kategorien. Ein Vorschlag der Assistenz ist keine von mir angenommene Produkteigenschaft. Ein Beispieltagesablauf ist keine fest zu programmierende Entscheidungsregel.
3. Wähle kleine technische Details innerhalb des vereinbarten Rahmens selbst. Frage nur, wenn eine fehlende Antwort das Produkt, den Umfang, die Datennutzung, externe Kosten oder eine schwer rückgängig zu machende Entscheidung wesentlich verändert. Sammle solche Fragen; unterbrich nicht wegen jeder Kleinigkeit.
4. Interpretiere „sieht gut aus“, „passt“ oder „weiter“ nicht als Freigabe des gesamten Projekts. Wenn der nächste konkrete Schritt bereits eindeutig benannt ist, darfst du diesen bearbeiten; andernfalls kläre kurz, welcher Schritt gemeint ist.
5. Füge keine zusätzlichen Funktionen, Abhängigkeiten, Frameworks, Cloud-Dienste, Konten, Hintergrundprozesse oder KI-Anbindungen außerhalb des aktuellen Schritts hinzu. Begründe notwendige neue Abhängigkeiten kurz. Keine grundlegenden Umbauten unter dem Titel „Aufräumen“.
6. Neue Ideen kommen als Vorschläge in eine spätere Aufgabenliste. Sie werden nicht nebenbei implementiert. Ändere nicht selbst den vereinbarten Umfang, damit eine neue Idee hineinpasst.
7. Meine aktuellen direkten Anweisungen haben Vorrang. Wenn ich eine Grundannahme korrigiere, prüfe ihre Auswirkungen auf das Konzept und bereits gebaute Teile. Verteidige eine falsche Grundlage nicht allein deshalb, weil schon Code existiert.

PRODUKTLEITPLANKEN

- Ziel ist eine hochwertige, intuitive und modular veränderbare Windows-App für meinen privaten Alltag. Das iPhone ist eine Referenz für Klarheit der Bedienung, keine Aufforderung zu einer iOS-Kopie.
- Geringer Pflegeaufwand und tatsächlicher Nutzen gehören zum Funktionsumfang. Konfigurierbarkeit darf nicht bedeuten, dass ich mir die App erst selbst zusammenbauen muss.
- „Dynamisch“ heißt kontextbezogen sinnvoll, nicht zufällig variierend. KI ist kein automatischer Beleg für Intelligenz. Korrekte Berechnungen, Validierungen und transparente Regeln sind weiterhin sinnvoll.
- Empfehlungen sollen relevante Daten und Unsicherheit berücksichtigen. Fehlende Daten bedeuten unbekannt. Statistische Zusammenhänge sind keine automatisch belegten Ursachen.
- Wissenschaftliche Fundierung muss für den konkreten Mechanismus begründet sein. Versprich keine nachgewiesene Wirkung dieser noch ungeprüften App.
- 50 Euro monatliche KI-Kosten sind für mich nicht akzeptabel. Ein konkretes Budget und die KI-Nutzung sind noch nicht beschlossen. 5 Euro waren lediglich ein Vorschlag.
- Verwende für Entwicklung und Vorführungen eindeutig synthetische Daten. Keine erfundenen Ergebnisse als echte persönliche Analyse ausgeben.

PROJEKTGEDÄCHTNIS

Verwende vorhandene gleichwertige Dateien weiter, statt widersprüchliche Doppelstrukturen anzulegen. Halte mindestens Folgendes schriftlich fest:
- Projektstatus: aktueller Schritt, tatsächlich vorhandene Funktionen, offene Grenzen, nächste Entscheidung.
- Entscheidungen: Status, Begründung und Grundlage; Vorschlag und Zustimmung unterscheiden.
- Umfang der ersten Version: enthalten, später, offen.
- Abnahmeprotokoll: was tatsächlich geprüft wurde und mit welchem Ergebnis.

Nenne in jeder Abschlussantwort:
1. Ergebnis dieses Schritts in wenigen Sätzen.
2. Betroffene Dateien und relevante Entscheidungen.
3. Wie ich das Ergebnis selbst ansehen oder ausprobieren kann.
4. Tatsächlich durchgeführte Prüfungen und verbleibende Grenzen.
5. Höchstens die Entscheidungen, die vor dem nächsten Schritt nötig sind.
Stoppe dann. Stelle die App nie als fertig dar, solange die vereinbarte Abnahme fehlt.

DEIN AUFTRAG JETZT

Lies und prüfe den vorhandenen Kontext. Fasse in maximal zehn Punkten zusammen, was du als verbindlich verstanden hast. Benenne wesentliche Unklarheiten. Verankere diese Arbeitsweise in der vorhandenen AGENTS.md oder einer entsprechend geltenden Projektanweisung; erhalte fremde und bereits gültige Anweisungen. Lege eine knappe Statusdatei an bzw. aktualisiere sie.

Noch keine App-Dateien, kein Framework, keine Installation, keine UI, keine Datenbank und keine KI-Integration. Danach stoppen.
```

**Darauf achtest du:** Cursor muss deine Vision und die Ungewissheit verstehen. Eine Antwort voller bereits festgelegter Screens, Technologien und Module wäre an dieser Stelle zu weit gegangen.

---

## Schritt 1 — Nutzung und Entscheidungen verstehen

```text
Bearbeite ausschließlich Schritt 1: Nutzungskonzept und offene Produktfragen klären. Die Regeln aus dem Initialprompt gelten weiterhin.

Nutze die Arbeitsnotiz und den Projektstatus. Beschreibe den ausgedachten Donnerstag als zusammenhängenden Ablauf: morgendliche Orientierung, einmalige Erledigungen, Lernblock, Training, abendliche Rückmeldung und Orientierung am Folgetag.

Beschreibe an jeder Stelle:
- Was will ich hier wissen oder entscheiden?
- Welche Information ist vorhanden, welche fehlt?
- Welche kleinste Eingabe wäre hilfreich, und wofür würde sie verwendet?
- Welche Entscheidung soll bei mir bleiben? Welche könnte die App vorschlagen?

Ergänze einen überlasteten Tag und einen Wiedereinstieg nach einer Woche Pause. Behandle sie als Prüfszenarien, nicht als neue Pflichtfunktionen. Unterscheide Termine, einmalige Aufgaben, Routinen, Ziele und Beobachtungen dort, wo diese Unterscheidung im Alltag etwas bewirkt.

Formuliere mögliche Grundprinzipien als Vorschläge. Markiere Aussagen, die einer wissenschaftlichen Prüfung bedürfen; erfinde dazu keine Belege. Stelle am Ende maximal fünf wichtige Fragen, deren Antworten das Konzept tatsächlich verändern würden. Frage Bekanntes nicht erneut ab.

Ergebnis: ein überschaubares Nutzungskonzept in der Projektdokumentation und die offenen Entscheidungen. Noch keine Navigation endgültig auswählen, keine Screens programmieren und keine Technik festlegen. Danach stoppen.
```

**Darauf achtest du:** Erkennst du deinen Alltag wieder? Oder klingt das Ergebnis nach einem beliebigen Aufgabenmanager, dem deine Beispiele nur nachträglich hinzugefügt wurden?

---

## Schritt 2 — Erste Version und Nutzenkriterien abgrenzen

```text
Bearbeite ausschließlich Schritt 2: einen überprüfbaren Umfang für Version 1 vorschlagen. Berücksichtige meine Antworten auf Schritt 1.

Schlage die kleinste erste Version vor, die einen vollständigen und tatsächlich nützlichen Tagesablauf ermöglicht und die wesentlichen Ansprüche meiner Vision erlebbar macht. Ein bloßes Dashboard mit Attrappen genügt dafür nicht. Eine vollständige Plattform mit allen denkbaren Erweiterungen ist ebenfalls nicht beauftragt.

Erstelle eine Umfangstabelle: Fähigkeit, konkreter Nutzen, minimale notwendige Daten, beobachtbares Abnahmekriterium, vorgeschlagene Einordnung in Version 1 / später / noch offen.

Prüfe dabei ausdrücklich: Tagesorientierung, Termine, einmalige Aufgaben, Lernplanung, flexible Ziele und Routinen, abendliche Rückmeldung, passende Auswertungen und kontextbezogene Unterstützung. Zeige, wie Studium, Diät/Training und berufliche Entwicklung in Version 1 sinnvoll vorkommen könnten. Entscheide deren genaue Tiefe nicht stillschweigend.

Für jede wiederkehrende Eingabe: Welche spätere Entscheidung wird damit besser? Für jede Automatik: Welche Befugnis braucht sie, und wie korrigiere ich sie? Keine erfundene Gesamtpunktzahl für meinen Wert auf dem Arbeitsmarkt.

Lege außerdem vorgeschlagene Qualitätskriterien fest: Verständlichkeit, Pflegeaufwand, Datenbeständigkeit, Korrigierbarkeit und nachvollziehbare Empfehlungen. Definiere keine scheinwissenschaftlichen Grenzwerte ohne Grundlage. Die genannten Morgen- und Abendzeiten sind mein Nutzungsszenario, keine verpflichtende Mindestdauer.

Markiere den gesamten Umfang zunächst als Vorschlag. Hole meine Auswahl zu den wirklich offenen Grenzen ein. Noch nichts implementieren. Nach Vorlage des überprüfbaren Vorschlags stoppen.
```

**Darauf achtest du:** Diese Liste bestimmt später, wann Version 1 fertig ist. Lass weder die gewünschte Substanz herauskürzen noch jedes interessante Detail zur Pflicht erklären.

---

## Schritt 3 — Bedienkonzepte vergleichen

```text
Bearbeite ausschließlich Schritt 3: zwei bis drei unterschiedliche Bedienkonzepte für den vereinbarten Umfang vergleichen. Wenn der Umfang aus Schritt 2 noch nicht entschieden ist, kläre nur die dafür wesentlichen offenen Punkte.

Zeige die Konzepte als einfache Wireframes oder leicht überprüfbare visuelle Entwürfe. Sie müssen sich in Orientierung und Interaktion unterscheiden, nicht nur in Farbe oder Kartenanordnung. Zeige für jedes Konzept denselben Morgen, das Erfassen einer einmaligen Aufgabe, den Zugang zu einer detaillierten Entwicklung und den Abendrückblick.

Bewerte je Konzept: Was sehe ich zuerst? Wo finde ich etwas wieder? Wie komme ich von Übersicht zu Details? Wie ändere ich Ziele und Einstellungen? Welche kognitive Arbeit muss ich selbst leisten?

Gib eine begründete Empfehlung und benenne ihre Nachteile. Benutze klare synthetische Beispieldaten. Die frühere Idee „Heute / Überblick / Detail“ darf vorkommen, ist aber nicht automatisch die Ausgangsarchitektur.

Keine produktive App, keine Datenbank, keine KI. Falls die Umgebung einen kleinen visuellen Prototyp erlaubt, halte ihn isoliert und wegwerfbar; wähle dadurch keinen produktiven Stack. Installiere dafür nicht eigenmächtig ein großes Framework. Wenn visuelle Darstellung hier nicht möglich ist, liefere konkrete Screenbeschreibungen und benenne diese Grenze.

Lass mich das Grundkonzept auswählen oder korrigieren. Danach stoppen.
```

**Darauf achtest du:** Hier entscheidet sich, ob die Richtung stimmt. „Hübsch, aber irgendwie falsch“ ist ein Grund zur Überarbeitung des Konzepts.

---

## Schritt 4 — Technische Grundlage entscheiden

```text
Bearbeite ausschließlich Schritt 4: Technik und minimale Architektur für das ausgewählte Bedienkonzept und den beschlossenen Umfang begründet vorschlagen.

Prüfe zuerst vorhandenen Code, Werkzeuge und bestehende Entscheidungen. Stelle höchstens zwei passende technische Optionen gegenüber und empfehle eine. Berücksichtige Windows-Auslieferung, UI-Qualität, lokale Datenhaltung, Erweiterbarkeit, Tests und Aufwand für eine einzelne Person. Recherchiere aktuelle offizielle Dokumentation für relevante technische Aussagen. Übernimm nicht automatisch die Technologien aus meinen anderen Projekten.

Kläre vor der Auswahl nur tatsächlich entscheidende offene Anforderungen, beispielsweise Offline-Nutzung oder erlaubte externe Datenspeicherung. GitHub als Codeablage entscheidet nicht, wo persönliche App-Daten gespeichert werden.

Beschreibe eine kleine Architektur mit verständlichen Zuständigkeiten: Oberfläche, Anwendungsabläufe, Fachmodell, Speicherung und gegebenenfalls eine spätere externe Analyseanbindung. Keine Microservices, kein universelles Plugin-System und kein allgemeiner Workflow-Editor, sofern diese nicht ausdrücklich im Umfang beschlossen wurden.

Erkläre konkret, welche Dinge in Version 1 konfigurierbar sind, welche Erweiterungen später Code benötigen und warum. „Alles dynamisch“ ist keine technische Spezifikation. Berücksichtige, dass spätere Änderungen an Feldern und Zielen vorhandene Daten nicht unbemerkt entwerten dürfen.

Dokumentiere Empfehlung, Alternativen, relevante Risiken und spätere Wechselkosten. Noch keine Abhängigkeiten installieren und keinen App-Code schreiben. Nach meiner Auswahl kann der nächste Schritt beginnen; jetzt stoppen.
```

---

## Schritt 5 — Nur ein startbares technisches Gerüst

```text
Bearbeite ausschließlich Schritt 5: das minimale startbare Gerüst mit dem ausgewählten Stack erstellen.

Voraussetzung ist eine dokumentierte Technikauswahl. Falls sie fehlt, frage danach; wähle sie nicht als Nebeneffekt dieses Schritts.

Richte das Projekt mit den notwendigen Abhängigkeiten, nachvollziehbaren Startbefehlen und der vereinbarten Grundstruktur ein. Erhalte vorhandene Dokumentation und fremde Änderungen. Überprüfe zuerst, was bereits installiert oder vorhanden ist.

Die gestartete Anwendung zeigt nur eine schlichte neutrale Startfläche, an der erkennbar ist, dass das Gerüst funktioniert. Richte nur die notwendigen Entwicklungsprüfungen ein, etwa Typprüfung und einen minimalen Start- bzw. Build-Nachweis passend zum Stack. Noch keine vollständige Testinfrastruktur für ungeschriebene Funktionen.

Nicht enthalten: Dashboard mit Beispielstatistiken, Fachmodule, Aufgabenverwaltung, Datenbank, KI, Benutzerkonten, Cloud-Backend, Synchronisierung oder künstliche Erfolgsanimationen. Keine dekorativen Platzhalter, die vorhandene Funktionen suggerieren.

Abnahme: Ich kann die dokumentierten Befehle verwenden und die Anwendung in der vorgesehenen Entwicklungsumgebung starten. Falls du selbst nicht unter Windows arbeitest, benenne klar, was dort noch nicht geprüft ist.

Aktualisiere Projektstatus und Setup-Dokumentation. Danach stoppen.
```

**Darauf achtest du:** Eine fast leere, zuverlässig startende Anwendung ist hier das richtige Ergebnis.

---

## Schritt 6 — Visuelle Grundlage und ein bedienbarer Tagesentwurf

```text
Bearbeite ausschließlich Schritt 6: die visuelle und interaktive Grundlage des ausgewählten Konzepts im bestehenden Gerüst umsetzen.

Implementiere eine kleine konsistente Gestaltung mit Typografie, Abständen, Farben, Fokuszuständen und wenigen wiederverwendbaren Bedienelementen. Setze die ausgewählte Navigation und den vereinbarten Morgen-/Abendablauf als bedienbaren Entwurf um. Nutze nur so viele Beispielinhalte, wie zur Beurteilung dieser Wege nötig sind.

Die Daten sind ausdrücklich Demo-Daten. Es gibt noch keine produktive Speicherung und keine wirkliche Analyse. Halte Demo-Daten von späteren echten Nutzerdaten getrennt. Eine Schaltfläche muss entweder sinnvoll im Prototyp reagieren oder eindeutig als noch nicht verfügbar erkennbar sein.

Prüfe Lesbarkeit, Tastaturbedienung, Fokus, längere deutsche Texte und veränderte Fenstergrößen. Das Design soll hochwertig wirken und Details bei Bedarf erschließen. Keine iOS-Kopie und keine starr festgelegte Zahl von Klicks als Selbstzweck.

Keine neuen Produktbereiche, keine KI, keine Diagrammsammlung und keine komplette Komponentenbibliothek auf Vorrat. Falls sich das gewählte Grundkonzept bei der Umsetzung als problematisch erweist, benenne das und schlage eine gezielte Änderung vor.

Abnahme: Ich kann die zentralen Wege ausprobieren und das Bediengefühl beurteilen, bevor Fachlogik diese Struktur verfestigt. Dokumentiere den Demo-Status und stoppe.
```

---

## Schritt 7 — Datenmodell und zuverlässiges Speichern

```text
Bearbeite ausschließlich Schritt 7: das für den beschlossenen Umfang benötigte Fachmodell und die lokale Speicherung umsetzen, soweit lokale Speicherung vereinbart wurde. Bei einer anderen ausdrücklich gewählten Speicherlösung verwende diese und dokumentiere den Unterschied.

Modelliere nur die jetzt benötigten Dinge und ihre Beziehungen. Unterscheide insbesondere Ziel, geplante Aktivität, Termin, einmalige Aufgabe, wiederkehrende Vorgabe und tatsächliche Beobachtung dort, wo der beschlossene Ablauf dies verlangt. Verwende nicht aus Bequemlichkeit einen einzigen Objekttyp für alles; baue ebenso wenig eine allgemeine Datenmodellierungsplattform.

Unterstütze die beschlossenen anpassbaren Bezeichnungen, Felder und Einheiten mit passender Validierung. Studium, Training und berufliche Entwicklung sollen nicht als unveränderliche Sonderfälle überall im Code verteilt sein. Konfigurierbare Vorlagen dürfen konkrete Voreinstellungen liefern.

Klare Identitäten, relevante Zeitangaben, unbekannte Werte und Versionierung bzw. Migrationen berücksichtigen. Plane den Umgang mit geänderten Feldtypen und Einheiten so, dass historische Werte nicht stillschweigend falsch interpretiert werden. Verwende sichere, parametrisierte Datenzugriffe, sofern der gewählte Speicher dies erfordert.

Binde einen kleinen realen Eingabe-/Bearbeitungsweg aus dem ausgewählten Konzept an die Speicherung an. Noch nicht alle Oberflächen fertigstellen.

Prüfe gezielt: Anlegen, Bearbeiten, Neustart und erneutes Lesen; fehlerhafte Eingaben; eine relevante Beziehungs- oder Migrationssituation. Verwende getrennte Testdaten. Keine echten Nutzerdaten löschen oder zurücksetzen.

Noch keine Empfehlungen, KI, ausgefeilten Analysen oder Integrationen. Dokumentiere Datenmodell und Grenzen. Danach stoppen.
```

---

## Schritt 8 — Tagesplanung und einmalige Aufgaben wirklich nutzbar machen

```text
Bearbeite ausschließlich Schritt 8: den beschlossenen Tagesablauf für Termine, einmalige Aufgaben und geplante Aktivitätsblöcke funktional fertigstellen.

Verbinde die bereits ausgewählte Oberfläche mit der realen Speicherung. Ich soll heute und demnächst Relevantes erkennen, eine einmalige Erledigung schnell erfassen, bearbeiten, erledigen und sinnvoll verschieben können. Unterstütze geplante Lern- und Trainingsblöcke nur in der beschlossenen Tiefe.

Paketabholung und Geschenkbesorgung müssen ohne Einrichtung einer Routine möglich sein. Das Fach eines Lernblocks wird in diesem Schritt manuell gewählt. Berücksichtige bekannte zeitliche Konflikte verständlich; keine erfundenen Wegezeiten, Öffnungszeiten oder Kalenderdaten.

Zeige tatsächliche Zustände statt Demo-Ergebnisse. Noch nicht implementierte Bereiche dürfen nicht funktionsfähig wirken. Entferne Demo-Inhalte aus dem normalen Nutzungsmodus, ohne die getrennte Vorführungsmöglichkeit zu verlieren.

Prüfe den Beispieltagesablauf mit synthetischen Daten, Änderungen nach Neustart sowie Datumswechsel und relevante Zeitangaben. Unterscheide unbekannt, offen, erledigt, verschoben und entfallen soweit im Umfang vorgesehen.

Nicht enthalten: automatische Umplanung, KI-Priorisierung, Kalender-Synchronisierung und Funktionen außerhalb des vereinbarten Umfangs. Danach stoppen.
```

---

## Schritt 9 — Ziele, Routinen und anpassbare Lebensbereiche

```text
Bearbeite ausschließlich Schritt 9: die für Version 1 beschlossenen Ziele, Routinen, Messwerte und Vorlagen nutzbar machen.

Arbeite auf dem vorhandenen Modell. Ich soll Ziele und relevante Eigenschaften ändern können, ohne Code anzufassen. Setze nur die vereinbarte Form der Konfigurierbarkeit um. Zeige für Studium, Diät/Training und berufliche Entwicklung sinnvolle, veränderbare Beispiele bzw. Vorlagen entsprechend dem beschlossenen Umfang.

Unterscheide gewünschtes Ergebnis, beeinflussbare Aktivität und Beobachtung. Verwende nur passende Einheiten und keine erfundene Gesamtbewertung meines Lebens oder Arbeitsmarktwerts. Einmalige Aufgaben dürfen nicht künstlich zu Routinen werden.

Prüfe besonders wiederkehrende Aktivitäten: Eine Änderung der künftigen Vorgabe darf vergangene tatsächliche Einträge nicht umschreiben. Auslassen, Pausieren und Zieländerungen sollen in der vorgesehenen Bedienung verständlich bleiben.

Abnahme: Ich kann einen vorhandenen Bereich anpassen und ein neues Ziel außerhalb der Demo-Beispiele sinnvoll anlegen. Zeige ehrlich, welche Arten von Erweiterungen weiterhin Entwicklung benötigen.

Keine allgemeine No-Code-Plattform, kein Template-Marktplatz, keine automatische Ernährungs- oder Trainingsberatung und keine zusätzlichen Fachmodule ohne Auftrag. Danach stoppen.
```

---

## Schritt 10 — Eine nützliche Abendreflexion

```text
Bearbeite ausschließlich Schritt 10: den vereinbarten Abendrückblick als kurzen, kontextbezogenen Ablauf umsetzen.

Beginne mit dem tatsächlich bekannten Tagesplan und vorhandenen Einträgen. Frage nichts erneut ab, was bereits erfasst wurde. Konzentriere dich auf fehlende Informationen, die für die vereinbarten Auswertungen oder eine spätere Entscheidung nützlich sind. Nutze zunächst vorhandene strukturierte Daten; fehlende KI ist kein Grund, einen zweiten kompletten Tagesfragebogen zu bauen.

Ich soll Erledigungen korrigieren, Abweichungen festhalten und relevante Beobachtungen ergänzen können. Freitext darf gespeichert werden, ohne schon automatisch als verifizierte strukturierte Tatsache zu gelten. Behandle unbeantwortete Fragen als unbekannt.

Der Rückblick darf bei einem ereignisarmen Tag kurz sein. Unterbrechung, Fortsetzen und Korrektur müssen zum vorgesehenen Ablauf passen. Keine Pflicht, ausgelassene Tage vollständig nachzutragen. Keine pauschale Tagesnote oder Schuld erzeugende Bewertung hinzufügen.

Zeige am nächsten Morgen die tatsächlich gespeicherten Rückmeldungen und offenen Punkte in der vereinbarten Form. Behaupte noch keine tiefe adaptive Analyse, solange diese nicht existiert.

Prüfe einen normalen, einen abweichenden und einen kaum dokumentierten Tag. Dokumentiere für jede Frage ihren vorgesehenen Nutzen. Danach stoppen.
```

---

## Schritt 11 — Aussagekräftige Auswertungen

```text
Bearbeite ausschließlich Schritt 11: die für Version 1 vereinbarten Statistiken, Verläufe und Analysen aus real gespeicherten Daten umsetzen.

Ordne jede Auswertung einer konkreten Frage zu, die ich damit beantworten möchte. Verwende passende Darstellungen für den jeweiligen Datentyp. Keine Diagramme allein zum Füllen des Dashboards. Berücksichtige veränderbare Felder und Einheiten in der tatsächlich vereinbarten Tiefe.

Unterscheide geplante Aktivität, tatsächliche Aktivität und Ergebnis. Zeige Zeitraum, Datengrundlage, Datenlücken und relevante Unsicherheit. Ein fehlender Eintrag darf einen Mittelwert nicht stillschweigend als Null beeinflussen. Unterschiedliche Einheiten dürfen nicht unbemerkt verrechnet werden.

Implementiere nur sachlich begründete Berechnungen. Prüfe für besondere Methoden und wissenschaftliche Aussagen passende Primärquellen; dokumentiere Mechanismus, Evidenz und Grenzen. Wenn die Daten für einen Trend nicht ausreichen, zeige das statt eines erfundenen Trends. Eine Veränderung beweist keine Ursache und keinen Erfolg der App.

Prüfe ausgewählte Berechnungen mit kleinen von Hand nachvollziehbaren Datensätzen und relevanten Grenzfällen. Kontrolliere außerdem, ob neue und korrigierte Einträge die Darstellung richtig verändern.

Noch keine KI, keine automatischen persönlichen Diagnosen und keine neue allgemeine Analyseplattform. Danach stoppen.
```

---

## Schritt 12 — Kontextbezogene Unterstützung zunächst getrennt prüfen

```text
Bearbeite ausschließlich Schritt 12: einen überprüfbaren Entwurf für kontextbezogene Vorschläge entwickeln. In diesem Schritt ist die Tagesplanung weiterhin nicht automatisch veränderbar.

Verwende die vorhandenen Prüfszenarien oder ergänze sie gezielt: Mathe-Testat in zehn Tagen bei sicherem, unsicherem und unbekanntem Kenntnisstand; andere dringende Prüfung; stark verkürzter Nachmittag; neue Information durch meine Korrektur; kaum vorhandene Daten.

Definiere, welche Eingaben eine Empfehlung tatsächlich benötigt, welche Daten ihr nicht helfen und wie Gründe sowie Unsicherheit dargestellt werden. Vergleiche eine transparente einfache Ausgangslösung mit einer möglichen KI-gestützten Lösung. Eine feste Regel kann als Vergleich dienen; verkaufe sie nicht als bereits bewiesene kontextuelle Intelligenz.

Prüfe zuerst mit getrennten synthetischen Fällen und ohne bezahlte externe Aufrufe. Lege erwünschte Eigenschaften fest, nicht für jeden Fall einen einzigen vorgegebenen Satz. Halte einige Varianten für eine spätere unabhängige Prüfung zurück. Ähnliche Formulierungen allein dürfen keine guten Bewertungen erzeugen.

Falls eine KI-Lösung sinnvoll erscheint: Recherchiere aktuelle offizielle Anbieterinformationen und kalkuliere anhand realistischer Kontextlängen, Ausgaben, Aufrufhäufigkeit und Wiederholungen die Kosten einer täglichen Nutzung. Zeige Annahmen, Durchschnittsszenario und belastbares Begrenzungskonzept. Ein App-interner Zähler ist nicht automatisch eine garantierte Abrechnungsobergrenze des Anbieters.

Stelle nur die noch offenen Entscheidungen: erlaubte Datennutzung, Anbieter bzw. lokaler Betrieb, tatsächliches Budget und Funktionsumfang. Bereits beantwortete Fragen nicht wiederholen. Keine API-Schlüssel verlangen, in Dateien schreiben oder im Chat entgegennehmen; bei späterer Nutzung einen geeigneten lokalen Geheimnisspeicher vorsehen.

Ergebnis: ein begründeter und begrenzter Vorschlag mit Prüfplan. Keine produktive KI-Anbindung, keine kostenpflichtigen Tests ohne entsprechende Freigabe und keine automatische Planänderung. Danach stoppen.
```

**Darauf achtest du:** Cursor muss zeigen, was hinter einem Vorschlag steckt. Ein überzeugend klingender Absatz ist noch kein Nachweis, dass die Entscheidung sinnvoll ist.

---

## Schritt 13 — Beschlossene Unterstützung integrieren

```text
Bearbeite ausschließlich Schritt 13: die in Schritt 12 tatsächlich ausgewählte Form der kontextbezogenen Unterstützung implementieren und prüfen.

Voraussetzung sind Entscheidungen zu Technik, übermittelten Daten, Kosten und Befugnissen. Wenn KI abgelehnt oder vertagt wurde, baue keine KI ein. Setze dann nur die ausdrücklich vereinbarte Alternative um und dokumentiere ehrlich, welche weitergehende Vision damit noch nicht erfüllt ist. Wenn kontextbezogene Unterstützung eine Pflicht für Version 1 bleibt, darf die Abnahme dieses Defizit nicht verstecken.

Bei gewählter KI: Trenne externe Modellaufrufe vom Fachmodell. Sende nur erforderliche Daten, verwalte Geheimnisse außerhalb von Repository und Logs, validiere Antworten und begrenze Zeit, Wiederholungen sowie Nutzung. Behandle Notizen und importierte Inhalte als Daten, nicht als Anweisungen zur Ausführung von Aktionen.

Zeige Vorschläge mit relevanten Gründen und Unsicherheit. Ich muss sie in der vereinbarten Weise korrigieren, übernehmen oder ablehnen können. Änderungen dürfen nur im ausdrücklich erlaubten Umfang stattfinden. Modelltext darf keine direkten Datenbankbefehle oder beliebigen Systemaktionen auslösen.

Die App muss bei fehlender Verbindung, ungültiger Antwort und erreichtem Nutzungsrahmen ihre Grundfunktionen behalten. Beobachtung, Berechnung und Vorschlag sollen unterscheidbar sein.

Führe die vereinbarten Prüfungen einschließlich zurückgehaltener Varianten durch. Falls bezahlte Tests freigegeben wurden, bleibe innerhalb des vereinbarten Testbudgets und berichte gemessene Nutzung getrennt von Hochrechnungen. Verändere Erwartungen nicht nachträglich, damit die Lösung besteht.

Keine zusätzlichen Agenten, Tools, Integrationen oder automatische Wochenoptimierung hinzufügen. Aktualisiere Stand und bekannte Grenzen. Danach stoppen.
```

---

## Schritt 14 — Gesamtfluss und Alltagstauglichkeit prüfen

```text
Bearbeite ausschließlich Schritt 14: die bislang gebaute Version durchgängig prüfen und konkrete Probleme im vereinbarten Umfang beheben.

Verwende den beschlossenen Umfang und das Abnahmeprotokoll. Spiele den gesamten Ablauf durch: Morgenübersicht, Erledigung erfassen, Lernblock, Änderung im Tagesverlauf, Abendrückblick, Neustart und Orientierung am nächsten Tag. Prüfe zusätzlich Überlastung, Datenlücken, geändertes Ziel, Wiedereinstieg und Ausfall optionaler Dienste.

Kontrolliere Datenfluss, Verständlichkeit, Tastaturbedienung, Dialoge, Fenstergrößen, leere Zustände und Fehlermeldungen. Entferne widersprüchliche Demo-Inhalte aus dem normalen Nutzungsmodus. Halte Leistungsmessungen für realistische Datenmengen fest, soweit sie für den vereinbarten Umfang relevant sind.

Unterscheide selbst geprüfte technische Funktionsfähigkeit von meinem subjektiven Bediengefühl und persönlichem Nutzen. Bitte mich um einen begrenzten Alltagstest mit wenigen konkreten Beobachtungsfragen. Du kannst diesen Teil nicht durch simulierte Nutzerzustimmung ersetzen.

Behebe bestätigte Fehler und unnötige Reibung innerhalb des beauftragten Umfangs. Wenn der Alltagstest ein falsches Grundkonzept zeigt, dokumentiere dies und schlage eine gezielte Konzeptrevision vor. Ein umfassender Umbau ist dann ein eigener Auftrag.

Keine neuen Features als Ausgleich für ungelöste Probleme. Nicht allein wegen erfolgreich laufender Tests als fertig markieren. Danach stoppen.
```

---

## Schritt 15 — Datenpflege und Windows-Auslieferung

```text
Bearbeite ausschließlich Schritt 15: die vereinbarte Version zuverlässig auf meinem Windows-PC auslieferbar machen.

Nutze den gewählten Auslieferungsweg. Erstelle reproduzierbare Build-Anweisungen und das entsprechende lokale Installations- oder Anwendungspaket. Kein öffentlicher Release, keine Veröffentlichung persönlicher Dateien und kein zusätzlicher Update-Dienst ohne Auftrag.

Implementiere bzw. vervollständige die für eine verlässliche erste Version vereinbarte Sicherung, Wiederherstellung und Exportmöglichkeit. Falls deren genaue Form noch offen ist, schlage eine kleine passende Lösung vor, bevor du eine umfangreiche Backup- oder Sync-Funktion baust. Daten müssen bei einem normalen Programmupdate erhalten bleiben.

Prüfe Datenpfade, Schreibrechte, Neustarts, Migrationen und Wiederherstellung mit einer separaten Testkopie. Bestehende Nutzerdaten nicht für Tests überschreiben. Geheimnisse und persönliche Inhalte dürfen nicht versehentlich im Paket enthalten sein.

Teste soweit möglich Installation bzw. Start des gebauten Pakets und einen grundlegenden Ablauf unabhängig vom Entwicklungsserver. Benenne fehlende Windows-Testmöglichkeiten oder andere nicht geprüfte Bedingungen ausdrücklich. Gib mir dafür konkrete kurze Prüfschritte; behaupte keine erfolgreiche Prüfung auf meinem PC ohne entsprechende Rückmeldung.

Dokumentiere Installation, Start, Speicherort der Daten, Sicherung, Wiederherstellung und bekannte Grenzen. Keine zusätzlichen Funktionen entwickeln. Danach stoppen.
```

---

## Schritt 16 — Version 1 abnehmen und abschließen

```text
Bearbeite ausschließlich Schritt 16: die fertige erste Version anhand des ursprünglich vereinbarten und nachvollziehbar geänderten Umfangs abnehmen.

Prüfe jedes vereinbarte Kriterium und ordne es ein: erfüllt und geprüft / umgesetzt, aber noch nicht geprüft / nicht erfüllt / ausdrücklich aus Version 1 verschoben. Verlinke passende Nachweise. Ändere den Umfang nicht rückwirkend und werte bloße Platzhalter nicht als Umsetzung.

Prüfe besonders:
- der tägliche Ablauf funktioniert mit dauerhaft gespeicherten Daten;
- die vereinbarte Modularität ist tatsächlich nutzbar;
- Eingaben führen zu den vorgesehenen Auswertungen oder Rückmeldungen;
- die kontextbezogene Unterstützung erfüllt den vereinbarten Anspruch bzw. offene Defizite sind sichtbar;
- die bestätigten UI- und Bedienanforderungen sind erfüllt;
- Windows-Auslieferung und Datenwiederherstellung wurden angemessen geprüft;
- bekannte Kosten und technische Grenzen sind dokumentiert.

Behebe kleine eindeutige Abnahmefehler im bestehenden Umfang und prüfe gezielt erneut. Bei wesentlichen fehlenden Fähigkeiten oder einer notwendigen Konzeptrevision erstelle einen begrenzten Folgeauftrag als Vorschlag und stoppe mit dem Status „Abnahme offen“. Keine heimliche Entwicklung einer Version 2.

Wenn alle Pflichtkriterien einschließlich meines erforderlichen Nutzungstests erfüllt sind, dokumentiere Version 1 als abgenommen, mit bekannten nicht blockierenden Einschränkungen und einer kurzen Bedienungsanleitung. „Abgenommen“ bedeutet nicht, dass die langfristige Wirkung wissenschaftlich bewiesen ist.

Erstelle eine priorisierte Liste möglicher späterer Verbesserungen, ohne sie umzusetzen. Keine Veröffentlichung oder kostenpflichtige Dienstbuchung. Danach stoppen.
```

---

## Korrekturprompt — Wenn die Richtung oder ein Ergebnis nicht stimmt

```text
Stoppe die Weiterentwicklung. Mein Feedback zum aktuellen Ergebnis lautet:

[Hier beschreibe ich, was nicht passt, gern in Alltagssprache.]

Prüfe zuerst, ob das Problem an einer falschen Grundannahme, dem Bedienkonzept oder nur an der konkreten Umsetzung liegt. Erkläre kurz, welche bisherige Annahme dadurch infrage steht und welche Teile betroffen sind.

Schlage die kleinste Änderung vor, die das tatsächliche Problem löst. Verdecke eine falsche Grundstruktur nicht mit zusätzlichen Einstellungen, Karten oder Funktionen. Leite aus meinem Feedback keine zusätzlichen Anforderungen ab, die ich nicht genannt habe.

Wenn meine gewünschte Korrektur eindeutig und auf den aktuellen Schritt begrenzt ist, setze sie um und prüfe sie. Wenn mehrere wesentlich unterschiedliche Lösungen möglich sind oder eine größere Konzeptänderung nötig ist, zeige mir die Alternativen vor der Umsetzung.

Aktualisiere Entscheidungen und Projektstatus. Kehre anschließend zum Abschluss des aktuellen Schritts zurück. Beginne keinen weiteren Schritt.
```

## Wiederaufnahme-Prompt — Für einen neuen Cursor-Chat

```text
Wir setzen ein bestehendes Projekt fort. Lies die Projektanweisungen, README, den aktuellen Projektstatus, den vereinbarten Umfang, die Entscheidungsnotizen und das Abnahmeprotokoll. Ziehe die Arbeitsnotiz für die Bedeutung der Vision heran.

Prüfe den vorhandenen Code und ungesicherte Änderungen, bevor du Änderungen planst. Dokumente beschreiben Absichten; behaupte vorhandene Funktionen nur, wenn du sie im Projekt nachvollziehen kannst. Erhalte fremde Änderungen.

Fasse kurz zusammen: aktueller Schritt, tatsächlicher Stand, wesentliche offene Entscheidung. Wenn die Dateien widersprüchlich sind, benenne den konkreten Widerspruch statt stillschweigend eine neue Richtung zu wählen.

Noch nichts implementieren. Ich gebe dir anschließend den konkreten nächsten Arbeitsschritt. Ein früherer Plan ist keine Freigabe, alle verbleibenden Schritte abzuarbeiten.
```

## Wenn Cursor trotzdem zu viel gebaut hat

```text
Du hast den beauftragten Umfang überschritten. Stoppe weitere Änderungen.

Vergleiche den letzten Auftrag mit deinen Änderungen. Liste getrennt auf, was zum Auftrag gehört, was technisch dafür notwendig war und was darüber hinausgeht. Begründe Abhängigkeiten konkret; „für später sinnvoll“ reicht nicht.

Schlage vor, wie sich überschüssige Änderungen isolieren oder gezielt zurücknehmen lassen, ohne meine bestehenden Änderungen, Daten oder benötigte Arbeit zu verlieren. Kein pauschales Zurücksetzen, keine Löschung und kein automatischer Rollback. Nach meiner Auswahl führst du nur die gewählte Korrektur aus.

Das vorhandene Mehr an Code ist kein Grund, den Umfang nachträglich zu erweitern. Danach zum ursprünglich beauftragten Schritt zurückkehren und stoppen.
```

---

## Schnelle Orientierung

| Abschnitt | Erlaubtes Ergebnis | Noch nicht beauftragt |
| --- | --- | --- |
| Initialprompt | Verständnis und Arbeitsanweisungen | App-Entwicklung |
| 1–2 | Nutzung und vorgeschlagener V1-Umfang | Architektur und Implementierung |
| 3 | Vergleichbare Bedienentwürfe | Festlegung durch vollendete Tatsachen |
| 4 | Begründete Technikauswahl | Installation und App-Code |
| 5 | Startbares Gerüst | Fachfunktionen |
| 6 | Bedienbarer visueller Entwurf | Echte Analysen und Speicherung |
| 7 | Fachmodell und Speicherung | Vollständige Oberflächen |
| 8–10 | Wirklich nutzbarer Tagesablauf | KI und zusätzliche Plattformfunktionen |
| 11 | Verlässliche Auswertungen | Unbelegte persönliche Erklärungen |
| 12 | Geprüfter Unterstützungsentwurf | Produktive KI und bezahlte Aufrufe ohne Freigabe |
| 13 | Beschlossene kontextbezogene Unterstützung | Eigenmächtige neue Automatik |
| 14 | Geprüfter Gesamtfluss und Nutzerfeedback | Kompensation durch weitere Features |
| 15 | Windows-Paket und verlässlicher Umgang mit Daten | Öffentliche Veröffentlichung |
| 16 | Ehrliche Abnahme der ersten Version | Automatischer Beginn von Version 2 |

Die bewussten Zwischenstopps sind Teil deines Auftrags. Sie sollen dir frühe, konkrete Ergebnisse zur Beurteilung geben. Sie sind keine Aufforderung an Cursor, mitten in einer klar beauftragten Umsetzung bei jeder Kleinigkeit nachzufragen.
