```rust,ignore
struct UserFactory<F1 = No, F2 = No> { /* private */ }

impl UserFactory<No, No> {
    pub fn new() -> Self;
}
impl Default for UserFactory<No, No> { /* delegates to new() */ }

// Setters: applicable while the matching flag is `No`. Consume `self`,
// flip the flag to `Yes`. The other flag is left as a free generic.
impl<F2> UserFactory<No, F2> {
    pub fn name(self, val: String) -> UserFactory<Yes, F2>;
}
impl<F1> UserFactory<F1, No> {
    pub fn  age(self, val: u32)    -> UserFactory<F1, Yes>;
}

// Getters: applicable while the matching flag is `Yes`.
impl<F2> UserFactory<Yes, F2> { pub fn name(&self) -> &String; }
impl<F1> UserFactory<F1, Yes> { pub fn  age(&self) -> &u32;    }

// Finalize: callable once every required flag is `Yes`.
impl UserFactory<Yes, Yes> {
    pub fn finalize(self) -> User;
}

// Companion "this bag is finalize-callable" trait. Lets downstream
// code write `where B: UserFactoryReady` instead of spelling out the
// flag tuple. See `./ready_trait.rs`.
trait UserFactoryReady: Sized {
    fn finalize(self) -> User;
}
impl<F1: Satisfied, F2: Satisfied> UserFactoryReady
    for UserFactory<F1, F2> { /* delegates to finalize() */ }
```
