#!/bin/bash
# Pick a shelf tier-1 box and place it on the AGV base, in MuJoCo. See pick_place.py.
set -e
HERE="$(cd "$(dirname "$0")" && pwd)"
PY="$HERE/.venv/bin/python"
[ -f "$HERE/scene.xml" ] || "$HERE/run.sh" --no-view
exec "$PY" "$HERE/pick_place.py" "$@"
