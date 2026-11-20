# mod API: camera pose roll for Legacy4J elytra banking

> 🤖 Investigated by Jacob0225's AI agent (Codex, GPT-6), not by Jacob0225 personally.

Existing issue: https://github.com/horizon-rust/horizon/issues/18694

Port use-case comment: https://github.com/horizon-rust/horizon/issues/18694#issuecomment-7030565477

No duplicate issue was created. The existing camera-pose request includes roll;
its maintainer reply explicitly confirms roll is missing. The pinned SDK already
has writable yaw and pitch, so this port does not repeat the original issue's
claim about those two fields.

Legacy4J's `CameraMixin` applies `LegacyRenderUtil.getFlyingViewRollingRotation`
to elytra flight. It compares the horizontal view and interpolated velocity,
then banks by `min(pi/8, acos(clamp(dot, -1, 1)) / 2.5)`, with the direction
from their cross product. Its maximum bank is 22.5 degrees.

Horizon `camera_pose` has position, yaw and pitch but no roll. `camera_shake`
does have roll, but `native_shake_matrix` passes it through `bounded_shake`
and scales it by Distortion Effects. That produces a different result,
including disappearing when that setting is zero. It cannot provide exact
camera-pose banking. This is separate from issue 15's movement physics.

Inspected target: Horizon `0b14f8acfd97fa2c750e42795249dcc981178ca4`, Mod API 0.2.
The port leaves flight camera banking unimplemented pending the existing request.
