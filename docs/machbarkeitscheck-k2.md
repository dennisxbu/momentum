# K2 — Früher Machbarkeitscheck

Stand: 9. September 2026 · **abgeschlossen**

## Ergebnis in einem Satz

Der in K1 gewählte V1-Kern ist **mit klaren Bedingungen machbar**: verlässliche Fakten, Zustände, Zeitkonflikte, Einheiten und Befugnisse brauchen eine transparente Prüfung; kontextuelle Abwägung kann darauf aufsetzen, darf aber weder Tatsachen erfinden noch direkt Daten verändern.

Das größte erkannte Risiko ist nicht die Darstellung, sondern die Verbindung zwischen beliebig benannten persönlichen Informationen und einer verlässlichen Empfehlung. Eine freie Eigenschaft wird deshalb erst dann handlungsrelevant, wenn ihre Bedeutung und ihr Bezug zu einem aktiven Vorhaben bestätigt sind.

## Was hier geprüft wurde

K2 prüft den Produktgedanken vor Architektur und Bedienentwürfen:

- Reagiert die Empfehlung auf relevante Änderungen?
- Bleibt ihr Kern bei belanglosen Änderungen stabil?
- Wird eine entscheidende Informationslücke erkannt?
- Kann ein nicht vorgebautes Thema am selben Beurteilungsprozess teilnehmen?
- Welche Informationen sind dafür mindestens nötig?
- Welche Arbeit würde regelmäßig bei Dennis verbleiben?
- Was lässt sich transparent bestimmen und was braucht kontextuelle Beurteilung?

Die Prüfung ist ein konzeptioneller Trockenlauf mit getrennten synthetischen Varianten und erwartbarem Verhalten. Es wurde keine App implementiert, kein produktives Modell angeschlossen, kein Anbieter verglichen und kein kostenpflichtiger Aufruf durchgeführt. K2 belegt daher keine reale Modellgüte und kein fertiges Nutzungserlebnis. Es begrenzt, was in K3 ehrlich entworfen und später technisch geprüft werden kann.

## Prüfmethode

Ausgangspunkt ist ein einziger synthetischer Grundzustand. Jede Variante verändert möglichst nur eine entscheidende Information. Dadurch wird sichtbar, ob eine Reaktion durch den geänderten Kontext oder nur durch beliebige Textvariation entsteht.

Drei Lösungsrichtungen wurden gegeneinander beurteilt:

1. **Transparente Basis:** bekannte Fakten normalisieren, Fristen und Zeitfenster berechnen, Konflikte erkennen und feste Sicherheitsgrenzen anwenden.
2. **Reine generative Beurteilung:** den gesamten Kontext als Text an ein mögliches Sprachmodell geben und dessen Antwort unmittelbar verwenden.
3. **Begrenzte Kombination:** transparente Fakten- und Berechnungsschicht plus austauschbare kontextuelle Beurteilung; deren Ergebnis muss auf vorhandene Informationen verweisen und durch Befugnis- sowie Konsistenzprüfungen gehen.

Die dritte Richtung ist ein Ergebnis für weitere Konzepte, noch keine Wahl eines Modells, Anbieters oder Technik-Stacks.

## Synthetischer Grundzustand

Der Testtag ist Donnerstag, 10. September 2026. Alle Angaben sind erfunden.

| Information | Bekannter Stand | Bedeutung |
| --- | --- | --- |
| Universität | 09:00–12:30 Uhr, fest | nicht frei verschiebbar |
| Übergang nach der Uni | 60 Minuten für Weg und Essen bestätigt | erster realistischer freier Beginn 13:30 Uhr |
| Mathe-Testat | in zehn Tagen | relevant, aber Kenntnisstand unbekannt |
| Lernabsicht | 90 Minuten nach der Uni | verschiebbar; genauer Fokus offen |
| Paket | Abholung beabsichtigt, Frist unbekannt | keine harte Dringlichkeit ableitbar |
| Geschenk | bis Sonntag gewünscht | wichtig, aber nicht zwingend heute |
| Training | 19:00–20:30 Uhr geplant | persönliche Absicht, keine beobachtete Durchführung |
| Vorhaben | „Arbeitsplatz verbessern“ ist aktiv | von Dennis gesetzter Zusammenhang |
| Stuhl-Kandidat Nord | 420 Euro bei 450 Euro Budget, Eindruck „ungeeignet“, Rückgabefrist Freitag 18:00 Uhr | freie Sammlung; Rückgabeabsicht bestätigt, Ablauf unbekannt |

