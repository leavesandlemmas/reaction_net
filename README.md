# reaction_net

**WORK IN PROGRESS**

## Overivew 

`reaction_net` is a parser and compiler designed for mathematical models of chemical reaction networks found in biology and chemistry.

### The Problem...

The purpose of `reaction_net` is best illustrated using an example. Consider the following system of chemical reactions:

```
A + B -> C
C -> D <-> E
```
Here, the symbols A, B, C, and so on simply stand in for any chemical species. If each of these reactions has mass-action kinetics, then the time-evolution of each of the chemical species satisfies the following system of differential equations:

$$
    \begin{align} 
        \frac{d}{dt}[A] &= - r_1 & &=  - k_1 [A] [B]\\   
        \frac{d}{dt}[B] &= - r_1 & &=  - k_1 [A] [B]\\ 
        \frac{d}{dt}[C] &=   r_1 - r_2 & &=  k_1 [A] [B] - k_2 [C]\\ 
        \frac{d}{dt}[D] &=   r_2 - r_3 & &=  k_2 [C] - k_3 [D] + k_4 [E]\\
        \frac{d}{dt}[E] &=   r_3 & &=  k_3 [D] - k_4 [E]\\
    \end{align}
$$

In general, a system of chemical reactions corresponds to a dynamical system, given a choice of kinetics for each reaction. The behavior of large systems of chemical reactions can be simulated by solving this system of differential equations. For example, photosynthesis in plants involves a several pathways of chemical reactions such as the light reactions, the dark reactions (Calvin-Benson cycle), and photorespiration reactions. The overall rate of photosynthesis depends on the overall behavior of all of these reactions and their associated intermediate metabolites. Numerical simulations and other mathematical analyses provide the means to analyze how changing a particular part of this metabolic pathway affects the overall rate of photosynthesis, with the ultimate goal of bioengineering plants with better forms of photosynthesis.  

To perform numerical computations with the system of chemical reactions or the system of differential equations, the system must be translated into code for numerical integration. For example, here is a translation into python (using `numpy`):

```py
import numpy 
def vec(x : numpy.ndarray, k : numpy.ndarry) -> numpy.ndarray:
    v = numpy.empty(5, dtype = x.dtype)
    r1 = k[0] * x[0] * x[1]         # A + B -> C
    r2 = k[1] * x[2]                # C -> D
    r3 = k[2] * x[3] - k[3] * x[4]  # D <-> E
    v[0] = - r1
    v[1] =   r1
    v[2] =   r1 - r2
    v[3] =   r2 - r3
    v[4] =   r3
    return v
```

And here's a translation into R.

```R
vec <- function(x, k){
    v <- numeric(5) # preallocate
    r1 <- k[1] * x[1] * x[2]         # A + B -> C
    r2 <- k[2] * x[3]                # C -> D
    r3 <- k[3] * x[4] - k[4] * x[5]  # D <-> E
    v[1] <- - r1
    v[2] <-   r1
    v[3] <-   r1 - r2
    v[4] <-   r2 - r3
    v[5] <-   r3
    return(v) 
}
```

While this translation process is most mechanical, it must repeated every time the model's definition changes. If a reaction is added or removed, or if a reactant is added or removed, then the code must be updated. While this isn't hard for a small number of reactions, metabolic pathways often involved tens to hundreds of reactions and as many metabolites. 

Moreover, the reaction kinetics are often unknown for many metabolic model as they are often approximations to enzyme kinetics, which are not elementary reactions, but involve many steps. The exact kinetics are often unknown or approximate so investigating how the system changes when the reaction kinetics change is potentially useful. For instance, suppose we annotate the reaction `C -> D` with a kinetics formula:

```
A + B -> C
C -> D : V * C / (Km + C)
D <-> E 
```

