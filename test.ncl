type Person = {
    name: String,
    age: Int,
    friends: List(String)
};

let alice = {
    name: "Alice",
    age: 30,
    friends: ["Bob", "Charlie"]
};

let bob = {
    name: "Bob",
    age: 25,
    friends: ["Alice"]
};

let greet = (person: Person) => "Hello, " ++ person.name ++ "!";

let is_adult = (age: Int) => age >= 18;

let number_of_friends = (person: Person) => length(person.friends);

let filter_adults = (people: List(Person)) => filter((p) => is_adult(p.age), people);

let greeting_message = greet(alice);
let adult_check = is_adult(alice.age);
let friends_count = number_of_friends(alice);
let people = [alice, bob];
let adults = filter_adults(people);

greeting_message, adult_check, friends_count, adults
