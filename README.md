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
2. Write a scanner which parses files into tokens
3. Write a parseer which builds a syntax tree from the sequence of tokens.
4. Transform or translate the syntax tree into other forms.
5. Compile into code (.eg., c/c++, python, r)
4. Start with converting reaction equations to a matrix, rate-to-time-derivatives map, etc.
4. Parse mathematical expressions that define the rates. 


Useful links:

* https://craftinginterpreters.com/
* https://tiarkrompf.github.io/notes/?/just-write-the-parser/aside10



## Reaction Net's Language. 

Reaction net's language has three contexts: names, formulas, and rate-kinetics expressions.  The idea is that a reaction can have a name, formula showing its stoichiometry, and a mathematical expression showing the reaction rate:

```
reaction_name_1 : reaction_formula_1 : reaction_rate_1; 
reaction_name_2 : reaction_formula_2 : reaction_rate_2; 
```

The reaction name and reaction rate expression are optional.

### Reaction formulas

A reaction formula has one of the following forms:

```
X -> Y
X <- Y
X <-> Y
X = Y 
```

The reaction arrows indicate the direction of the reaction. `<->` is a reversible reaction and `=` indicates a reversible reaction. Here `X` and `Y` are a vector with only positive coefficients, or more properly a *commutative monoid*. To spell out what that means, in general, `X` and `Y` have the form:
```
X = a1 * X1 + a2 * X2 + ...  + an * Xn  
```
where `a1, a2, ...` are stoichiometric coefficients (either positive integers or positive real numbers) and `X1` are chemical species or metabolites. 


In general, it would be nice if chemical formulas could be used: `NH4(+)`, `H2O`,  `1,1-difluoroethane` and `2-amino-4-carbamoylbutanoic acid`. 
* What to allow for symbols `X` ? 
* Is this valid `2-oxoglutarate`? 
* Is this valid `NH4(+)` ?  
* How should `2 * (X + Y) -> Z` be interpreted? 
The issues are how to parse the identifiers correctly if they permit chracters like `-`, `+`, `(`, and `)`. It would be nice to allow for substitutions: `X = 1,1-difluoroethane` in the file. 

Solutions:

1. Use `[` and `]` to i
ndicate any name: e.g. `[2-amino-4-carbamoylbutanoic acid]`. All keywords and reserved symbols would be allow inside `[` and `]` and spaces would not be ignored. 

#### Formal Grammar of Reaction Formulas

Syntax rules for formal grammars, especially context free grammars, can be described as a string rewriting rules. Using [Extended Backus Naur Form] as notation to write the syntax rules for our reaction formulas. Quoted strings are terminal symbols while unquoted strings are non-terminal symbols. I use `->` to indicate a production (or rewriting rule). For conciseness, the `|` symbol indicates "one of" or "or". Basically, `X -> A | B` means the symbol `X` can be replaced with `A` or `B`. This is equivalent to multiple production rules `X -> A` and `X -> B`. Likewise `["*"]` indicates that the symbol `*` is optional.  

Here's the basic syntax rules. 
```
reaction -> (complex yield complex) | reaction ";" reaction
yield -> "->" | "<-" | "<->" | "="
complex -> complex "+" complex
complex ->  number ["*"] complex
complex -> "(" complex ")"
complex -> species
number -> ? any integer ?
species -> ? any alphanumeric identifier ? 
```

The first two give us all possible reaction equations, but does not allow chaining. I.e., `A -> B -> C` would be invalid. A reaction has two "complexes" (this is the word used in mathematical chemistry). The product/reactant complex is always an element of a free commutative monoid over some species symbols, where we write the monoid operation as `+`. That's a fancy way of saying that you can add any species symbol to any other as many times as you like, and the "addition" is commutative (so like regular addition). `X+Y` and `Y + X` are equivalent ways of writing the same thing.  We want to allow people to use integers to abbreviate `X + X + Y` into ` `2 X + Y`  or `2 * X + Y`

We might want to make the complexes into elements of a free vector space so that we have arbitrary symbols for stoichiometric coefficients. That way, you could write `a * X + Y` and specify the stoichiometric coefficient later. But let's put a pin in that.



### Reaction Rate Expressions

Mostly should be mathematical notation as in any programming language.