Any translation would have to be updated and two new parameters would have to be added to the list of all parameters. Again when the model is small as the above example, the translation step is easy enough to do by hand. However, as the number of variables grows, the translation step becomes more difficult, hindering changes. Furthermore, errors in translation can slip into the translations meaning that the code does not necessarily represent the same system of reactions. Such bugs are incredibly hard to track down. 

Given a translation of the system of reactions into code, a large number of numerical computations become available. For instance, the concentrations of each chemical species may be simulated so that the resulting curve may be fit to data. These types of numerical algorithms are often easy to apply to generic functions, like the translations above, and are applied to many different models. Thus, any translation of the system of reactions into code must take into account all of the algorithms that might be built on top of it. Or the translation is constrained to have a certain format required by the desired algorithm. Both requirements make translation by hand even more complex.  

### `reaction_net`'s Solution

In programming, there exists an aphorism "keep code and data separate" meaning that a program is an algorithm that performs some computation on its input (the data). Ideally the program shouldn't have to change when the input changes. Here, the mathematical model is really data, and not code. The model itself should be the input to a program. The program should take a data structure which defines the model, then perform the desired computations.  The goal of `reaction_net` is to offer a solution of this kind.  

`reaction_net` seeks to solve this problem by performing the translation mechanically. Thus, a system of chemical reactions, with annotations for their reaction kinetics, can be written in a domain-specific language that mirrors the notation of chemical reactions used in chemistry (as shown above). A model's definition can be written in a human-readable text. Given this definition file, `reaction_net` parses and translates the model into code that can be used simulations or for other kinds of analyses. 

