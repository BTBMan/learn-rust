use macro_procedural_derive::MacroProcedural;

fn main() {
    trait MacroProcedural {
        fn greeting();
    }

    #[derive(MacroProcedural)]
    struct User;

    User::greeting();
}
