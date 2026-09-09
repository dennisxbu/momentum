import { useEffect, useMemo, useState } from "react";
import {
  createCollection,
  createItem,
  exportData,
  loadWorkspace,
  renameProperty,
  resetDemo,
  respondToBriefing,
  restoreData,
  setStudioCollection,
} from "./api";
import type {
  ActionChoice,
  CollectionDraft,
  CollectionView,
  ItemDraftValue,
  PropertyDraft,
  PropertyKind,
  WorkspaceView,
} from "./types";

type Section = "briefing" | "studio" | "data";

const propertyLabels: Record<PropertyKind, string> = {
  text: "Text",
  number: "Zahl mit Einheit",
  date: "Datum",
  choice: "Auswahl",
  relation: "Beziehung",
};

function App() {
  const [workspace, setWorkspace] = useState<WorkspaceView>();
  const [section, setSection] = useState<Section>("briefing");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string>();
  const [notice, setNotice] = useState<string>();
  const [pendingAction, setPendingAction] = useState<ActionChoice>();
  const [showCreateCollection, setShowCreateCollection] = useState(false);
  const [showCreateItem, setShowCreateItem] = useState(false);

  useEffect(() => {
    void perform(() => loadWorkspace());
  }, []);

  async function perform(action: () => Promise<WorkspaceView | null>, success?: string) {
    setBusy(true);
    setError(undefined);
    try {
      const next = await action();
      if (next) setWorkspace(next);
      if (success) setNotice(success);
    } catch (caught) {
      setError(typeof caught === "string" ? caught : caught instanceof Error ? caught.message : "Das hat nicht geklappt.");
    } finally {
      setBusy(false);
    }
  }

  async function chooseAction(action: ActionChoice) {
    if (action.preview?.length) {
      setPendingAction(action);
      return;
    }
    await perform(() => respondToBriefing(action.id));
  }

  async function confirmAction() {
    if (!pendingAction) return;
    const action = pendingAction;
    setPendingAction(undefined);
    await perform(() => respondToBriefing(action.id), "Die Änderung ist bestätigt und im Verlauf festgehalten.");
  }

  if (!workspace) {
    return (
      <main className="launch-state">
        <div className="brand-mark">M</div>
        <h1>Momentum wird vorbereitet</h1>
        <p>{error ?? "Deine lokale Arbeitsgrundlage wird geöffnet …"}</p>
        {error && <button onClick={() => void perform(() => loadWorkspace())}>Erneut versuchen</button>}
      </main>
    );
  }

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand"><span className="brand-mark small">M</span><span>Momentum</span></div>
        <nav aria-label="Hauptbereiche">
          <NavButton active={section === "briefing"} label="Briefing" hint="Was jetzt zählt" onClick={() => setSection("briefing")} />
          <NavButton active={section === "studio"} label="Studio" hint="Dein eigener Raum" onClick={() => setSection("studio")} />
          <NavButton active={section === "data"} label="Datenhoheit" hint="Lokal & bei dir" onClick={() => setSection("data")} />
        </nav>
        <div className="sidebar-foot">
          <span className="privacy-dot" />
          <div><strong>Lokal geschützt</strong><small>Keine Cloud · kein Konto</small></div>
        </div>
      </aside>

      <main className="workspace">
        <header className="topbar">
          <div>
            <span className="synthetic-label">Synthetischer Entwicklungsstand</span>
          </div>
          <div className="top-status"><span>{workspace.history_count} nachvollziehbare Änderungen</span><span className="avatar">D</span></div>
        </header>

        {notice && <div className="notice" role="status">{notice}<button aria-label="Hinweis schließen" onClick={() => setNotice(undefined)}>×</button></div>}
        {error && <div className="notice error" role="alert">{error}<button aria-label="Fehler schließen" onClick={() => setError(undefined)}>×</button></div>}

        {section === "briefing" && (
          <Briefing workspace={workspace} busy={busy} onChoose={chooseAction} onOpenStudio={() => setSection("studio")} />
        )}
        {section === "studio" && (
          <Studio
            workspace={workspace}
            busy={busy}
            onSelect={(id) => void perform(() => setStudioCollection(id))}
            onOpenCollection={() => setShowCreateCollection(true)}
            onOpenItem={() => setShowCreateItem(true)}
            onRename={(id, name) => void perform(() => renameProperty(id, name), "Neue Eigenschaftsversion angelegt. Ältere Werte behalten ihren damaligen Namen.")}
          />
        )}
        {section === "data" && (
          <DataControl
            workspace={workspace}
            busy={busy}
            onExport={async () => {
              setBusy(true); setError(undefined);
              try {
                const path = await exportData();
                if (path) setNotice(`Vollständiger Export gespeichert: ${path}`);
              } catch (caught) { setError(String(caught)); }
              finally { setBusy(false); }
            }}
            onRestore={() => void perform(() => restoreData(), "Der geprüfte Export wurde vollständig wiederhergestellt.")}
            onReset={() => void perform(() => resetDemo(), "Der synthetische Prüffall wurde neu angelegt.")}
          />
        )}
      </main>

      {pendingAction && (
        <ConfirmPanel action={pendingAction} busy={busy} onCancel={() => setPendingAction(undefined)} onConfirm={() => void confirmAction()} />
      )}
      {showCreateCollection && (
        <CollectionComposer
          busy={busy}
          onClose={() => setShowCreateCollection(false)}
          onCreate={(draft) => {
            setShowCreateCollection(false);
            void perform(() => createCollection(draft), `„${draft.name}“ ist bereit.`);
          }}
        />
      )}
      {showCreateItem && (
        <ItemComposer
          collection={workspace.studio.collections.find((entry) => entry.id === workspace.studio.selected_collection_id)!}
          collections={workspace.studio.collections}
          busy={busy}
          onClose={() => setShowCreateItem(false)}
          onCreate={(title, values) => {
            setShowCreateItem(false);
            void perform(() => createItem(workspace.studio.selected_collection_id, title, values), `„${title}“ wurde eingeordnet.`);
          }}
        />
      )}
    </div>
  );
}

