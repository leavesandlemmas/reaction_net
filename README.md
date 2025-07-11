# reaction_net

`reaction_net` is a parser and compiler designed for mathematical models of chemical reaction networks found in biology and chemistry. For example, consider the following reactions:

```
1 : A + B -> C
2 : C -> D
3 : D <-> E
```

If each of these reactions has mass-action kinetics, then they correspond to the following equations:

$$
    \begin{align} 
        \frac{d}{dt}[A] &= - r_1 & &=  - k_1 [A] [B]\\   
        \frac{d}{dt}[B] &= - r_1 & &=  - k_1 [A] [B]\\ 
        \frac{d}{dt}[C] &=   r_1 - r_2 & &=  k_1 [A] [B] - k_2 [C]\\ 
        \frac{d}{dt}[D] &=   r_2 - r_3 & &=  k_2 [C] - k_3 [D] + k_4 [E]\\
        \frac{d}{dt}[E] &=   r_3 & &=  k_3 [D] - k_4 [E]\\
    \end{align}
$$

These equations can be translated into code for numerical integration. Or into a sparse matrix format for flux balance analysis. While this translation process is most mechanical, it must repeated every time the model's defining equations change, meaning the code has to be rewritten every time. Moreover, many biological models involve many parameters for the reaction kinetics. The exact kinetics are often unknown or approximate. Or reactions might be added or removed as the model changes. When the model is small as the above example, the translation step is easy enough to do by hand. However, as the number of variables grows, the translation step becomes more difficult, hindering changes. 

Furthermore, many numerical algorithms benefit from access to the derivatives (joacbian of the vector field). Both numerical integration and parameter optimization (curve-fitting when the curve is the solution to the set of differential equations). However, the complexity of the models makes differentiation by hand error prone and time-consuming for large models. However, differentiation is also a mechanical process.  Lastly, the code implementation of a model might deviate from the mathematical definition; either due to improvements or due to errors in translation. However, code implementations sometimes obscur the mathematical structure of the model. 

The goal of `reaction_net` is to solve these problems by allowing a model to be defined in a formal language, similar to a programming language, which is then translated into code for various tasks. 

**WORK IN PROGRESS**

Goals/Task:

1. Develop the formal language 
2. Write a parser which builds a parsed syntax tree
3. Start with converting reaction equations to a matrix, rate-to-time-derivatives map, etc.
4. Parse mathematical expressions that define the rates. 

Useful links:

* https://craftinginterpreters.com/scanning.html
* https://tiarkrompf.github.io/notes/?/just-write-the-parser/aside10

## Reaction Net's Langauge. 