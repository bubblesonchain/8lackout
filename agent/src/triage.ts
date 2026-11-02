// Raw text/voice -> structured, routable need. PLACEHOLDER.
export interface Need { kind: string; count: number; location: string; urgency: "low"|"med"|"high"; }
export function triage(raw: string): Need {
  return { kind: raw.split(",")[0]?.trim() || "unknown", count: 1, location: "unknown", urgency: "med" };
}
