class Person:
    def __init__(self, name: str, prenom: str, age: int):
        self.name = name
        self.prenom = prenom
        self.age = age

    def grow(self):
        self.age += 1


person = Person("FITAHIANA", "Nomeniavo Joe", 22)
print(person.__dict__)

person.grow()

print(person.__dict__)
