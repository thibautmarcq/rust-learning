# Leçon 1 — Toolchain, Cargo, structure de projet & ownership de base

## 1. Installer et comprendre la toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
rustc --version
cargo --version
```

- `rustc` = le compilateur (équivalent `gcc`/`clang`)
- `cargo` = build system + gestionnaire de paquets (équivalent `make` + `cmake` + `conan`/`vcpkg` réunis)
- `rustup` = gestionnaire de toolchains (versions stable/beta/nightly, cibles de cross-compilation)

## 2. Créer un projet

```bash
cargo new mon_projet
cd mon_projet
```

Structure générée :

```
mon_projet/
├── Cargo.toml      # équivalent CMakeLists.txt / .vcxproj — métadonnées + dépendances
├── Cargo.lock       # équivalent lock file (versions exactes résolues), généré automatiquement
└── src/
    └── main.rs      # point d'entrée, équivalent main.cpp
```

`Cargo.toml` minimal :

```toml
[package]
name = "mon_projet"
version = "0.1.0"
edition = "2021"

[dependencies]
```

Commandes essentielles :

```bash
cargo check   # vérifie que ça compile SANS générer de binaire (rapide, à utiliser en boucle)
cargo build   # compile (debug par défaut)
cargo build --release   # compile avec optimisations (équivalent -O2/-O3)
cargo run     # build + exécute
cargo test    # lance les tests unitaires intégrés au langage
```

Différence clé avec C/C++ : **il n'y a qu'une seule façon standard** de builder/tester/gérer les dépendances dans tout l'écosystème Rust. Fini les Makefile artisanaux.

## 3. Un projet avec plusieurs fichiers

```
src/
├── main.rs
└── conversion.rs
```

`main.rs` :
```rust
mod conversion; // déclare le module, cherche conversion.rs ou conversion/mod.rs

fn main() {
    let c = conversion::celsius_vers_fahrenheit(20.0);
    println!("20°C = {c}°F");
}
```

`conversion.rs` :
```rust
pub fn celsius_vers_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
```

Contrairement à C++, pas besoin de `.h`/`.cpp` séparés ni de déclarations dupliquées : un module = un fichier, et `pub` remplace la logique "header expose / cpp implémente".

## 4. Ownership — le vrai changement de paradigme

En C++, vous gérez la durée de vie mentalement (RAII, `unique_ptr`, `shared_ptr`) mais le compilateur ne vous empêche pas d'utiliser un pointeur après `delete`. Rust **impose** ces règles à la compilation.

### Règle fondamentale
Chaque valeur a **un seul propriétaire**. Quand le propriétaire sort de portée, la valeur est libérée (comme un destructeur RAII automatique, mais vérifié statiquement).

```rust
fn main() {
    let s1 = String::from("bonjour");
    let s2 = s1; // s1 est "move" dans s2 : s1 n'est PLUS valide

    println!("{s2}");
    // println!("{s1}"); // ERREUR de compilation : value moved
}
```

C'est différent d'un `shared_ptr` (pas de comptage de référence ici) et différent d'une copie implicite C++ (pas de duplication silencieuse coûteuse).

### Types `Copy` vs types qui bougent (move)

Les types simples (entiers, `bool`, `char`, tuples de types `Copy`) implémentent `Copy` : ils sont dupliqués automatiquement, comme un `int` en C.

```rust
fn main() {
    let x = 5;
    let y = x; // copie, pas move : x reste valide
    println!("{x} {y}"); // OK
}
```

`String`, `Vec<T>`, et la plupart des types possédant de la mémoire heap ne sont **pas** `Copy` : ils sont déplacés (move) par défaut. Pour dupliquer explicitement :

```rust
let s1 = String::from("bonjour");
let s2 = s1.clone(); // copie explicite et coûteuse, visible dans le code
```

### Ownership et fonctions

```rust
fn consomme(s: String) {
    println!("{s}");
} // s est libéré ici

fn main() {
    let s = String::from("test");
    consomme(s);
    // println!("{s}"); // ERREUR : s a été move dans consomme()
}
```

C'est l'équivalent Rust de passer un `std::string` par valeur en C++ — sauf que le compilateur vous interdit formellement de continuer à utiliser l'original après le transfert, alors qu'en C++ ce serait juste une copie silencieuse (ou un move avec `std::move`, utilisable par erreur après coup sans erreur de compilation).

## 5. Ce qu'il faut retenir avant la Leçon 2

- Une valeur a un seul propriétaire à la fois
- Le déplacement (move) est la sémantique par défaut pour les types non-`Copy`
- `.clone()` rend les copies explicites et visibles
- Le compilateur refuse d'utiliser une valeur après qu'elle ait été move — pas de "use after move" possible, contrairement au "use after free" en C++

La Leçon 2 couvrira les **références** (`&T`), qui permettent d'utiliser une valeur sans en prendre possession — l'équivalent contrôlé des références C++.

---

## Mini-projet de la leçon 1 : Convertisseur d'unités en CLI

**Objectif :** manipuler ownership et move sur des `String`, sans utiliser encore de références.

### Cahier des charges
Un programme en ligne de commande qui :
1. Lit une valeur numérique et une unité tapées par l'utilisateur (via `std::io::stdin`), par exemple `"20 celsius"`
2. Convertit vers l'unité opposée (celsius ↔ fahrenheit, ou km ↔ miles, à votre choix)
3. Affiche le résultat
4. Boucle jusqu'à ce que l'utilisateur tape `"quit"`

### Contraintes pédagogiques (à respecter exprès)
- Organisez le code en **au moins 2 fichiers** : `main.rs` + un module `conversion.rs`
- La fonction de parsing de l'entrée doit **prendre possession** (`String`, pas `&str`) de la ligne lue, pour observer le move en pratique
- Utilisez `.clone()` au moins une fois là où c'est nécessaire, et commentez pourquoi c'était nécessaire à cet endroit précis
- Gérez le cas d'une entrée invalide sans faire planter le programme (`match` sur le résultat du `parse::<f64>()`)

### Indices techniques (sans donner la solution)
- `std::io::stdin().read_line(&mut buffer)` pour lire une ligne
- `str::trim()` pour enlever le `\n`
- `str::split_whitespace()` pour séparer nombre et unité
- `str::parse::<f64>()` retourne un `Result`

Quand vous aurez terminé, montrez-moi votre code : on fera une revue orientée ownership avant de passer à la Leçon 2.