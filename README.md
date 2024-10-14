# reaction_net

`reaction_net` is a python library for manipulating networks of chemical reactions. For example, consider the following reactions:

```
A + B -> C
C -> D
D <-> E
```

This library provides tools to parse the text above into a nested data structure (essentially a dictionary). This data structure can be used to create diagrams, network-related matrices, or nonlinear vector fields that describe the behavior of the chemical reaction networks. 

This library is currently a work in progress.