
const THRESHOLD: u32 = 10; // cant be changed
static LANG: &str = "EN"; // mutable

fn main() {
    let pre_declared_binding;

    let an_int = 1u32;
    let a_bool = true;
    let unit = ();

    let copied_int = an_int;

    println!("An integer: {:?}", copied_int);
    println!("A bool: {:?}", a_bool);
    println!("Meet the unit value: {:?}", unit);

    let _unused_var = 3u32;
    let noisy = 2u32; // We can prefix with _ to supress

    // Vars are immutable by default, can use mut modifier to allow editing

    let _immutable_bind = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // _immutable_binding += 1; - Throws error since it's immutable

    let shadowed_binding = 1;
    {
        let short_lived = 2;
        println!("We can create small scopes for variables too! {}", short_lived);

        let shadowed_binding = 3;
        println!("Shadowed: {}", shadowed_binding);
    }

    println!("Not shadowed {}", shadowed_binding);

    let shadowed_binding = 8;

    println!("Shadowed: {}", shadowed_binding);

    pre_declared_binding = 3;

    println!("We declared our binding at the top, and initialized it down here, make sure to always init before use: {}", pre_declared_binding);


    // FREEZING

    let mut _mutable_int = 7i32;

    {
        let _mutable_int = _mutable_int;
        // _mutable_int = 50; - _mutable_int became frozen because it's no longer mut in the shadowed context, will throw an error
    }

    _mutable_int = 80; // OKAY - no longer frozen
}
