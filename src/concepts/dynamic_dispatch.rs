trait Animal {
    fn make_sound(&self);
}
struct Dog;
struct Cat;
impl Animal for Dog {
    fn make_sound(&self) {
        println!("barking..");
    }
}
impl Animal for Cat {
    fn make_sound(&self) {
        println!("meaw..");
    }
}

pub fn dynamic() {
    // u want to create a vector contains animals
    //let animals: Vec<dyn Animal> = Vec::new();
}
