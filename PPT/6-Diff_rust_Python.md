# Différence de Rust par rapport à Python

## 1. Performance

Rust est un langage de programmation à la mémoire sécurisée qui offre des performances bien meilleures que Python, qui
est interprété. Rust permet un contrôle précis sur la gestion de la mémoire et n'a pas de coûts d'overhead liés à
l'interprétation. Si vous travaillez sur des applications nécessitant des performances élevées, Rust peut être un
meilleur choix.

## 2. Sûreté

Rust est réputé pour sa sûreté. Il empêche les erreurs de mémoire courantes telles que les pointeurs nuls, les courses
de données et les fuites de mémoire grâce à son système de propriété. Python n'offre pas le même niveau de sécurité en
ce qui concerne la gestion de la mémoire.

## 3. Typage statique

Rust est un langage à typage statique, ce qui signifie que les types des variables sont vérifiés à la compilation.
Python, en revanche, est à typage dynamique. Cela signifie que Rust peut détecter davantage d'erreurs à un stade
précoce, tandis que Python peut rencontrer des erreurs de type à l'exécution.

## 4. Écosystème

Python a un écosystème très riche avec de nombreuses bibliothèques et frameworks pour diverses tâches. Rust a également
un écosystème en croissance rapide, mais il peut ne pas offrir autant de choix que Python. Cependant, Rust peut
facilement intégrer du code écrit dans d'autres langages, y compris Python, grâce à des API C étrangères.

## 5. Courbe d'apprentissage

Rust peut être plus difficile à apprendre pour les développeurs Python en raison de sa syntaxe et de son système de
propriété. Cependant, une fois que vous avez acquis de l'expérience avec Rust, vous apprécierez ses avantages en matière
de performance et de sécurité.

## 6. Cas d'utilisation

Rust est idéal pour les applications nécessitant une haute performance, une sécurité rigoureuse et un contrôle sur la
mémoire. Python, en revanche, est plus adapté aux tâches de scripting, au développement web rapide et à l'analyse de
données.

### 1. Affichage de "Hello, World!"

**Rust :**

```rust
fn main() {
    println!("Hello, World!");
}
```

**Python :**

```python
def main():
    print("Hello, World!")


if __name__ == "__main__":
    main()
```

L'affichage de "Hello, World!" est similaire dans les deux langages.

### 2. Déclaration de variables

**Rust :**

```rust
fn main() {
    let x = 42; // Déclaration d'une variable immuable (par défaut)
    let mut y = 7; // Déclaration d'une variable mutable avec 'mut'

    println!("x : {}", x);
    println!("y : {}", y);

    y = 14; // Modification de la variable mutable y
    println!("y (modifié) : {}", y);
}
```

**Python :**

```python
def main():
    x = 42  # Déclaration d'une variable
    y = 7

    print("x :", x)
    print("y :", y)

    # En Python, les variables sont par défaut mutables


if __name__ == "__main__":
    main()
```

En Rust, vous devez spécifier `mut` pour déclarer une variable comme mutable. En Python, les variables sont par défaut
mutables.

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

Bien sûr, parlons des boucles (for et while), de la gestion des erreurs et de la manipulation de ces concepts en Rust et
Python.

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

### 6. `struct` en Rust vs `class` en d'autres langages

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

**Autres langages (`class`) :**
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