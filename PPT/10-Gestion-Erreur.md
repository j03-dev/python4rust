### 5. Gestion des Erreurs

**Rust :**

```rust
fn main() {
    let num_str = "42";
    let num = num_str.parse::<i32>(); // La méthode parse peut échouer

    match num {
        Ok(value) => println!("Valeur parsée : {}", value),
        Err(e) => println!("Erreur de parsing : {:?}", e),
    }
}
```

**Python :**

```python
def main():
    num_str = "42"
    try:
        num = int(num_str)  # La conversion peut échouer
        print("Valeur convertie :", num)
    except ValueError as e:
        print("Erreur de conversion :", e)


if __name__ == "__main__":
    main()
```

En Rust, la gestion des erreurs est gérée à l'aide du type `Result` avec les variantes `Ok` et `Err`. Dans Python, vous
utilisez des blocs `try` et `except` pour gérer les exceptions.