function NavButton({ active, label, hint, onClick }: { active: boolean; label: string; hint: string; onClick: () => void }) {
  return <button className={`nav-button ${active ? "active" : ""}`} onClick={onClick}><span>{label}</span><small>{hint}</small></button>;
}

function Briefing({ workspace, busy, onChoose, onOpenStudio }: {
  workspace: WorkspaceView;
  busy: boolean;
  onChoose: (action: ActionChoice) => void;
  onOpenStudio: () => void;
}) {
  const briefing = workspace.briefing;
  return (
    <div className="page briefing-page">
      <section className="briefing-hero">
        <div className="hero-copy">
          <span className="overline">{briefing.overline}</span>
          <h1>{briefing.title}</h1>
          <p>{briefing.lead}</p>
        </div>
        <div className="day-progress" aria-label={`${briefing.progress} Prozent des Ablaufs`}>
          <span>{briefing.day_label}</span>
          <div className="progress-track"><i style={{ width: `${briefing.progress}%` }} /></div>
          <small>{briefing.status_note}</small>
        </div>
      </section>

      <section className="reasoning-flow" aria-label="So kommt Momentum zum Vorschlag">
        <article>
          <StepNumber value="01" label="Bekannt" />
          <div className="fact-list">
            {briefing.known.map((fact) => (
              <div className="fact" key={`${fact.label}-${fact.value}`}>
                <div><strong>{fact.label}</strong><span>{fact.value}</span></div>
                <small className={`state ${fact.state}`}>{fact.state} · {fact.source}</small>
              </div>
            ))}
          </div>
          <button className="text-action" onClick={onOpenStudio}>Grundlage im Studio ansehen →</button>
        </article>

        <article>
          <StepNumber value="02" label="Bedeutung" />
          <p className="meaning-copy">{briefing.meaning}</p>
          {briefing.unknowns.length > 0 && <div className="unknown-block"><span>Noch offen</span>{briefing.unknowns.map((value) => <p key={value}>{value}</p>)}</div>}
        </article>

        <article className="recommendation-pane">
          <StepNumber value="03" label="Vorschlag" />
          <h2>{briefing.recommendation}</h2>
          <p>{briefing.reason}</p>
          <div className="alternative"><span>Gute Alternative</span><strong>{briefing.alternative}</strong><small>{briefing.alternative_cost}</small></div>
        </article>

        <article>
          <StepNumber value="04" label="Wirkung" />
          {briefing.changed.length > 0 ? (
            <ChangeList title="Ändert sich" values={briefing.changed} changed />
          ) : <p className="muted-copy">Noch nichts wird verändert. Erst deine konkrete Bestätigung macht den Vorschlag zum geltenden Plan.</p>}
          {briefing.unchanged.length > 0 && <ChangeList title="Bleibt bewusst gleich" values={briefing.unchanged} />}
        </article>
      </section>

      <section className="decision-dock">
        <div><span>Nächster sinnvoller Moment</span><h2>{briefing.question}</h2></div>
        <div className="action-row">
          {briefing.actions.map((action) => <button disabled={busy} className={`action ${action.tone}`} key={action.id} onClick={() => onChoose(action)}>{action.label}</button>)}
        </div>
      </section>
    </div>
  );
}

