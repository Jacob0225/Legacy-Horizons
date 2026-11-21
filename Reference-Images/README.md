# Legacy4J visual references

User-supplied reference captures for screen-by-screen GUI parity. The initial
collection contains 145 PNGs at 2560×1440, organized into main-menu, settings,
screens, dialogs, inventory and containers, plus `manifest.json`.

The capture metadata identifies **Legacy4J 1.9.1 on Minecraft 1.21.10** and
includes screen class/title, GUI scale, timestamps, hashes and capture state.
Horizon currently targets Minecraft 26.3: distinguish version-specific content
from Legacy4J's design instead of assuming every item/recipe is identical.
The manifest is a snapshot of the capture run and may not describe every PNG;
use the records available for the specific image, without inventing missing
navigation or input-state information.

## Backgrounds and comparison rules

The owner clarified that backgrounds are fine where Legacy4J actually has
them. This supersedes the earlier blanket rule requiring every button-list
screen to be transparent. Match each exact reference: retain its panel when
present and retain transparency when clear. Do not infer How To Play's or a
container's background from a different settings screen.

Before calling a screen visually complete:

1. Identify its matching reference, version, GUI scale and visible state.
2. Inspect the corresponding Legacy4J source for behavior and layout.
3. Capture Horizon at matching resolution/scale/state. Compare the GUI regions
   directly, using overlays/differences where useful; account separately for
   animated panoramas or different world scenes.
4. Check actual widget types, typography, spacing, borders, focus, tabs,
   tooltips and disabled states. A working vertical button list does not match
   a reference that uses checkboxes/sliders just because its background matches.
5. Test interactions in-game, recording any remaining API or behavior gaps.
   A static screenshot does not establish navigation, animation or gameplay.

The images are references, not artwork to paste over a nonfunctional screen.
Current port screenshots elsewhere in the repository are progress evidence;
they are not authoritative Legacy4J references. Keep the README checklist and
`GAMEPLAY-STATUS.md` honest about differences and completed verification.
