### 6. `struct` en Rust vs `class` en Python

En Rust, une `struct` (structure) est un type de données qui vous permet de regrouper différentes valeurs sous un nom de structure. Elle est similaire à une classe dans d'autres langages de programmation, mais avec des différences importantes. Voici une comparaison entre les deux :

**Rust (`struct`) :**
```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Nom : {}", person1.name);
    println!("Âge : {}", person1.age);
}
```

**Python (`class`) :**
```python
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age

person1 = Person("Alice", 30)
print("Nom :", person1.name)
print("Âge :", person1.age)
```

Les `struct` en Rust et les `class` dans d'autres langages (comme Python) permettent d'encapsuler des données, mais en Rust, il n'y a pas de méthodes attachées par défaut à la `struct`, et la notion d'héritage n'existe pas. Rust utilise des traits (traits) pour ajouter des fonctionnalités à une `struct`.

### Traits (trait) en Rust

En Rust, un trait est un mécanisme permettant de définir un ensemble de méthodes (fonctions) que les types peuvent implémenter. Les traits permettent de partager du code entre différents types de données sans avoir besoin d'une hiérarchie d'héritage.

**Rust (Trait) :**
```rust
trait Printable {
    fn print(&self);
}

struct Person {
    name: String,
    age: u32,
}

impl Printable for Person {
    fn print(&self) {
        println!("Nom : {}", self.name);
        println!("Âge : {}", self.age);
    }
}

fn main() {
    let person1 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    person1.print();
}
```

En Rust, on définit un trait (`trait`) qui déclare la méthode `print`, puis on implémente ce trait pour le type `Person`. Ainsi, `Person` peut utiliser la méthode `print`.

Les traits en Rust permettent une implémentation flexible de comportements spécifiques pour différents types de données sans avoir besoin de l'héritage de classe.

En résumé, en Rust, les `struct` sont utilisées pour encapsuler des données, mais les méthodes sont gérées à travers l'implémentation de traits, ce qui rend le système de types de Rust plus flexible que les `class` d'autres langages avec l'héritage.