Die Rückgabefrist allein erzeugt keine Aufgabe. Erst die bestätigte Absicht, den ungeeigneten Kandidaten zurückzugeben, macht sie für die Planung relevant. Unbekannt ist zunächst, ob eine Online-Anmeldung reicht oder ein Weg während einer Öffnungszeit nötig ist.

## Getrennte Prüffälle

### K2-01 — entscheidende Lücke

**Änderung gegenüber dem Grundzustand:** keine; der Rückgabeweg ist weiterhin unbekannt.

**Erwartbares Verhalten:** Die App kann feste Uni, realistischen Beginn des freien Nachmittags und vorhandene Absichten vorbereiten. Sie darf aber keinen endgültigen Nachmittagsplan ausgeben, weil die Rückgabe entweder kaum Zeit oder einen gebundenen Weg verursachen kann. Die zuerst sinnvolle Frage lautet sinngemäß: „Reicht für die Rückgabe eine Online-Anmeldung, oder musst du den Stuhl während einer Öffnungszeit wegbringen?“

Der unbekannte Mathe-Kenntnisstand ist ebenfalls relevant, beeinflusst aber zunächst eher den Inhalt eines bereits gewünschten Lernblocks als die grundsätzliche Machbarkeit des Nachmittags. Deshalb hat die Rückgabefrage die größere unmittelbare Planwirkung.

**Ergebnis:** machbar. Entscheidend ist nicht, ob irgendeine Lücke erkannt wird, sondern ob zwischen mehreren Lücken die mit der größeren Auswirkung ausgewählt wird.

### K2-02 — relevante Antwort ohne zusätzlichen Weg

**Änderung:** Online-Anmeldung bis Freitag 18:00 Uhr reicht; sie dauert ungefähr zehn Minuten.

**Erwartbares Verhalten:** Der Stuhl verliert für Donnerstag seine harte zeitliche Konkurrenz. Der Kernvorschlag schützt den Lernblock nach dem bestätigten Übergang und plant die kurze Anmeldung spätestens Freitag ein. Paket und Geschenk bleiben ohne erfundene Öffnungszeiten flexibel.

**Ergebnis:** machbar. Datum, Dauer und Zeitfenster lassen sich verlässlich prüfen; die genaue Platzierung bleibt eine begründete Empfehlung.

### K2-03 — relevante Antwort mit harter heutiger Bindung

**Änderung:** Rückgabe nur vor Ort, Annahme schließt Donnerstag um 17:30 Uhr, Gesamtweg und Abgabe benötigen 70 Minuten; Freitag ist wegen eines bestätigten Termins nicht möglich.

**Erwartbares Verhalten:** Die Rückgabe wird zum harten Bestandteil des Nachmittags. Der frühere Vorschlag darf nicht nur umformuliert werden. Die App muss beispielsweise Rückgabe vor Lernblock empfehlen und erklären, welcher weiche Bestandteil gekürzt oder verschoben wird. Eine echte Alternative kann den vollen Lernblock schützen und dafür Training oder Geschenk verschieben. Die Wahl bleibt bei Dennis.

**Ergebnis:** machbar. Harte Machbarkeit ist berechenbar; die Abwägung zwischen weichen Absichten braucht persönliche Priorität und kontextuelle Beurteilung.

### K2-04 — belanglose Änderung

**Änderung:** Beim Stuhl wird zusätzlich „Farbe: grün“ gespeichert. Farbe ist weder als Kriterium noch als Priorität beschrieben.

**Erwartbares Verhalten:** Konflikt, Empfehlung und Alternative bleiben gleich. Eine generative Formulierung darf sprachlich leicht variieren, aber nicht so wirken, als sei eine neue Strategie entstanden.

