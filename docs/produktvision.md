# Produktvision

Stand: 9. September 2026 · aktuelle Leitplanke nach der Korrektur von Dennis

Dieses Dokument ist die aktuelle inhaltliche Nordrichtung des Projekts. Die ausführliche Arbeitsnotiz unter `docs/context/` bleibt Quelle des Gesprächsverlaufs. Bei einem Widerspruch haben aktuelle direkte Aussagen von Dennis, `docs/entscheidungen.md` und dieses Dokument Vorrang.

## Kernsatz

Momentum soll ein **hochgradig hilfreicher, persönlicher Assistent für Dennis' Alltag und Ziele** werden: eine fokussierte Windows-App, die relevante Informationen zusammenführt, Lage und Möglichkeiten versteht, Arbeit abnimmt, begründete Vorschläge macht und aus Korrekturen sowie beobachteten Ergebnissen lernt.

Die App ist nicht primär eine Oberfläche zum Pflegen von Aufgaben, Statusfeldern und Messwerten. Diese Daten können notwendig sein, sind aber Mittel für Unterstützung, Orientierung und Lernen — nicht der eigentliche Produktnutzen.

## Das gewünschte Gefühl

Dennis soll beim Öffnen nicht denken: „Welche Felder muss ich heute ausfüllen?“, sondern etwa:

- Die App hat meine aktuelle Lage bereits sinnvoll vorbereitet.
- Sie zeigt mir, was heute wirklich Bedeutung hat und warum.
- Sie erkennt Konflikte, Lücken und sinnvolle Möglichkeiten, bevor ich alles selbst zusammensuchen muss.
- Sie fragt nur dort nach, wo meine Antwort die Beurteilung tatsächlich verändert.
- Ich kann ihre Annahmen verstehen, korrigieren und ihre Vorschläge übernehmen oder ablehnen.
- Meine kurze Mitwirkung verbessert spätere Unterstützung spürbar.
- Ich kann eigene Ziele, Strukturen und Arbeitsweisen abbilden, ohne dass sie im Programmcode vorgesehen sein müssen.

Die Nutzung darf Freude machen und Tiefe besitzen. „Wenig Pflegeaufwand“ allein wäre ebenso unzureichend wie eine schöne Oberfläche ohne tragfähige Unterstützung.

## Verständlichkeit ist Produktfunktion

Momentum darf nicht nur richtige oder nützliche Ergebnisse liefern. Dennis muss jederzeit ohne Fachwissen nachvollziehen können:

1. **Was weiß Momentum — und was nicht?** Angaben, Beobachtungen, Annahmen und Lücken bleiben unterscheidbar.
2. **Warum ist das gerade relevant?** Ein Vorschlag zeigt den Zusammenhang mit der aktuellen Absicht oder Lage.
3. **Was empfiehlt Momentum?** Empfehlung, sinnvolle Alternative und Unsicherheit werden in einer ruhigen Standardsicht verständlich.
4. **Was würde sich dadurch ändern?** Vor einer wirksamen Änderung ist der betroffene Teil sichtbar; alles andere bleibt erkennbar unberührt.

Details sollen auf Nachfrage erreichbar sein, nicht den Einstieg überladen. Die Oberfläche darf jedoch nie zur Blackbox werden. Aus „Ich verstehe, was hier passiert“ soll das begründete Gefühl entstehen: **„Das bringt mich weiter.“** Dafür muss jede wiederkehrende Eingabe oder Handlung einen sichtbaren späteren Nutzen haben; reine Datenpflege ohne erkennbaren Beitrag ist ein Produktfehler.

## Arbeitsteilung zwischen Dennis und App

| Die App soll möglichst übernehmen | Bei Dennis bleibt |
| --- | --- |
| Informationen aus vorhandenen Quellen und Einträgen zusammenführen | persönliche Bedeutung und Prioritäten festlegen |
| eine aktuelle Lage mit relevanten Unsicherheiten bilden | wichtige Korrekturen und neue Umstände mitteilen |
| Konflikte, offene Fragen und Alternativen erkennen | entscheiden, welche Handlung oder Änderung gilt |
| einen begründeten nächsten Schritt oder Plan vorschlagen | ausdrücklich zustimmen, wenn eine Änderung Befugnis braucht |
| bestehende Daten für Rückblicke und Verläufe wiederverwenden | Beobachtungen beitragen, die nicht anderweitig bekannt sein können |
| Wirkungen und Fehlannahmen im Verlauf prüfen | beurteilen, ob die Unterstützung persönlich hilfreich ist |

Ein Ablauf, bei dem Dennis überwiegend Karten liest, Checkboxen anklickt und Zustände pflegt, verschiebt die Arbeit in die falsche Richtung.

## Vier zusammengehörende Ebenen

### 1. Fokussierte Alltagsoberfläche

Der normale Gebrauch ist klar, ruhig und vorbereitet. Der Tag kann ein wichtiger Einstieg sein, ist aber nicht automatisch die einzige Organisationslogik. Tiefe erscheint im passenden Kontext, ohne den Alltag mit Konfiguration zu überladen.

