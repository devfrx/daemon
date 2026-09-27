import { Orientation, type SerializedDockview } from "dockview-core";

/** One group of a layout, as a miniature draws it: where it sits, in fractions of the whole, and its panels. */
export interface Tile {
  x: number;
  y: number;
  width: number;
  height: number;
  views: string[];
  active?: string;
}

/** A node of the serialized grid, as `toJSON()` writes it: a branch holds nodes, a leaf holds a group. */
interface GridNode {
  type: "branch" | "leaf";
  data: GridNode[] | { views: string[]; activeView?: string };
  size?: number;
}

/**
 * The miniature of a saved layout (answer 19 of the design system): the groups of its grid as rectangles in fractions of
 * the unit square, drawn from the tree and not from a `dockview` of their own -- they always say what the layout holds,
 * cost almost nothing and hold with ten views.
 *
 * ⛔ THE ORIENTATION ALTERNATES AT EVERY LEVEL, starting from `grid.orientation`: the root lays its children along it and
 * each branch below along the other axis -- how `dockview-core` 8.3.1 reads the tree back (`_deserializeNode` hands
 * `orthogonal(orientation)` to the children). A node's `size` is its extent along its parent's axis.
 * ⛔ THE FLOATING GROUPS ARE NOT DRAWN (D5 of the plan): they have no place in the grid.
 * ⛔ A MAXIMIZED GROUP IS DRAWN ALONE, ON THE WHOLE SQUARE (E85 of the plan): `dockview-core` 8.3.1 writes the grid as it
 * is with nothing maximized, plus `grid.maximizedNode` -- the indices from the root down to that group, which the public
 * type `SerializedDockview` does not declare -- and `fromJSON` opens the layout with the group maximized: what opens is
 * what is drawn. ⚠️ A node hidden with `setVisible` is written with `visible: false` and the size it had: nothing here
 * hides one, and the miniature would draw it.
 */
export function schematic(layout: SerializedDockview): Tile[] {
  const tiles: Tile[] = [];
  const place = (node: GridNode, box: Omit<Tile, "views" | "active">, orientation: Orientation): void => {
    if (node.type === "leaf") {
      const group = node.data as { views: string[]; activeView?: string };
      tiles.push(group.activeView === undefined ? { ...box, views: group.views } : { ...box, views: group.views, active: group.activeView });
      return;
    }
    const children = node.data as GridNode[];
    const total = children.reduce((sum, child) => sum + (child.size ?? 0), 0);
    const next = orientation === Orientation.HORIZONTAL ? Orientation.VERTICAL : Orientation.HORIZONTAL;
    let offset = 0;
    for (const child of children) {
      const share = total > 0 ? (child.size ?? 0) / total : 1 / children.length;
      place(
        child,
        orientation === Orientation.HORIZONTAL
          ? { x: box.x + offset * box.width, y: box.y, width: share * box.width, height: box.height }
          : { x: box.x, y: box.y + offset * box.height, width: box.width, height: share * box.height },
        next,
      );
      offset += share;
    }
  };
  const { maximizedNode } = layout.grid as { maximizedNode?: { location: number[] } };
  const shown = (maximizedNode?.location ?? []).reduce<GridNode>(
    (node, index) => (node.data as GridNode[])[index] as GridNode,
    layout.grid.root as unknown as GridNode,
  );
  place(shown, { x: 0, y: 0, width: 1, height: 1 }, layout.grid.orientation);
  return tiles;
}