**Ergebnis:** machbar, wenn Empfehlungskern und verwendete Belege strukturiert festgehalten und verglichen werden. Bei einer reinen Textantwort ohne diese Grenze ist Stabilität nicht zuverlässig beurteilbar.

### K2-05 — fachlich relevante Korrektur

**Änderung:** Dennis sagt: „Mathe sitzt schon ziemlich gut; in Datenbanken schreibe ich übermorgen und Joins sind noch unsicher.“

**Erwartbares Verhalten:** Der Lernfokus wechselt begründet zu Datenbanken. Rückgabe, Uni und andere bestätigte Bedingungen bleiben unverändert. Die frühere Vermutung „Mathe braucht heute Vorrang“ wird als überholt kenntlich, nicht aus der Historie gelöscht.

**Ergebnis:** machbar. Die Korrektur muss als neue Nutzerangabe mit Bezug und Zeitpunkt gespeichert werden; ein bloßer Chatverlauf genügt als Projektgedächtnis nicht.

### K2-06 — Tagesänderung mit Überlastung

**Änderung:** Die Universität endet unerwartet erst um 14:00 Uhr. Der bestätigte Übergang verschiebt den ersten freien Beginn auf 15:00 Uhr; K2-03 gilt weiterhin.

**Erwartbares Verhalten:** Die App erkennt, dass Rückgabe, 90 Minuten Lernen, Geschenk, Paket und Training nicht mehr seriös als unverändert machbarer Nachmittag dargestellt werden können. Sie beurteilt nur die betroffene Zeit nach 14:00 Uhr neu, schützt die harte Rückgabe und legt offen, welche weichen Vorhaben nicht mehr gleichzeitig passen.

**Ergebnis:** machbar. Dafür sind Dauer, Zeitfenster, Verschiebbarkeit und bestätigte Bedeutung nötig. Ohne Dauerangaben ist nur eine qualitative Warnung möglich, keine belastbare Ablaufplanung.

### K2-07 — freies Datum ohne bestätigte Bedeutung

**Änderung:** In einer eigenen Sammlung steht ein Datumsfeld „Merken bis: Samstag“, dessen Beschreibung leer ist.

**Erwartbares Verhalten:** Die App darf daraus weder Frist noch Aufgabe erfinden. Sie kann einmalig vorschlagen, die Bedeutung zu klären, wenn der Eintrag mit einem aktiven Vorhaben verbunden ist. Ohne Bestätigung bleibt das Datum informativ und beeinflusst keine Planung.

**Ergebnis:** nur mit semantischer Brücke machbar. Name und Datentyp allein reichen für eine verlässliche Assistenz nicht.

### K2-08 — zurückgehaltener fremder Anwendungsfall

**Änderung:** Statt Stühlen wird eine nicht vorgebaute Sammlung „Rezeptideen“ verwendet. Sie enthält „Dauer: 25 Minuten“, „Zutat verbrauchen bis: heute“, „Aufwand: niedrig“ und eine Beziehung zum bestätigten Vorhaben „Abendessen zuhause“. Dennis bestätigt einmalig, dass „verbrauchen bis“ eine Verderbgrenze und nicht nur eine Erinnerung bezeichnet.

**Erwartbares Verhalten:** Derselbe Mechanismus kann das Rezept als sinnvolle Abendoption erkennen, ohne ein Ernährungs- oder Rezeptmodul zu besitzen. Die App darf daraus keine gesundheitliche Aussage ableiten. Wird die Beziehung zum heutigen Abend entfernt, fällt der Eintrag aus dem Tageskontext.

**Ergebnis:** grundsätzlich machbar. Der Fall benötigt keine zusätzliche Eigenschaftsform, aber dieselbe explizite Bedeutungs- und Beziehungsbrücke.

### K2-09 — Abend ohne vollständige Antwort

**Änderung:** Bestätigt sind Uni-Verlängerung und Rückgabe. Zum Lernen und Training antwortet Dennis abends nicht.