function StepNumber({ value, label }: { value: string; label: string }) {
  return <header className="step-title"><span>{value}</span><h3>{label}</h3></header>;
}

function ChangeList({ title, values, changed = false }: { title: string; values: string[]; changed?: boolean }) {
  return <div className={`change-list ${changed ? "changed" : ""}`}><span>{title}</span>{values.map((value) => <p key={value}>{value}</p>)}</div>;
}

function Studio({ workspace, busy, onSelect, onOpenCollection, onOpenItem, onRename }: {
  workspace: WorkspaceView;
  busy: boolean;
  onSelect: (id: string) => void;
  onOpenCollection: () => void;
  onOpenItem: () => void;
  onRename: (id: string, name: string) => void;
}) {
  const studio = workspace.studio;
  const selected = studio.collections.find((entry) => entry.id === studio.selected_collection_id) ?? studio.collections[0];
  return (
    <div className="page studio-page">
      <header className="page-heading">
        <div><span className="overline">Dein eigener Denkraum</span><h1>Studio</h1><p>Strukturen passen sich deiner Frage an. Kein Lebensbereich ist vorgeschrieben.</p></div>
        <button className="action primary" onClick={onOpenCollection}>Neue Sammlung</button>
      </header>

      <div className="collection-tabs">
        {studio.collections.map((collection) => <button key={collection.id} className={collection.id === selected.id ? "active" : ""} onClick={() => onSelect(collection.id)}>{collection.name}</button>)}
      </div>

      <section className="collection-heading">
        <div><h2>{selected.name}</h2><p>{selected.description}</p></div>
        <button className="action secondary" disabled={busy} onClick={onOpenItem}>Eintrag hinzufügen</button>
      </section>

      <div className="view-rationale"><span>Aktuelle Sicht</span><strong>{studio.view_label}</strong><p>{studio.view_explanation}</p></div>

      <div className="data-table-wrap">
        <table>
          <thead><tr><th>Eintrag</th>{selected.properties.filter((p) => p.active).map((property) => <th key={property.id}><PropertyHeader property={property} onRename={onRename} /></th>)}</tr></thead>
          <tbody>
            {selected.items.map((item) => (
              <tr key={item.id}><td><strong>{item.title}</strong></td>{selected.properties.filter((p) => p.active).map((property) => {
                const value = item.values.find((entry) => entry.logical_id === property.logical_id);
                return <td key={property.id}>{value?.display ?? <span className="empty-value">—</span>}</td>;
              })}</tr>
            ))}
            {selected.items.length === 0 && <tr><td className="empty-table" colSpan={selected.properties.length + 1}>Noch keine Einträge. Die Struktur ist bereit, wenn du sie brauchst.</td></tr>}
          </tbody>
        </table>
      </div>
      <p className="studio-footnote">Bedeutungen mit grünem Punkt dürfen nach deiner Bestätigung in Vorschläge einfließen. Feldnamen allein erzeugen keine Verpflichtung.</p>
    </div>
  );
}

function PropertyHeader({ property, onRename }: { property: CollectionView["properties"][number]; onRename: (id: string, name: string) => void }) {
  const [editing, setEditing] = useState(false);
  const [name, setName] = useState(property.name);
  if (editing) return <form className="inline-rename" onSubmit={(event) => { event.preventDefault(); if (name.trim()) onRename(property.id, name.trim()); setEditing(false); }}><input autoFocus value={name} onChange={(event) => setName(event.target.value)} /><button>Speichern</button></form>;
  return <button className="property-head" title={`${propertyLabels[property.kind]} · Version ${property.version}`} onDoubleClick={() => setEditing(true)}><span>{property.meaning_confirmed && <i />} {property.name}</span><small>{propertyLabels[property.kind]}{property.unit ? ` · ${property.unit}` : ""}</small></button>;
}

