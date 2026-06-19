#!/usr/bin/env python3
"""Print per-part AABB bounds of the gripper DAE meshes in the link frame.

Run after re-exporting the gripper from Blender; the numbers feed directly into
gripper.xacro <collision> boxes and the cbirrt_pick_place.py pad constants.
See gripper_change_checklist.md for which value goes where.

Usage:
    python3 measure_gripper_dae.py gripper_long/meshes/base.dae \
                                   gripper_long/meshes/finger.dae
"""
import sys
import xml.etree.ElementTree as ET
import numpy as np

NS = {'c': 'http://www.collada.org/2005/11/COLLADASchema'}


def per_node_bounds(fn):
    """Yield (node_name, min_xyz, max_xyz) for every instanced geometry, with the
    Collada scene-node matrices accumulated (so bounds are in the link frame)."""
    root = ET.parse(fn).getroot()
    geos = {}
    for g in root.iter('{%s}geometry' % NS['c']):
        for src in g.iter('{%s}source' % NS['c']):
            fa = src.find('{%s}float_array' % NS['c'])
            if fa is not None and 'position' in src.get('id', '').lower():
                geos[g.get('id')] = np.array(
                    [float(x) for x in fa.text.split()]).reshape(-1, 3)
    out = []

    def walk(node, M):
        m = node.find('{%s}matrix' % NS['c'])
        if m is not None:
            M = M @ np.array([float(x) for x in m.text.split()]).reshape(4, 4)
        ig = node.find('{%s}instance_geometry' % NS['c'])
        if ig is not None:
            gid = ig.get('url').lstrip('#')
            if gid in geos:
                pts = geos[gid]
                w = (M @ np.hstack([pts, np.ones((len(pts), 1))]).T).T[:, :3]
                out.append((node.get('name') or node.get('id') or gid,
                            w.min(0), w.max(0)))
        for ch in node.findall('{%s}node' % NS['c']):
            walk(ch, M)

    for vs in root.iter('{%s}visual_scene' % NS['c']):
        for node in vs.findall('{%s}node' % NS['c']):
            walk(node, np.eye(4))
    return out


def main(files):
    for f in files:
        print("====", f)
        for nm, lo, hi in per_node_bounds(f):
            c = (lo + hi) / 2
            s = hi - lo
            print("  %-14s X[%+.4f,%+.4f] Y[%+.4f,%+.4f] Z[%+.4f,%+.4f]  "
                  "origin(%+.4f,%+.4f,%+.4f) size(%.4f,%.4f,%.4f)" % (
                      nm, lo[0], hi[0], lo[1], hi[1], lo[2], hi[2],
                      c[0], c[1], c[2], s[0], s[1], s[2]))


if __name__ == '__main__':
    args = sys.argv[1:] or ['gripper_long/meshes/base.dae',
                            'gripper_long/meshes/finger.dae']
    main(args)