**Erwartbares Verhalten:** Rückgabe und Planänderung können zusammengefasst werden. Lernen und Training bleiben „Durchführung unbekannt“. Die App darf am Folgetag offene Auswirkungen zeigen, aber keinen Misserfolg, keine Serie und keinen Leistungswert berechnen.

**Ergebnis:** machbar. Dafür braucht jeder relevante Zustand eine Herkunft und die Trennung zwischen geplant, bestätigt, beobachtet, vermutet und unbekannt.

### K2-10 — belanglose Formulierungsänderung

**Änderung:** Eine bereits bekannte Beobachtung wird nur anders formuliert, ohne neue Bedeutung oder anderen Zeitpunkt.

**Erwartbares Verhalten:** Sie erzeugt keine zweite Tatsache, keine künstlich erhöhte Evidenz und keinen neuen Plan. Bei unklarer Gleichbedeutung kann die App die mögliche Dublette zeigen, aber nicht selbst beide Beobachtungen als unabhängige Belege zählen.

**Ergebnis:** bedingt machbar. Eindeutige Identitäten und bestätigte Korrekturen sind transparent lösbar; semantisch ähnliche Freitexte brauchen kontextuelle Beurteilung und einen Korrekturweg.

## Vergleich der Lösungsrichtungen

Bewertung: **stark** = für V1 verlässlich erreichbar, **begrenzt** = nur mit engen Voraussetzungen, **schwach** = als alleinige Grundlage nicht vertretbar. Dies ist eine technische Einschätzung aus dem Trockenlauf, kein gemessener Modellbenchmark.

| Fähigkeit | Transparente Basis | Reine generative Beurteilung | Begrenzte Kombination |
| --- | --- | --- | --- |
| Zeitfenster, Fristen und Einheiten | stark | begrenzt; mögliche Rechen- und Auslassungsfehler | stark |
| Unbekannt von „nicht erledigt“ trennen | stark | begrenzt; muss ausdrücklich erzwungen werden | stark |
| Schreibbefugnisse durchsetzen | stark | schwach bei direktem Modellzugriff | stark durch getrennte Bestätigung |
| Belanglose Änderungen stabil behandeln | stark bei bekannten Bedeutungen | schwach bis begrenzt | stark, wenn Belege und Empfehlungskern validiert werden |
| Freie Feldnamen verstehen | schwach ohne manuelle Zuordnung | stark im Sprachverständnis, aber unsicher | stark nach vorgeschlagener und bestätigter Bedeutung |
| Wichtigste von mehreren Lücken wählen | begrenzt; starre Rangfolge droht | grundsätzlich stark, aber nicht garantiert | gut prüfbar anhand unterschiedlicher Handlungsfolgen |
| Persönliche Zielkonflikte abwägen | begrenzt | grundsätzlich stark, aber wechselhaft | am plausibelsten mit Faktenbasis und sichtbaren Gründen |
| Natürliche Synthese statt Kartenliste | schwach bis begrenzt | stark | stark |
| Nachvollziehbarkeit | stark | schwach ohne Belegzwang | ausreichend, wenn jede Behauptung auf Daten verweist |
| Offline und laufende Kosten | stark | abhängig von späterer Wahl | Basiskern stark; kontextuelle Schicht bleibt offen |

### Schlussfolgerung

Eine rein transparente Lösung wäre als zuverlässiger Termin- und Konfliktprüfer machbar, erreicht aber wahrscheinlich nicht das von Dennis gewünschte Assistentengefühl. Sie würde schnell wieder zu sichtbaren Regeln und Verwaltungslogik tendieren.

Eine rein generative Lösung könnte überzeugend klingen und freie Begriffe verstehen, ist aber als alleinige Wahrheits-, Rechen- und Schreibinstanz nicht akzeptabel. Sprachliche Qualität darf nicht über fehlende Daten, wechselnde Prioritäten oder unerlaubte Änderungen hinwegtäuschen.

Für K3 ist deshalb eine **begrenzte Kombination** die tragfähigste Annahme:

1. Eine verlässliche Schicht stellt Fakten, Herkunft, Einheiten, Zeiten, Beziehungen und erlaubte Änderungen bereit.
2. Eine kontextuelle Schicht bildet daraus Optionen, wählt eine entscheidende Frage und formuliert eine Synthese.
3. Jede Empfehlung benennt verwendete Informationen, Annahmen und offene Lücken.
4. Ein getrennter Prüfschritt verwirft ungestützte Behauptungen und unerlaubte Datenänderungen.
5. Erst Dennis' konkrete Antwort oder Bestätigung verändert den geltenden Plan oder persönliche Daten.

Ob die kontextuelle Schicht später ein externes Sprachmodell, ein lokales Modell, begrenzte Heuristiken oder eine Kombination verwendet, bleibt offen. K2 wählt keinen Anbieter und verspricht keine identische Qualität ohne Modell.

## Minimal notwendiger Kontext

### Für jedes planungsrelevante Element

- verständlicher Titel und Art der Aussage;
- Herkunft: Nutzerangabe, Beobachtung, Berechnung oder Vermutung;
- Zustand: vorgeschlagen, bestätigt, beobachtet, überholt oder unbekannt;
- zeitliche Bedeutung: fester Termin, Fenster, Frist oder keine zeitliche Bindung;
- Dauer oder wenigstens die Kennzeichnung „unbekannt“, wenn eine Ablaufplanung erwartet wird;
- Verschiebbarkeit beziehungsweise Härte der Bindung;
- Beziehung zu einem aktiven Vorhaben oder einer bestätigten Absicht;
- Zeitpunkt und Historie wichtiger Korrekturen.

### Zusätzlich für freie Eigenschaften

- Name und Datentyp;
- Einheit bei Zahlen;
- kurze Beschreibung der Bedeutung;
- bestätigte Rolle im konkreten Zusammenhang, falls die Eigenschaft Empfehlungen beeinflussen soll;
- sichtbarer Ursprung einer von der App vorgeschlagenen Interpretation.

Ein Datum wird nicht allein aufgrund seines Typs zur Frist. Eine Zahl wird nicht ohne Einheit und Zweck optimiert. Eine Beziehung macht sichtbar, *wofür* etwas relevant ist, beweist aber noch keine Priorität.

### Nicht dauerhaft erforderlich

Öffnungszeiten, Wege, Energie, Stimmung oder Detailwissen müssen nicht als tägliche Pflichtfelder existieren. Sie können unbekannt bleiben oder genau dann erfragt werden, wenn verschiedene Antworten die aktuelle Empfehlung wesentlich verändern. Spätere Datenquellen dürfen diese Arbeit reduzieren, werden in K2 aber nicht vorausgesetzt.

## Erwartbarer Pflegeaufwand

Der Nachweis bleibt vertretbar, wenn die Arbeit so verteilt wird:

| Moment | Regelmäßige Mitwirkung von Dennis |
| --- | --- |
| Einmalige Einrichtung einer freien Sammlung | Name, Bedeutungsbeschreibung, benötigte Eigenschaften und mindestens eine Beziehung; vorgeschlagene Bedeutungen einmal bestätigen oder korrigieren |
| Normaler Morgen | im Regelfall keine Neuerfassung; höchstens eine Frage mit hoher Planwirkung |
| Tagesänderung | die neue Information einmal mitteilen; keine vollständige Neuplanung |
| Abend | bekannte Änderungen bestätigen oder korrigieren; höchstens wenige Fragen mit erkennbarem späterem Nutzen |
| Strukturänderung | Bedeutung bewusst ändern; Historie bleibt erhalten |

K3 darf keine Bedienrichtung vorschlagen, die jeden Morgen Priorität, Dauer und Zustand aller Einträge neu abfragt. Wenn der lokale Kontext veraltet ist, muss die App das sichtbar sagen und einen vorsichtigen Vorschlag machen, statt den Pflegeaufwand zu verstecken.

## Erreichbare Qualitätsgrenze für Version 1

Mit dem beschriebenen Mindestkontext erscheint für V1 erreichbar:

- einen begrenzten Tag mit festen und beweglichen Teilen realistisch zusammenzufassen;
- harte Konflikte zuverlässig zu erkennen;
- eine kleine Zahl von Optionen mit sichtbaren Gründen zu vergleichen;
- die Informationslücke mit der größten unmittelbaren Planwirkung zu erfragen;
- eine bestätigte freie Struktur in dieselbe Beurteilung einzubeziehen;
- relevante Änderungen begrenzt neu zu beurteilen;
- bekannte Informationen abends und am Folgetag weiterzuverwenden;
- fehlende Daten, Annahmen und Befugnisse ehrlich zu behandeln.

Für V1 nicht seriös erreichbar oder nicht belegt sind:

- umfassendes Verständnis beliebiger persönlicher Daten ohne Bedeutungsbestätigung;
- optimale Lebensplanung über viele konkurrierende Ziele;
- belastbare Kausaldiagnosen aus wenigen Alltagsbeobachtungen;
- psychologische, medizinische, Ernährungs- oder Trainingsberatung allein aus diesem Kontext;
- dauerhaft hochwertige Empfehlungen ohne aktuelle Daten oder kurze Korrekturen;
- sichere autonome Änderungen außerhalb der App;
- ein Qualitätsversprechen für ein noch nicht getestetes Sprachmodell.

Momentum kann in V1 also ein glaubwürdiger **Co-Pilot für einen begrenzten Entscheidungsraum** werden. Ein allwissender oder autonom handelnder Lebensassistent wäre eine falsche Behauptung.

## Begrenzte Folgen für Version 1 und K3

1. Die fünf in K1 gewählten Eigenschaftsformen reichen für den Nachweis; K2 begründet keine zusätzliche Form.
2. Assistenzrelevante freie Eigenschaften benötigen eine kurze Bedeutung und eine bestätigte Verbindung zum aktiven Zusammenhang.
3. Der Grundzustand muss Herkunft, Gewissheit und zeitliche Rolle von Informationen unterscheiden.
4. Empfehlung, Belege, Annahmen, offene Lücken und vorgeschlagene Änderungen müssen als unterscheidbare Inhalte konzipiert werden.
5. Eine kontextuelle Komponente erhält keine direkte Schreibbefugnis.
6. K3 muss zeigen, wie genau eine entscheidende Frage gestellt wird, ohne den Alltag in ein Formular zu verwandeln.
7. K3 muss außerdem die stabile Reaktion auf eine belanglose Änderung und den zurückgehaltenen Fall „Rezeptideen“ sichtbar machen.
8. Eine Anbieter-, Kosten-, Offline- oder endgültige KI-Entscheidung wird nicht vorgezogen.

## K2-Abnahme

| Prüfpunkt | Ergebnis |
| --- | --- |
| Relevante Änderung | bestanden im Trockenlauf; K2-02, K2-03, K2-05 und K2-06 verlangen unterschiedliche begründete Folgen |
| Belanglose Änderung | konzeptionell bestanden mit strukturiertem Empfehlungskern und Belegprüfung; K2-04 und K2-10 |
| Entscheidende Lücke | bestanden; K2-01 zeigt unterschiedliche Handlungszweige |
| Nicht vorgebautes Thema | bedingt bestanden; braucht bestätigte Semantik und Beziehung; K2-07 und K2-08 |
| Datenbedarf | Mindestinformationen und bewusst optionale Angaben abgegrenzt |
| Pflegeaufwand | ohne tägliche Vollabfrage plausibel; im späteren Bedienkonzept noch persönlich zu prüfen |
| Sichere Berechnung versus Beurteilung | Aufgaben getrennt und Grenzen dokumentiert |
| Produktive Modellgüte | nicht geprüft und nicht behauptet |

Gesamturteil: **K2 bestanden mit Bedingungen.** Es gibt keinen konzeptionellen Grund, K3 zu blockieren. K3 muss die festgehaltenen Grenzen sichtbar tragen; tatsächliche Assistenzqualität bleibt später mit ausführbaren und zurückgehaltenen Fällen zu validieren.