### 2. Flexible Grundlage

„Notion-Stil“ meint hier **Freiheit der abbildbaren Gedanken und Zusammenhänge**, nicht eine Kopie von Notions Oberfläche oder vollständigem Funktionsumfang.

Als fachliche Übersetzung, die K4 mit einem flexiblen lokalen Datenkern technisch verankert:

- wenige verständliche Grundbausteine wie Text, Zahl mit Einheit, Datum/Zeit, Status, Wiederholung und Beziehung;
- daraus zusammensetzbare Ziele, Vorhaben, Lerngegenstände, Kontakte, Routinen, Beobachtungen, Messgrößen oder neue eigene Arten von Dingen;
- eigene Eigenschaften und Beziehungen, ohne für jeden Lebensbereich Programmcode zu benötigen;
- unterschiedliche passende Sichten auf dieselben Informationen;
- gute Vorlagen, die sofort funktionieren und verändert werden können.

Freiheit darf nicht bedeuten, dass Dennis zuerst ein leeres System bauen und dauerhaft administrieren muss. Leitidee: **Orientierung als Standard, Freiheit auf Nachfrage.**

### Werkzeuge statt fest verdrahteter Lebensbereiche

Ein smarter Kalender oder ein Werkzeug zum Erfassen und Analysieren einer Diät sind gewünschte Beispiele für starke spätere Fähigkeiten — aber keine verpflichtenden Hauptbereiche, die Momentum jedem Nutzerablauf aufzwingt.

Solche Werkzeuge sollen auf derselben flexiblen Grundlage aufsetzen und sich aktivieren, anpassen, verschieben, ausblenden, pausieren oder gar nicht verwenden lassen. Gute Vorlagen und fachlich sinnvolle Voreinstellungen dürfen sofort Nutzen liefern. Innerhalb eines aktivierten Werkzeugs sind überprüfbare Spezialberechnungen ausdrücklich sinnvoll. Weder das Werkzeug selbst noch seine vorgegebenen Ziele, Abläufe oder täglichen Eingaben dürfen jedoch den Rest der App bestimmen.

Der Assistent verwendet Daten eines Werkzeugs nur, wenn es aktiviert ist, die Information für die aktuelle Absicht relevant ist und ihre Bedeutung bestätigt wurde. Kalender und Diätanalyse sind damit mögliche **Werkzeuge im persönlichen System**, nicht das persönliche System selbst. Diese Einordnung ist noch keine Zusage, beide bereits in Version 1 zu bauen.

Der [Machbarkeitscheck K2](machbarkeitscheck-k2.md) ergänzt eine Sicherheitsgrenze: Eine frei benannte Zahl oder ein Datum wird nicht automatisch zur Priorität, Frist oder Aufgabe. Soll eine eigene Eigenschaft Empfehlungen beeinflussen, müssen ihre Bedeutung und ihr Bezug zu einer aktiven Absicht bestätigt sein. Die App darf diese Zuordnung verständlich vorschlagen.

### 3. Assistenzschicht

Der Assistent arbeitet nicht nach einer versteckten Sammlung starrer Lebensregeln. Ein sinnvoller Zyklus ist:

1. relevante Lage und vorhandene Informationen zusammenführen;
2. entscheidende Lücken und Unsicherheiten erkennen;
3. mehrere passende Möglichkeiten bilden und abwägen;
4. einen Vorschlag mit Gründen und Grenzen machen;
5. nur im erlaubten Umfang handeln;
6. Korrekturen, Durchführung und beobachtete Ergebnisse für die nächste Beurteilung nutzen.

Eine feste, überprüfbare Berechnung ist erlaubt und oft notwendig. Nicht akzeptabel ist, Lebensentscheidungen auf „Checkbox A + Checkbox B = immer Aktion C“ zu reduzieren oder diese Starrheit nur mit variierendem Text zu verdecken.

Unabhängig von einer späteren KI-Entscheidung bleiben Faktenherkunft, Zustände, Einheiten, Zeitkonflikte und Schreibbefugnisse transparent prüfbar. Kontextuelle Beurteilung kann darauf aufsetzen, verändert persönliche Daten oder geltende Pläne aber nicht ohne Dennis' konkrete Bestätigung.

### 4. Ehrliche Entwicklung über Zeit

Die App unterscheidet:

- von Dennis angegeben;
- tatsächlich beobachtet;
- von der App vermutet;
- in einem begrenzten Versuch erprobt.

Plan, Durchführung, Beobachtung und Ergebnis bleiben unterscheidbar. Fehlende Daten bedeuten unbekannt. Eine Korrelation ist keine Ursache. Eine plausible Formulierung ist kein Nachweis für eine gute Empfehlung.

## Der Donnerstag neu verstanden

Der synthetische Donnerstag bleibt ein wichtiges Prüfszenario, aber **kein Screen-Bauplan**.

