### 4. Boucles (for et while)

**Rust :**

```rust
fn main() {
    // Boucle for
    for i in 1..5 {
        println!("Valeur de i : {}", i);
    }

    // Boucle while
    let mut j = 1;
    while j <= 5 {
        println!("Valeur de j : {}", j);
        j += 1;
    }
}
```

**Python :**

```python
def main():
    # Boucle for
    for i in range(1, 5):
        print("Valeur de i :", i)

    # Boucle while
    j = 1
    while j <= 5:
        print("Valeur de j :", j)
        j += 1


if __name__ == "__main__":
    main()
```

Les boucles for et while sont similaires en Rust et Python, bien que la syntaxe puisse varier légèrement. Dans les deux
langages, vous pouvez utiliser ces boucles pour itérer sur des séquences de valeurs.