function DataControl({ workspace, busy, onExport, onRestore, onReset }: { workspace: WorkspaceView; busy: boolean; onExport: () => void; onRestore: () => void; onReset: () => void }) {
  return <div className="page data-page"><header className="page-heading"><div><span className="overline">Datenhoheit</span><h1>Deine Grundlage bleibt bei dir.</h1><p>Momentum funktioniert ohne Konto und ohne Internet. Die laufenden Daten verlassen diesen Computer nicht.</p></div></header><section className="data-summary"><div className="shield">✓</div><div><h2>Lokale Datenbank verschlüsselt</h2><p>{workspace.storage_label}</p><small>Der Schlüssel ist an dein Windows-Benutzerkonto gebunden.</small></div></section><section className="data-actions"><article><span>Mitnehmen</span><h2>Vollständigen Export speichern</h2><p>Ein lesbarer JSON-Export enthält Struktur, Verlauf und aktuellen Stand. Wähle einen sicheren Ablageort.</p><button className="action primary" disabled={busy} onClick={onExport}>Exportieren</button></article><article><span>Zurückholen</span><h2>Geprüft wiederherstellen</h2><p>Momentum prüft den gesamten Export zuerst. Erst ein gültiger Bestand ersetzt die lokalen Daten.</p><button className="action secondary" disabled={busy} onClick={onRestore}>Export auswählen</button></article><article><span>Entwicklungsstand</span><h2>Synthetischen Fall neu beginnen</h2><p>Setzt ausschließlich die künstlichen Prüfdaten zurück. Für K5 sind keine echten persönlichen Daten vorgesehen.</p><button className="action quiet danger" disabled={busy} onClick={onReset}>Prüffall zurücksetzen</button></article></section></div>;
}

function ConfirmPanel({ action, busy, onCancel, onConfirm }: { action: ActionChoice; busy: boolean; onCancel: () => void; onConfirm: () => void }) {
  return <div className="overlay" role="dialog" aria-modal="true"><section className="confirm-panel"><span className="overline">Änderung prüfen</span><h2>Nur das hier wird übernommen</h2><div className="contract-list">{action.preview?.map((value) => <p key={value}>{value}</p>)}</div><p className="confirmation-note">Alles andere bleibt unverändert. Momentum protokolliert diese Bestätigung nachvollziehbar.</p><div className="action-row"><button className="action quiet" onClick={onCancel}>Noch nicht</button><button className="action primary" disabled={busy} onClick={onConfirm}>Genau so übernehmen</button></div></section></div>;
}

function CollectionComposer({ busy, onClose, onCreate }: { busy: boolean; onClose: () => void; onCreate: (draft: CollectionDraft) => void }) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [properties, setProperties] = useState<PropertyDraft[]>([{ name: "", kind: "text", meaning_confirmed: false }]);
  const valid = name.trim() && properties.every((property) => property.name.trim());
  return <div className="overlay" role="dialog" aria-modal="true"><form className="composer wide" onSubmit={(event) => { event.preventDefault(); if (valid) onCreate({ name: name.trim(), description: description.trim(), properties }); }}><header><div><span className="overline">Freie Grundlage</span><h2>Welche Art von Dingen möchtest du verstehen?</h2></div><button type="button" className="close" onClick={onClose}>×</button></header><label>Name<input autoFocus value={name} onChange={(event) => setName(event.target.value)} placeholder="z. B. Rezeptideen" /></label><label>Worum geht es dabei?<textarea value={description} onChange={(event) => setDescription(event.target.value)} placeholder="Kurze Bedeutung für dich – kein starres Ziel." /></label><div className="property-editor"><div className="editor-label"><span>Eigenschaften</span><button type="button" onClick={() => setProperties([...properties, { name: "", kind: "text", meaning_confirmed: false }])}>+ Eigenschaft</button></div>{properties.map((property, index) => <PropertyDraftRow key={index} property={property} onChange={(next) => setProperties(properties.map((entry, entryIndex) => entryIndex === index ? next : entry))} onRemove={() => setProperties(properties.filter((_, entryIndex) => entryIndex !== index))} />)}</div><footer><button type="button" className="action quiet" onClick={onClose}>Abbrechen</button><button className="action primary" disabled={busy || !valid}>Sammlung anlegen</button></footer></form></div>;
}

