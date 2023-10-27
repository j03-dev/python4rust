## _Différence de Rust avec Python_

### Exemple 1 : Gestion de la mémoire

**Rust :**
```rust
fn main() {
    let x = 5;  // Variable avec une valeur entière
    let y = &x; // Référence à la variable x

    println!("x : {}", x);
    println!("y : {}", y);
}
```

**Python :**
```python
def main():
    x = 5  # Variable avec une valeur entière
    y = x   # y fait une copie de la valeur de x

    print("x :", x)
    print("y :", y)

if __name__ == "__main__":
    main()
```

En Rust, la variable `y` est une référence à la variable `x`. En Python, `y` contient une copie de la valeur de `x`. Cela montre comment Rust permet un contrôle plus précis sur la gestion de la mémoire.

### Exemple 2 : Typage statique

**Rust :**
```rust
fn main() {
    let x: i32 = 5;
    let y: f64 = 5.0;

    let sum = x + y; // Erreur de compilation : types incompatibles
}
```

**Python :**
```python
def main():
    x = 5
    y = 5.0

    sum = x + y  # Aucune erreur à la compilation, mais une erreur de type pourrait se produire à l'exécution

if __name__ == "__main__":
    main()
```

En Rust, le mélange de types incompatibles (entier et flottant) génère une erreur de compilation. En Python, le mélange de types est autorisé, mais cela pourrait entraîner des erreurs à l'exécution.

Ces exemples mettent en évidence les différences dans la gestion de la mémoire et le typage statique entre Rust et Python. Rust offre une plus grande sécurité et des erreurs détectées à la compilation, tandis que Python est plus permissif mais peut avoir des erreurs de type à l'exécution.

### [< Précédent](./5-build_in.md)
