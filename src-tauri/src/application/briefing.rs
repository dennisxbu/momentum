use crate::domain::{ActionChoice, BriefingView, KnownFact, RelevantDeadline, ScenarioState};

pub trait Reasoner: Send + Sync {
    fn prepare(
        &self,
        scenario: &ScenarioState,
        deadline: Option<&RelevantDeadline>,
    ) -> BriefingView;
}

#[derive(Default)]
pub struct LocalReasoner;

impl Reasoner for LocalReasoner {
    fn prepare(&self, state: &ScenarioState, deadline: Option<&RelevantDeadline>) -> BriefingView {
        let Some(deadline) = deadline.cloned() else {
            return prepare_without_relevant_deadline(state);
        };
        let deadline_label = pretty_date(&deadline.date);
        let mut known = vec![
            fact(
                "Feste Bindung",
                if state.uni_extended {
                    "Uni · 09:00–13:30"
                } else {
                    "Uni · 09:00–12:00"
                },
                "bestätigt",
                "lokaler Termin",
            ),
            fact(
                "Vorhaben",
                "Mathe-Testat in 10 Tagen · Kenntnisstand offen",
                "angegeben",
                "synthetischer Prüffall",
            ),
            fact(
                &deadline.collection,
                &format!("{} · Rückgabe bis {deadline_label}", deadline.item),
                "bestätigt",
                &format!(
                    "freie Sammlung · {} · Bezug: {}",
                    deadline.meaning, deadline.related_intention
                ),
            ),
        ];
        if let Some(home) = state.home_after_uni {
            known.push(fact(
                "Rückkehr nach der Uni",
                if home {
                    "Gegen 13:00 zuhause"
                } else {
                    "Danach nicht direkt zuhause"
                },
                "angegeben",
                "deine Antwort",
            ));
        }
        if state.plan_confirmed {
            known.push(fact(
                "Geltender Plan",
                "Rückgabe und geschützter Matheblock",
                "bestätigt",
                "deine Freigabe",
            ));
        }
        if let Some(result) = &state.learning_result {
            known.push(fact(
                "Lernblock",
                if result == "confirmed_done" {
                    "durchgeführt"
                } else {
                    "Durchführung unbekannt"
                },
                if result == "confirmed_done" {
                    "bestätigt"
                } else {
                    "unbekannt"
                },
                if result == "confirmed_done" {
                    "deine Beobachtung"
                } else {
                    "keine Angabe"
                },
            ));
        }

        match state.phase.as_str() {
            "morning_unknown" => BriefingView {
                phase: "morning".into(), day_label: "Donnerstag · Morgen".into(), overline: "Vorbereitetes Morgenbriefing".into(),
                title: "Zwei wichtige Dinge konkurrieren um denselben Nachmittag.".into(),
                lead: "Momentum hat feste Zeit, Testat und deine frei angelegte Rückgabefrist zusammengeführt. Eine einzige Information entscheidet, welcher Ablauf ehrlich ist.".into(),
                known, meaning: format!("Die Rückgabefrist ist relevant, weil ihre Bedeutung bestätigt und {} mit „{}“ verbunden ist. Gleichzeitig braucht das Testat einen echten Lernblock. Ob beides nacheinander passt, hängt an deiner Rückkehr.", deadline.item, deadline.related_intention),
                recommendation: "Noch keinen starren Plan festschreiben.".into(),
                reason: "Ohne deine Rückkehrzeit wäre jede genaue Reihenfolge erfunden. Momentum hält deshalb zwei tragfähige Wege bereit, statt Sicherheit vorzutäuschen.".into(),
                alternative: "Mathe fest schützen, Rückgabe separat klären".into(),
                alternative_cost: "Schützt die Prüfungsvorbereitung, lässt aber die bestätigte Rückgabeabsicht unter Zeitdruck.".into(),
                unknowns: vec!["Ob du nach der Uni direkt zuhause bist".into()], changed: vec![], unchanged: vec!["Uni bleibt feste Bindung".into(), "Kenntnisstand in Mathe bleibt unbekannt".into()],
                question: Some("Bist du nach der Uni gegen 13 Uhr zuhause?".into()),
                actions: vec![action("home_by_13", "Ja, gegen 13 Uhr", "primary", None), action("away_after_uni", "Nein, erst später", "secondary", None)],
                progress: 18, status_note: "Eine Antwort fehlt für den belastbaren Vorschlag".into(),
            },
            "morning_answered" => {
                let home = state.home_after_uni.unwrap_or(false);
                BriefingView {
                    phase: "morning".into(), day_label: "Donnerstag · Morgen".into(), overline: "Lücke geklärt".into(),
                    title: if home { "Der Nachmittag hat jetzt eine klare, realistische Reihenfolge." } else { "Der Engpass liegt vor der Uni – nicht im Lernblock." }.into(),
                    lead: if home { "Deine Rückkehr macht Rückgabe und Lernen nacheinander möglich. Momentum schlägt nur die betroffenen Teile vor." } else { "Weil du später nicht zuhause bist, kann Momentum die Rückgabe nicht seriös in den Nachmittag legen." }.into(),
                    known,
                    meaning: if home { "Die bestätigte Rückgabefrist wird zuerst bedient; danach bleibt ein zusammenhängender Matheblock. Paket, Geschenk und Training bleiben nachgeordnet." } else { "Die Rückgabe braucht vor der Uni eine eigene Klärung. Der geplante Lernblock wird nicht für eine unrealistische Nachmittagsroute geopfert." }.into(),
                    recommendation: if home { "Nach der Uni zuerst Rückgabe, danach 90 Minuten Mathe." } else { "Vor der Uni Rückgabe klären, nachmittags Mathe schützen." }.into(),
                    reason: if home { "So verliert die nahe, bestätigte Frist ihren Druck, ohne den Lernfortschritt nur als Restzeit zu behandeln." } else { "Das trennt den zeitkritischen Vorgang vom Fokusblock und vermeidet eine Planung auf unbekannter Verfügbarkeit." }.into(),
                    alternative: if home { "Mathe direkt nach der Uni, Rückgabe am späten Nachmittag" } else { "Rückgabechance heute offenlassen" }.into(),
                    alternative_cost: if home { "Mehr Fokus direkt nach der Uni, aber weniger Puffer für die Rückgabe." } else { "Einfacher Tagesablauf, aber reales Fristrisiko." }.into(),
                    unknowns: vec!["Mathe-Kenntnisstand bleibt offen; er ändert heute nur das Lernmaterial, nicht den geschützten Block.".into()],
                    changed: if state.irrelevant_color.is_some() { vec!["Stuhlfarbe als unbestätigte Zusatzangabe gespeichert – ohne Planwirkung".into()] } else { vec!["Rückkehrzeit in die Lage übernommen".into()] },
                    unchanged: vec!["Uni-Zeit bleibt fest".into(), "Kernvorschlag bleibt trotz belangloser Zusatzangaben stabil".into()],
                    question: Some("Soll genau dieser Ablauf gelten?".into()),
                    actions: vec![
                        action("confirm_plan", "Ablauf übernehmen", "primary", Some(vec![if home { "Rückgabe nach der Uni zuerst" } else { "Rückgabe vor der Uni klären" }, "90 Minuten Mathe als geschützter Block"])),
                        action("irrelevant_color", "Nebeninfo: Stuhl ist salbeigrün", "quiet", None),
                    ], progress: 32, status_note: "Vorschlag vorbereitet · noch keine Planänderung".into(),
                }
            },
            "plan_confirmed" => BriefingView {
                phase: "changed".into(), day_label: "Donnerstag · im Verlauf".into(), overline: "Plan gilt".into(),
                title: "Der Plan steht – Momentum wartet nur auf echte Änderungen.".into(),
                lead: "Die Entscheidung ist nachvollziehbar gespeichert. Zusätzliche Informationen verändern den Ablauf nur, wenn ihre Bedeutung das wirklich rechtfertigt.".into(),
                known, meaning: "Rückgabe und Mathe haben jetzt einen bestätigten Platz. Paket, Geschenk und Training bleiben sichtbar, aber sie konkurrieren nicht automatisch um denselben Fokus.".into(),
                recommendation: "Den bestätigten Ablauf beibehalten.".into(), reason: "Seit deiner Freigabe ist keine planungsrelevante Änderung bekannt. Stabilität ist hier hilfreicher als ständig neue Vorschläge.".into(),
                alternative: "Bei einer echten Verzögerung nur den betroffenen Abschnitt neu planen".into(), alternative_cost: "Erfordert eine kurze Korrektur, bewahrt dafür den Rest des Tages.".into(), unknowns: vec![],
                changed: if state.irrelevant_color.is_some() { vec!["Salbeigrün als Nebeninformation aufgenommen – Vorschlag unverändert".into()] } else { vec!["Morgenvorschlag ist jetzt geltender Plan".into()] },
                unchanged: vec!["Reihenfolge und Matheblock".into(), "Unbekannter Kenntnisstand wird nicht erfunden".into()],
                question: Some("Gab es eine relevante Änderung – oder ist der Tag bereit für den Rückblick?".into()),
                actions: vec![action("uni_extended", "Uni dauert 90 Minuten länger", "secondary", None), action("to_evening", "Zum Abendrückblick", "primary", None), action("irrelevant_color", "Stuhlfarbe ergänzen", "quiet", None)],
                progress: 53, status_note: "Bestätigt · auf relevante Änderung vorbereitet".into(),
            },
            "day_changed" => BriefingView {
                phase: "changed".into(), day_label: "Donnerstag · Änderung".into(), overline: "Begrenzte Neuplanung".into(),
                title: "90 Minuten fehlen – aber nicht der ganze Plan ist kaputt.".into(), lead: "Momentum hat die verlängerte Uni als Korrektur übernommen und nur den dadurch betroffenen Nachmittag neu bewertet.".into(),
                known, meaning: "Rückgabe und Mathe passen nicht mehr mit demselben Puffer. Die Rückgabefrist bleibt heute näher als die Testatfrist; ein kürzerer Lernstart schützt trotzdem Kontinuität.".into(),
                recommendation: "Rückgabe direkt erledigen, Mathe heute auf 45 Minuten fokussieren.".into(), reason: "Das erhält die bestätigte Rückgabeabsicht und verhindert, dass Lernen wegen einer einzigen Störung vollständig ausfällt.".into(),
                alternative: "90 Minuten Mathe halten, Training auslassen".into(), alternative_cost: "Mehr Lernzeit, aber höhere Belastung am Abend und keine bewusste Erholung.".into(), unknowns: vec!["Tatsächliche Energie am Abend bleibt unbekannt".into()],
                changed: vec!["Uni endet 13:30 statt 12:00".into(), "Matheblock wird als Vorschlag auf 45 Minuten verkürzt".into()], unchanged: vec!["Rückgabe bleibt erster Schritt".into(), "Paket und Geschenk bleiben nachgeordnet".into()],
                question: Some("Soll nur diese Anpassung gelten?".into()), actions: vec![action("confirm_replan", "Anpassung übernehmen", "primary", Some(vec!["Uni-Ende 13:30 als Korrektur", "Matheblock heute 45 statt 90 Minuten", "Rest des Plans bleibt unverändert"]))],
                progress: 62, status_note: "Änderung verstanden · Anpassung noch nicht bestätigt".into(),
            },
            "replan_confirmed" => BriefingView {
                phase: "changed".into(), day_label: "Donnerstag · später".into(), overline: "Anpassung gilt".into(), title: "Der restliche Tag ist wieder ruhig.".into(), lead: "Nur der Matheblock wurde angepasst. Momentum trägt die bestätigte Korrektur in den Abend weiter.".into(), known,
                meaning: "Die Tagesänderung ist eingegrenzt. Eine ehrliche spätere Auswertung vergleicht nun nicht mehr mit dem alten 90-Minuten-Plan.".into(), recommendation: "Mit dem angepassten Ablauf fortfahren.".into(), reason: "Es gibt keine weitere entscheidende Lücke. Zusätzliche Pflege wäre jetzt keine Entlastung.".into(), alternative: "Bei Erschöpfung Durchführung offenlassen".into(), alternative_cost: "Keine falsche Erfolgsmeldung, aber weniger Information für morgen.".into(), unknowns: vec![], changed: vec!["45-Minuten-Matheblock ist bestätigt".into()], unchanged: vec!["Rückgabeabsicht und übrige Prioritäten".into()], question: Some("Bereit für einen kurzen Abendabschluss?".into()), actions: vec![action("to_evening", "Abendbriefing öffnen", "primary", None)], progress: 76, status_note: "Begrenzte Anpassung bestätigt".into(),
            },
            "evening" => BriefingView {
                phase: "evening".into(), day_label: "Donnerstag · Abend".into(), overline: "Abendbriefing".into(), title: "Momentum kennt den Plan und fragt nur nach der nützlichen Lücke.".into(), lead: "Uni-Verlängerung und angepasster Ablauf müssen nicht neu erzählt werden. Offen ist nur, was für das Testat morgen einen Unterschied macht.".into(), known,
                meaning: "Ob der Matheblock stattgefunden hat, entscheidet über den sinnvollen Wiedereinstieg. Eine ausgelassene Antwort ist kein Misserfolg, sondern bleibt unbekannt.".into(), recommendation: "Nur die Durchführung des Lernblocks festhalten.".into(), reason: "Paket, Geschenk, Training und Rückgabe müssen für die morgige Testatentscheidung nicht vollständig bewertet werden.".into(), alternative: "Ohne Antwort abschließen".into(), alternative_cost: "Weniger Kontext für morgen, aber keine erfundene Bewertung.".into(), unknowns: vec!["Durchführung des angepassten Matheblocks".into()], changed: vec![], unchanged: vec!["Bekannte Tagesänderung wird wiederverwendet".into(), "Keine Antwort wird nicht als Scheitern gewertet".into()], question: Some("Hat der geplante Matheblock stattgefunden?".into()), actions: vec![action("learning_done", "Ja, durchgeführt", "primary", None), action("learning_unknown", "Offenlassen", "secondary", None)], progress: 88, status_note: "Eine nützliche Lücke · kein Tagesformular".into(),
            },
            _ => {
                let done = state.learning_result.as_deref() == Some("confirmed_done");
                BriefingView { phase: "next_day".into(), day_label: "Freitag · Morgen".into(), overline: "Weitergetragen".into(), title: if done { "Gestern liefert einen konkreten Startpunkt – keine pauschale Note." } else { "Momentum beginnt ehrlich mit dem, was noch unbekannt ist." }.into(), lead: if done { "Der bestätigte Lernblock wird heute genutzt. Aus der Uni-Verlängerung entsteht kein Urteil über den ganzen Tag." } else { "Die ausgelassene Angabe wurde nicht als Misserfolg gespeichert. Der nächste Vorschlag bleibt entsprechend vorsichtig." }.into(), known,
                    meaning: if done { "Kontinuität ist bestätigt; der Kenntnisstand selbst bleibt trotzdem offen. Heute lohnt eine kurze Standortbestimmung vor dem nächsten Schwerpunkt." } else { "Ohne Durchführungsangabe kann Momentum keine Lernwirkung behaupten. Die Testatfrist bleibt relevant, der Startpunkt aber offen." }.into(),
                    recommendation: if done { "Heute mit einer 10-Minuten-Standortbestimmung starten." } else { "Vor einer neuen Planung kurz den Mathe-Stand klären." }.into(),
                    reason: if done { "Das nutzt die bestätigte Handlung, ohne daraus automatisch Kompetenz abzuleiten." } else { "Eine gezielte Klärung ist hilfreicher als den gestrigen Block still als erledigt oder ausgefallen zu behandeln." }.into(),
                    alternative: "Direkt einen neuen 45-Minuten-Block beginnen".into(), alternative_cost: "Schneller Einstieg, aber weniger Klarheit über die richtige Aufgabe.".into(), unknowns: vec!["Konkreter Mathe-Kenntnisstand".into()], changed: vec![if done { "Bestätigte Durchführung wird als Startpunkt genutzt" } else { "Unbekannte Durchführung bleibt ausdrücklich offen" }.into()], unchanged: vec!["Testatfrist bleibt bestehen".into(), "Aus einer Handlung wird keine Kompetenz erfunden".into()], question: Some("Der vollständige K5-Kreislauf ist durchlaufen. Noch einmal ansehen?".into()), actions: vec![action("restart_walkthrough", "Prüffall neu beginnen", "secondary", Some(vec!["Nur synthetische K5-Daten zurücksetzen"]))], progress: 100, status_note: "Morgen → Änderung → Abend → Folgetag verbunden".into() }
            }
        }
    }
}

