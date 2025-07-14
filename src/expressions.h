#pragma once

template<typename E>
struct Expr {
    static constexpr bool is_leaf = false; 

    
    double operator()() const {
        return static_cast<E const &>(*this)();
    }
};

struct Symbol : public Expr<Symbol> {
    static constexpr bool is_leaf = true;
    double value; 

    Symbol(double v) : value{v} {}
    
    double operator()() const { return value;}
};

template<typename L, typename R>
struct Add : public Expr<Add<L, R>> {
    typename std::conditional<L::is_leaf, const L&, const L>::type _left;
    typename std::conditional<R::is_leaf, const R&, const R>::type _right;

    Add(L const& x, R const& y) : _left{x}, _right{y} {}

    double operator()() const {
        return _left() + _right();
    }

};


template<typename L, typename R>
Add<L, R> operator+(Expr<L> const & x, Expr<R> const& y){
    return Add<L,R>(*static_cast<const L*>(&x), *static_cast<const R*>(&y));
}

template<typename L, typename R>
struct Mul : public Expr<Mul<L, R>> {
    typename std::conditional<L::is_leaf, const L&, const L>::type _left;
    typename std::conditional<R::is_leaf, const R&, const R>::type _right;

    Mul(L const& x, R const& y) : _left{x}, _right{y} {}
    
    double operator()() const {
        return _left() * _right();
    }
};

template<typename L, typename R>
Mul<L, R> operator*(Expr<L> const & x, Expr<R> const& y){
    return Mul<L,R>(*static_cast<const L*>(&x), *static_cast<const R*>(&y));
}

