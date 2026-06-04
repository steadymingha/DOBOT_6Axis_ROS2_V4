"""Quick check of copyFromReals / copyToReals call forms for this OMPL binding."""
from ompl import base as ob
import numpy as np

rvss = ob.RealVectorStateSpace(3)
b = ob.RealVectorBounds(3); b.setLow(-2.0); b.setHigh(2.0); rvss.setBounds(b)


class C(ob.Constraint):
    def __init__(self):
        super().__init__(3, 1)

    def function(self, x, out):
        a = np.asarray(x)
        out[0] = float(a @ a - 1.0)


css = ob.ProjectedStateSpace(rvss, C())
csi = ob.ConstrainedSpaceInformation(css)
st = csi.allocState()

print("copyFromReals exists:", hasattr(css, "copyFromReals"))
try:
    css.copyFromReals(st, [0.6, 0.8, 0.0])
    print("copyFromReals(state, list) OK")
except Exception as e:
    print("copyFromReals ERR:", repr(e))

print("copyToReals exists:", hasattr(css, "copyToReals"))
try:
    r = css.copyToReals(st)
    print("copyToReals(state) returns:", type(r).__name__, list(r))
except Exception as e:
    print("copyToReals(state) ERR:", repr(e))
