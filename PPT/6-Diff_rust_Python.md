# Différence de Rust par rapport à Python

Ce document vise à expliquer les principales différences entre Rust et Python, en mettant l'accent sur ce que les développeurs Python doivent savoir lorsqu'ils envisagent de se familiariser avec Rust.

## 1. Performance

Rust est un langage de programmation à la mémoire sécurisée qui offre des performances bien meilleures que Python, qui est interprété. Rust permet un contrôle précis sur la gestion de la mémoire et n'a pas de coûts d'overhead liés à l'interprétation. Si vous travaillez sur des applications nécessitant des performances élevées, Rust peut être un meilleur choix.

## 2. Sûreté

Rust est réputé pour sa sûreté. Il empêche les erreurs de mémoire courantes telles que les pointeurs nuls, les courses de données et les fuites de mémoire grâce à son système de propriété. Python n'offre pas le même niveau de sécurité en ce qui concerne la gestion de la mémoire.

## 3. Typage statique

Rust est un langage à typage statique, ce qui signifie que les types des variables sont vérifiés à la compilation. Python, en revanche, est à typage dynamique. Cela signifie que Rust peut détecter davantage d'erreurs à un stade précoce, tandis que Python peut rencontrer des erreurs de type à l'exécution.

## 4. Écosystème

Python a un écosystème très riche avec de nombreuses bibliothèques et frameworks pour diverses tâches. Rust a également un écosystème en croissance rapide, mais il peut ne pas offrir autant de choix que Python. Cependant, Rust peut facilement intégrer du code écrit dans d'autres langages, y compris Python, grâce à des API C étrangères.

## 5. Courbe d'apprentissage

Rust peut être plus difficile à apprendre pour les développeurs Python en raison de sa syntaxe et de son système de propriété. Cependant, une fois que vous avez acquis de l'expérience avec Rust, vous apprécierez ses avantages en matière de performance et de sécurité.

## 6. Cas d'utilisation

Rust est idéal pour les applications nécessitant une haute performance, une sécurité rigoureuse et un contrôle sur la mémoire. Python, en revanche, est plus adapté aux tâches de scripting, au développement web rapide et à l'analyse de données.

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
    x = 42 # Déclaration d'une variable
    y = 7

    print("x :", x)
    print("y :", y)

    # En Python, les variables sont par défaut mutables

if __name__ == "__main__":
    main()
```

En Rust, vous devez spécifier `mut` pour déclarer une variable comme mutable. En Python, les variables sont par défaut mutables.


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

**Python :**
```python
def main():
    s1 = "Hello" # Création d'une chaîne de caractères (str)
    s2 = s1 # s2 pointe vers la même chaîne que s1 (pas de transfert de propriété)

    print("s1 :", s1) # s1 et s2 partagent la même chaîne
    print("s2 :", s2)

if __name__ == "__main__":
    main()
```

En Rust, le concept d'ownership signifie qu'une variable détient la propriété de la valeur, et la propriété peut être transférée à une autre variable. En Python, les variables font souvent référence aux mêmes objets, et il n'y a pas de notion explicite de propriété. Python utilise la gestion automatique de la mémoire (garbage collector) pour gérer la durée de vie des objets.