### Morgens

Die App zeigt nicht bloß Uni, Paket, Geschenk, Lernblock und Gym als fünf Karten. Sie liefert eine vorbereitete Einordnung:

- welcher Teil des Tages fest gebunden ist;
- wo der Plan realistisch eng wird;
- was wegen Frist, Ziel oder vorhandener Beobachtung heute Bedeutung hat;
- welchen Lernfokus sie empfiehlt, welche Alternativen bestehen und was sie noch nicht weiß;
- welche eine Rückfrage die Planung tatsächlich verändern würde.

Dennis soll einen Vorschlag verstehen, korrigieren oder freigeben können, ohne die gesamte Planung selbst erneut zu leisten.

### Während des Tages

Neue Informationen oder Änderungen müssen leicht erfassbar sein. Die App ordnet sie nachvollziehbar in den bestehenden Zusammenhang ein. Ein einmaliges Paket braucht keinen Routinebaukasten; ein neues persönliches Thema darf dennoch in einer selbst gewählten Struktur vorkommen.

### Abends

Die App beginnt mit dem, was sie bereits weiß. Sie fasst den Tag vorläufig zusammen und stellt nur gezielte Fragen, deren Antworten für einen späteren Verlauf oder eine Entscheidung nützlich sind. Dennis soll keinen zweiten Tagesbericht ausfüllen.

### Später

Aus ausreichenden, vergleichbaren Informationen können vorsichtige Verläufe, Hypothesen und kleine überprüfbare Versuche entstehen. Die App zeigt Datenbasis, Lücken und Unsicherheit. Sie behauptet weder eine Ursache noch die Wirkung der App selbst.

## Was die verworfenen Entwürfe gezeigt haben

Die in PR #5 gezeigten Konzepte A, B und C wurden von Dennis als falsche Richtung zurückgewiesen. Das Problem war nicht primär Farbe oder Ausarbeitung, sondern ihr gemeinsames Produktbild:

- gleichartige Karten mit Zuständen bilden fast den gesamten Morgen;
- die Hauptinteraktion bleibt Lesen, Anklicken und manuelles Pflegen;
- „Assistenz“ erscheint als schmaler Textkasten innerhalb eines gewöhnlichen Tagesplaners;
- die drei Konzepte unterscheiden vor allem die Anordnung derselben Elemente;
- die Entwicklung reduziert sich auf wenige Balken und eine einzelne Messgröße;
- die sichtbare Arbeit liegt weiterhin überwiegend bei Dennis.

Solche Elemente können später Teil einer Oberfläche sein. Sie dürfen aber nicht erneut mit dem Kernnutzen oder einem vollständigen Bedienkonzept verwechselt werden.

## Bleibende Abnahmekriterien für die Umsetzung

K3 hat diese Kriterien konzeptionell getragen; die ausführbare Umsetzung muss sie weiterhin mindestens zeigen:

1. **Arbeitsübernahme:** Was hat die App vorbereitet, erkannt oder zusammengeführt, das Dennis nicht selbst sortieren musste?
2. **Beurteilungsqualität:** Warum ist der Vorschlag in dieser Lage sinnvoll, welche Alternative wurde abgewogen und was bleibt unbekannt?
3. **Bedeutungsvolle Korrektur:** Wie verändert eine relevante Korrektur die weitere Unterstützung?
4. **Flexible Neuheit:** Wie lässt sich ein vorher nicht fest eingebautes persönliches Thema abbilden und in Alltag sowie Auswertung verwenden?
5. **Tiefe über Zeit:** Wie wird aus mehreren Beobachtungen eine ehrliche, erkundbare Entwicklung statt einer dekorativen Kennzahl?
6. **Fokussierte Bedienung:** Wie bleibt das sofort nutzbar, obwohl die Grundlage anpassbar ist?
7. **Erfreuliche Qualität:** Warum fühlt sich die Nutzung hochwertig und lohnend an statt wie Verwaltungsarbeit?

Ein Wireframe, das nur Karten anders sortiert, beantwortet diese Fragen nicht.

## Bewusst offen

- genaue Bedienmetapher und Navigation;
- Notion-artige Freiheiten über den ausgewählten V1-Nachweis hinaus;
- weitere Grundbausteine, Beziehungen, Sichten oder Vorlagen nach Version 1;
- genaue Umsetzung innerhalb des gewählten Tauri-/Rust-/SQLite-Rahmens;
- KI ja/nein, Anbieter, lokale Möglichkeit, Datennutzung und akzeptiertes Budget unterhalb von 50 Euro;
- Datenquellen und Integrationen;
- Befugnisse über die vorsichtige V1-Grenze hinaus;
- konkrete Tiefe von Studium, Diät/Training und beruflicher Entwicklung.

Der für Version 1 gewählte Mindestumfang steht in [umfang-v1.md](umfang-v1.md). Die verbleibende Offenheit ist kein Auftrag, ein universelles No-Code-System oder eine vollständige Notion-Kopie zu bauen.
