### 3. Propriété (Ownership) et Emprunt (Borrowing)

**Rust :**

```rust
fn main() {
    let s1 = String::from("Hello"); // Création d'une chaîne de caractères
    let s2 = s1; // La propriété de la chaîne s1 est transférée à s2 (ownership)

    // La ligne suivante générerait une erreur car s1 ne peut plus être utilisé
    // println!("s1 : {}", s1); 

    println!("s2 : {}", s2); // s2 détient maintenant la propriété de la chaîne
}
```

En Rust, le concept d'ownership signifie qu'une variable détient la propriété de la valeur, et la propriété peut être
transférée à une autre variable. En Python, les variables font souvent référence aux mêmes objets, et il n'y a pas de
notion explicite de propriété. Python utilise la gestion automatique de la mémoire (garbage collector) pour gérer la
durée de vie des objets.

**Python :**

```python
def main():
    s1 = "Hello"  # Création d'une chaîne de caractères (str)
    s2 = s1  # s2 pointe vers la même chaîne que s1 (pas de transfert de propriété)

    print("s1 :", s1)  # s1 et s2 partagent la même chaîne
    print("s2 :", s2)


if __name__ == "__main__":
    main()
```
