import {
  Activity,
  AppWindow,
  Archive,
  BookmarkPlus,
  Box,
  Coins,
  Cpu,
  Eye,
  FolderTree,
  Gauge,
  GitCompare,
  Layers,
  LayoutGrid,
  ListChecks,
  Maximize2,
  MessageSquare,
  Mic,
  Network,
  Radar,
  RotateCcw,
  Search,
  Settings,
  ShieldCheck,
  SquareTerminal,
  type IconNode,
} from "lucide";

/**
 * ⛔ THE ONE MAP OF THE PROGRAM'S ICONS (answer 11 of the design system): OUR name -> a Lucide drawing, and no other
 * file imports `lucide` -- the linter says so. Changing the set touches this file alone. Only the icons we use are
 * here: one is about half a kB, the whole set hundreds of kB (the measures of answer 11).
 *
 * Lucide 1.47.0 is ISC, and MIT for the icons that come from Feather: the licences travel with the package the
 * shell of sub-project 10 will ship (trap 14 of the design).
 */
export const ICONS = {
  // the frame
  views: Layers,
  modules: LayoutGrid,
  search: Search,
  saveView: BookmarkPlus,
  float: AppWindow,
  fullPage: Maximize2,
  // one per module type of `panels/registry.ts`, BY THE SAME NAME: the big grab and the overview draw them. The
  // boards gave Stato, Permessi, Passi and Attività; the rest are the plan's choice, one line each (D6) -- Chat too:
  // on the boards `message-square` marks the status messages, and they would need another drawing if they entered
  // the kit (R2-15 of the review).
  chat: MessageSquare,
  status: Gauge,
  permissions: ShieldCheck,
  steps: ListChecks,
  activity: Activity,
  settings: Settings,
  scope: FolderTree,
  diff: GitCompare,
  preview: Eye,
  terminal: SquareTerminal,
  sensors: Radar,
  costs: Coins,
  knowledge: Network,
  assets3d: Box,
  voice: Mic,
  backup: Archive,
  checkpoint: RotateCcw,
  models: Cpu,
} satisfies Record<string, IconNode>;

export type IconName = keyof typeof ICONS;

export function isIconName(name: string): name is IconName {
  return Object.hasOwn(ICONS, name);
}
