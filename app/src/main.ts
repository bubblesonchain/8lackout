// Offline-first PWA entry. PLACEHOLDER.
import { renderBoard } from "./views/Board.ts";
const el = document.getElementById("app")!;
el.innerHTML = "<h1 style='letter-spacing:.14em'>8LACKOUT</h1><p>offline mode · 3 peers in range</p>";
renderBoard(el);