fn prepare_without_relevant_deadline(state: &ScenarioState) -> BriefingView {
    let mut known = vec![
        fact(
            "Feste Bindung",
            if state.uni_extended {
                "Uni · 09:00–13:30"
            } else {
                "Uni · 09:00–12:00"
            },
            "bestätigt",
            "lokaler Termin",
        ),
        fact(
            "Vorhaben",
            "Mathe-Testat in 10 Tagen · Kenntnisstand offen",
            "angegeben",
            "synthetischer Prüffall",
        ),
    ];
    if let Some(home) = state.home_after_uni {
        known.push(fact(
            "Rückkehr nach der Uni",
            if home {
                "Gegen 13:00 zuhause"
            } else {
                "Danach nicht direkt zuhause"
            },
            "angegeben",
            "deine Antwort",
        ));
    }
    if let Some(result) = &state.learning_result {
        known.push(fact(
            "Lernblock",
            if result == "confirmed_done" {
                "durchgeführt"
            } else {
                "Durchführung unbekannt"
            },
            if result == "confirmed_done" {
                "bestätigt"
            } else {
                "unbekannt"
            },
            if result == "confirmed_done" {
                "deine Beobachtung"
            } else {
                "keine Angabe"
            },
        ));
    }

    let (phase, day_label, question, actions, progress) = match state.phase.as_str() {
        "morning_unknown" => (
            "morning",
            "Donnerstag · Morgen",
            Some("Bist du nach der Uni gegen 13 Uhr zuhause?".into()),
            vec![
                action("home_by_13", "Ja, gegen 13 Uhr", "primary", None),
                action("away_after_uni", "Nein, erst später", "secondary", None),
            ],
            18,
        ),
        "morning_answered" => (
            "morning",
            "Donnerstag · Morgen",
            Some("Soll der Lernblock so geschützt werden?".into()),
            vec![action(
                "confirm_plan",
                "Lernblock übernehmen",
                "primary",
                Some(vec!["90 Minuten Mathe als geschützter Block"]),
            )],
            32,
        ),
        "plan_confirmed" => (
            "changed",
            "Donnerstag · im Verlauf",
            Some(
                "Gab es eine relevante Änderung – oder ist der Tag bereit für den Rückblick?"
                    .into(),
            ),
            vec![
                action(
                    "uni_extended",
                    "Uni dauert 90 Minuten länger",
                    "secondary",
                    None,
                ),
                action("to_evening", "Zum Abendrückblick", "primary", None),
            ],
            53,
        ),
        "day_changed" => (
            "changed",
            "Donnerstag · Änderung",
            Some("Soll nur der verkürzte Lernblock gelten?".into()),
            vec![action(
                "confirm_replan",
                "Anpassung übernehmen",
                "primary",
                Some(vec!["Matheblock heute 45 statt 90 Minuten"]),
            )],
            62,
        ),
        "replan_confirmed" => (
            "changed",
            "Donnerstag · später",
            Some("Bereit für einen kurzen Abendabschluss?".into()),
            vec![action(
                "to_evening",
                "Abendbriefing öffnen",
                "primary",
                None,
            )],
            76,
        ),
        "evening" => (
            "evening",
            "Donnerstag · Abend",
            Some("Hat der geplante Matheblock stattgefunden?".into()),
            vec![
                action("learning_done", "Ja, durchgeführt", "primary", None),
                action("learning_unknown", "Offenlassen", "secondary", None),
            ],
            88,
        ),
        _ => (
            "next_day",
            "Freitag · Morgen",
            Some("Den synthetischen Ablauf noch einmal ansehen?".into()),
            vec![action(
                "restart_walkthrough",
                "Prüffall neu beginnen",
                "secondary",
                Some(vec!["Nur synthetische Prüfdaten zurücksetzen"]),
            )],
            100,
        ),
    };

    BriefingView {
        phase: phase.into(),
        day_label: day_label.into(),
        overline: "Bestätigte Grundlage".into(),
        title: "Freie Datumsangaben bleiben ohne bestätigte Bedeutung aus dem Plan.".into(),
        lead: "Momentum nutzt nur die bestätigten Termin- und Lerninformationen. Es erfindet aus einem freien Datumsfeld keine Frist oder Aufgabe.".into(),
        known,
        meaning: "Aus dem Studio liegt keine Datumsangabe mit bestätigter Bedeutung und bestätigtem Vorhabensbezug vor. Sie beeinflusst den Vorschlag deshalb nicht.".into(),
        recommendation: if state.uni_extended {
            "Nach der verlängerten Uni einen realistischen 45-Minuten-Lernstart schützen."
        } else {
            "Nach der Uni einen zusammenhängenden Matheblock schützen."
        }
        .into(),
        reason: "Der Lernkontext ist belegt; eine weitere Dringlichkeit ist es nicht.".into(),
        alternative: "Das freie Datum zunächst nur als Information behalten".into(),
        alternative_cost: "Es wirkt erst dann auf Vorschläge, wenn seine Bedeutung bewusst bestätigt wurde.".into(),
        unknowns: vec!["Konkreter Mathe-Kenntnisstand".into()],
        changed: vec![],
        unchanged: vec!["Unbestätigte freie Datumsangaben bleiben ohne Planwirkung".into()],
        question,
        actions,
        progress,
        status_note: "Keine unbegründete Frist abgeleitet".into(),
    }
}