However, `reaction_net` is not the only solution to this problem. For instance, the [Systems Biology Markup Language](https://sbml.org/) (SBML) is a markup language for describing similar models. The idea of that project is to create a data file format, similar to HTML, that describes mathematical models in systems biology, which include systems of chemical reactions. Other programs provide GUIs for model construction and simulation. 

`reaction_net` differs from many other projects in the following respects. First, the domain-specific language is designed to be human-readable. SBML files are designed to be easily parsed by a computer, but SBML files require a programs to create or edit SBML files. Second, `reaction_net` produces a translation of the model into code, which can standalone. In essence, `reaction_net` is a compiler; the model's definition is the input code and the translation is output.  

## Reaction Net's Language. 

**WORK IN PROGRESS**

Some of the description is aspirational and not yet realized. This overview was written to plan features, not just describe existing ones.

### Overview 

In a nut shell, the language is a list of chemical reactions written like `A + B -> C`. By writing `A + B -> C`, we declare three chemical species and one reaction. Here's an example using the Michaelis-Menten kinetics system.
 
```
// michaelis-menten kinetics
S + E <-> ES
ES -> E + P 
```
The symbol `<->` indicates a reversible reaction. The double forward slash `//` indicates a comment that will be ignored by `reaction_net`. This allows the model's author to add useful information for other humans. We don't have to use single character symbols; this is equivalent:


```
// michaelis-menten kinetics
substrate + enzyme <-> "enzyme-substrate"
"enzyme-substrate" -> product + enzyme
```

Currently, we have to use quotes `"` around `enzyme-substrate` to tell `reaction_net` to ignore `-` and treat the whole string as a single unit. Alphanumeric identifiers do not require quotes however. We can annotate reactions with a name by putting `:` in front of the reaction formula.

```
// michaelis-menten kinetics
binding   : substrate + enzyme <-> "enzyme-substrate"
catalysis : "enzyme-substrate" -> product + enzyme
```

We can also annotate the kinetics using a colon `:` after the reaction formula, then a mathematical expression which evaluates to the reaction rate:

```
// michaelis-menten kinetics
substrate + enzyme <-> "enzyme-substrate" : k_forward * substrate * enzyme - k_reverse * "enzyme-substrate" 
"enzyme-substrate" -> product + enzyme    : k_cat * "enzyme-substrate" 
```

Arithmetic operations in expressions are denoted in the same way as many programming languages. Writing any identifier in the reaction kinetics expression declares its existence. So writing `k_cat` means that we have a rate-coefficient called `k_cat`  

By default, reactions without kinetics are given the appropriate mass-action kinetics expression; and the associated rate-constants are automatically declared. For anonymous (unnamed) reactions with the default kinetics, we can chain reactions `A -> B -> C` to make simple networks easy to write. 

Multiple reactions are allowed, if multiple reactions have the same reactants and products. 

```
slow              : A -> B : slow_k * A 
fast              : A -> B : fast_k * A
enzyme_catalyzed  : A -> B : Vmax * A / (Km + A)
product_inhibited : A -> B : Vmax * A / (A + Km * (1 + B / Ki))
```




### 

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

1. Use `[` and `]` to indicate any name: e.g. `[2-amino-4-carbamoylbutanoic acid]`. All keywords and reserved symbols would be allow inside `[` and `]` and spaces would not be ignored. 
2. Use `"` to indicate any identifier.
3. `let` block before reaction declarations.
4. `where` block after reaction declarations
```
let X = "1,1-difluoroethane" 
    Y = "2-amino-4-carbamoylbutanoic acid"
decay reaction : X -> Y : r
    where r = k1 * X

A + C1 -> B + C2
C2 -> C3 -> C1 
assume mass-action

```

#### Formal Grammar of Reaction Formulas

Syntax rules for formal grammars, especially context free grammars, can be described as a string rewriting rules. Using [Extended Backus Naur Form] as notation to write the syntax rules for our reaction formulas. Quoted strings are terminal symbols while unquoted strings are non-terminal symbols. I use `->` to indicate a production (or rewriting rule). For conciseness, the `|` symbol indicates "one of" or "or". Basically, `X -> A | B` means the symbol `X` can be replaced with `A` or `B`. This is equivalent to multiple production rules `X -> A` and `X -> B`. Likewise `["*"]` indicates that the symbol `*` is optional.  

Here's the basic syntax rules. 
```ebnf
<eof> ::= <reactionstmt> | <reactionstmt> ";" <eof> | <reactionstmt> "\n" <eof>
<reactionstmt> ::= (<symbol> ":")? <reaction> (":" <kinetics>)?  | <reactionpath>
<reactionpath> ::=  <complex> <yield> <reactionpath> | <reaction>
<reaction> ::= <complex> <yield> <complex>
<yield> ::= "->" | "<-" | "<->" | "="
<complex> ::=  <monomial> | <monomial> "+" <complex>
<monomial> ::= <symbol> | <number> "*"? <symbol> | <number>? "*"? "(" <complex> ")"
<symbol> ::= [A-Z]
<number> ::= [1-9]
<kinetics> ::= "mass-action" | "michaelis-menten" | <expr>
<expr> ::= <term> "+" <expr> | <term> "-" <expr> | <term>
<term> ::= <factor> "*" <term> | <factor> "/" <term> | <factor>
<factor> ::= <symbol> | <number> | "(" <expr> ")"
```

The first three give us all possible reaction equations as a list, but does not allow chaining. I.e., `A -> B -> C` would be invalid. A reaction has two "complexes" (this is the word used in mathematical chemistry). The product/reactant complex is always an element of a free commutative monoid over some species symbols, where we write the monoid operation as `+`. That's a fancy way of saying that you can add any species symbol to any other as many times as you like, and the "addition" is commutative (so like regular addition). `X+Y` and `Y + X` are equivalent ways of writing the same thing.  We want to allow people to use integers to abbreviate `X + X + Y` into ` `2 X + Y`  or `2 * X + Y`

We might want to make the complexes into elements of a free vector space so that we have arbitrary symbols for stoichiometric coefficients. That way, you could write `a * X + Y` and specify the stoichiometric coefficient later. But let's put a pin in that.



### Reaction Rate Expressions

Mostly should be mathematical notation as in any programming language.