function PropertyDraftRow({ property, onChange, onRemove }: { property: PropertyDraft; onChange: (property: PropertyDraft) => void; onRemove: () => void }) {
  return <div className="property-draft"><input value={property.name} onChange={(event) => onChange({ ...property, name: event.target.value })} placeholder="Name" /><select value={property.kind} onChange={(event) => onChange({ ...property, kind: event.target.value as PropertyKind })}>{Object.entries(propertyLabels).map(([value, label]) => <option value={value} key={value}>{label}</option>)}</select>{property.kind === "number" && <input value={property.unit ?? ""} onChange={(event) => onChange({ ...property, unit: event.target.value })} placeholder="Einheit" />}{property.kind === "choice" && <input value={property.options?.join(", ") ?? ""} onChange={(event) => onChange({ ...property, options: event.target.value.split(",").map((value) => value.trim()).filter(Boolean) })} placeholder="Optionen, getrennt durch Komma" />}<input value={property.meaning ?? ""} onChange={(event) => onChange({ ...property, meaning: event.target.value })} placeholder="Bedeutung (optional)" /><button type="button" className={`meaning-toggle ${property.meaning_confirmed ? "active" : ""}`} onClick={() => onChange({ ...property, meaning_confirmed: !property.meaning_confirmed })}>{property.meaning_confirmed ? "Bedeutung bestätigt" : "Noch nicht relevant"}</button><button type="button" className="remove" onClick={onRemove}>×</button></div>;
}

function ItemComposer({ collection, collections, busy, onClose, onCreate }: { collection: CollectionView; collections: CollectionView[]; busy: boolean; onClose: () => void; onCreate: (title: string, values: ItemDraftValue[]) => void }) {
  const [title, setTitle] = useState("");
  const [values, setValues] = useState<Record<string, string>>({});
  const relationOptions = useMemo(() => collections.flatMap((entry) => entry.items.map((item) => ({ id: item.id, label: `${entry.name}: ${item.title}` }))), [collections]);
  return <div className="overlay" role="dialog" aria-modal="true"><form className="composer" onSubmit={(event) => { event.preventDefault(); if (title.trim()) onCreate(title.trim(), Object.entries(values).filter(([, value]) => value !== "").map(([property_id, value]) => ({ property_id, value }))); }}><header><div><span className="overline">{collection.name}</span><h2>Neuen Eintrag einordnen</h2></div><button type="button" className="close" onClick={onClose}>×</button></header><label>Bezeichnung<input autoFocus value={title} onChange={(event) => setTitle(event.target.value)} /></label>{collection.properties.filter((property) => property.active).map((property) => <label key={property.id}>{property.name}<small>{property.meaning_confirmed ? property.meaning : "Keine Assistenzbedeutung bestätigt"}</small>{property.kind === "choice" ? <select value={values[property.id] ?? ""} onChange={(event) => setValues({ ...values, [property.id]: event.target.value })}><option value="">Nicht angegeben</option>{property.options.map((option) => <option key={option}>{option}</option>)}</select> : property.kind === "relation" ? <select value={values[property.id] ?? ""} onChange={(event) => setValues({ ...values, [property.id]: event.target.value })}><option value="">Keine Beziehung</option>{relationOptions.map((option) => <option value={option.id} key={option.id}>{option.label}</option>)}</select> : <input type={property.kind === "date" ? "date" : property.kind === "number" ? "number" : "text"} step={property.kind === "number" ? "any" : undefined} value={values[property.id] ?? ""} onChange={(event) => setValues({ ...values, [property.id]: event.target.value })} placeholder={property.unit} />}</label>)}<footer><button type="button" className="action quiet" onClick={onClose}>Abbrechen</button><button className="action primary" disabled={busy || !title.trim()}>Eintrag anlegen</button></footer></form></div>;
}

export default App;