fn fact(label: &str, value: &str, state: &str, source: &str) -> KnownFact {
    KnownFact {
        label: label.into(),
        value: value.into(),
        state: state.into(),
        source: source.into(),
    }
}
fn action(id: &str, label: &str, tone: &str, preview: Option<Vec<&str>>) -> ActionChoice {
    ActionChoice {
        id: id.into(),
        label: label.into(),
        tone: tone.into(),
        preview: preview.map(|values| values.into_iter().map(str::to_string).collect()),
    }
}
fn pretty_date(value: &str) -> String {
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map(|date| date.format("%d.%m.%Y").to_string())
        .unwrap_or_else(|_| value.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(phase: &str) -> ScenarioState {
        ScenarioState {
            phase: phase.into(),
            home_after_uni: Some(true),
            plan_confirmed: phase != "morning_unknown" && phase != "morning_answered",
            uni_extended: false,
            replan_confirmed: false,
            learning_result: None,
            irrelevant_color: None,
        }
    }
    fn deadline() -> RelevantDeadline {
        RelevantDeadline {
            collection: "Freie Sammlung".into(),
            item: "Eintrag A".into(),
            date: "2026-09-10".into(),
            meaning: "bestätigte Frist".into(),
            related_intention: "Eigenes Vorhaben".into(),
        }
    }

    #[test]
    fn briefing_uses_generic_confirmed_deadline_context() {
        let view = LocalReasoner.prepare(&state("morning_unknown"), Some(&deadline()));
        assert!(
            view.known
                .iter()
                .any(|fact| fact.label == "Freie Sammlung" && fact.value.contains("Eintrag A"))
        );
        assert!(view.meaning.contains("Eigenes Vorhaben"));
    }

    #[test]
    fn irrelevant_color_does_not_change_recommendation() {
        let before = LocalReasoner.prepare(&state("plan_confirmed"), Some(&deadline()));
        let mut after_state = state("plan_confirmed");
        after_state.irrelevant_color = Some("salbeigrün".into());
        let after = LocalReasoner.prepare(&after_state, Some(&deadline()));
        assert_eq!(before.recommendation, after.recommendation);
        assert_eq!(before.reason, after.reason);
    }

    #[test]
    fn missing_confirmed_deadline_is_not_invented() {
        let view = LocalReasoner.prepare(&state("morning_unknown"), None);
        assert!(view.meaning.contains("keine Datumsangabe"));
        assert!(
            view.known
                .iter()
                .all(|fact| !fact.value.contains("Rückgabe"))
        );
        assert!(view.lead.contains("keine Frist oder Aufgabe"));
    }

    #[test]
    fn return_time_changes_the_recommendation() {
        let mut home = state("morning_answered");
        home.home_after_uni = Some(true);
        let mut away = state("morning_answered");
        away.home_after_uni = Some(false);

        let home_view = LocalReasoner.prepare(&home, Some(&deadline()));
        let away_view = LocalReasoner.prepare(&away, Some(&deadline()));

        assert_ne!(home_view.recommendation, away_view.recommendation);
        assert_ne!(home_view.reason, away_view.reason);
    }
}
